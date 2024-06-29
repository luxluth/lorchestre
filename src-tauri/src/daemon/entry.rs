use crate::daemon::config;
use crate::daemon::config::Dir;
use crate::daemon::global::Media;
use crate::daemon::utils;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header::CACHE_CONTROL, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use axum_extra::{extract::OptionalQuery, headers::Range, TypedHeader};
use axum_range::{KnownSize, Ranged};
use image::io::Reader as ImageReader;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::io::{BufWriter, Cursor, Read};
use std::sync::Arc;
use tokio::fs::File;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

#[derive(Debug, Clone)]
struct AppData {
    media: Arc<RwLock<Media>>,
    dirs: Dir,
    io: SocketIo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SearchQuery {
    q: String,
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        "search",
        |sock: SocketRef,
         Data::<String>(q),
         media: socketioxide::extract::State<Arc<RwLock<Media>>>| async move {
            let m = media.read().await;
            let res = m.search(&q);
            let _ = sock.emit("searchresponse", res);
        },
    )
}

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut host = "localhost".to_string();
    let mut port: u32 = 7700;

    let dirs = config::get_dirs();
    let config_path = dirs.config.join("config.toml");
    let config = lorconf::Config::get(&config_path);
    if let Some(network) = config.network {
        if let Some(p) = network.port {
            port = p;
        }

        if let Some(h) = network.host {
            host = h;
        }
    }

    let req_client = reqwest::Client::new();
    let response = req_client.get(format!("http://{host}:{port}")).send().await;
    if let Ok(_) = response {
        tracing::error!("Daemon already running");
        return Ok(());
    } else {
        drop(req_client);
        drop(response);
    }

    let m = utils::cache_resolve(&dirs.cache).await;
    let media_data = Arc::new(RwLock::new(m));

    let (layer, io) = SocketIo::builder()
        .with_state(Arc::clone(&media_data))
        .build_layer();
    io.ns("/", on_connect);

    let app = Router::new()
        .route("/", get(ping))
        .route("/media", get(media))
        .route("/audio", get(audio))
        .route("/album/:id", get(album))
        .route("/cover/:handle", get(cover))
        .route("/updatemusic", put(updatemusic))
        .with_state(AppData {
            media: media_data,
            dirs: dirs.clone(),
            io,
        })
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    info!("lorchestre daemon started on http://{host}:{port}");
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct ImageSize {
    size: String,
}

#[derive(serde::Deserialize, Debug)]
struct MusicPath {
    path: String,
}

impl ImageSize {
    pub fn parse(self) -> Option<(u32, u32)> {
        self.size
            .split_once('x')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
    }
}

async fn cover(
    State(state): State<AppData>,
    Path(handle): Path<String>,
    OptionalQuery(size): OptionalQuery<ImageSize>,
) -> Response {
    let path = state.dirs.cache.join("covers").join(handle);

    if let Some(image_size) = size {
        if let Some((w, h)) = image_size.parse() {
            if let Ok(mut image) = ImageReader::open(&path).unwrap().decode() {
                image = image.resize(w, h, image::imageops::FilterType::Gaussian);

                let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
                let _ = image.write_to(&mut buffer, image::ImageFormat::Png);

                let bytes = buffer.into_inner().unwrap().into_inner();

                let body = Body::from(bytes);
                let resp = Response::new(body);
                return resp;
            }
        }
    }

    if let Ok(mut file) = std::fs::File::open(&path) {
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf);

        let body = Body::from(buf);
        let resp = Response::new(body);
        return resp;
    } else {
        warn!("Fail to retrieve the cover file `{}`", path.display());
        let buf = include_bytes!("./assets/default-cover.png");
        let body = Body::from(buf.as_slice());
        let mut resp = Response::new(body);

        resp.headers_mut().insert(
            CACHE_CONTROL,
            HeaderValue::from_static("public, max-age=2419200, immutable"),
        );

        resp
    }
}

async fn updatemusic(State(state): State<AppData>) {
    let m = utils::cache_resolve(&state.dirs.cache).await;
    let mut binding = state.media.write().await;
    binding.swap_with(m.clone());
    let _ = state.io.emit("newmedia", m);
}

async fn album(State(state): State<AppData>, Path(id): Path<String>) -> Response {
    if let Some(album) = state.media.read().await.get_album(&id) {
        Json(album).into_response()
    } else {
        let mut response = format!("no album found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn audio(
    range: Option<TypedHeader<Range>>,
    State(state): State<AppData>,
    Query(music_path): Query<MusicPath>,
) -> Response {
    let path = music_path.path;
    if let Some(track) = state.media.read().await.get_song(&path) {
        let file = File::open(&track.file_path).await.unwrap();
        let body = KnownSize::file(file).await.unwrap();
        let r = range.clone().map(|TypedHeader(range)| range);
        let response = Ranged::new(r, body).try_respond();
        if let Ok(response) = response {
            return response.into_response();
        } else {
            let mut response =
                format!("An error occured while satisfying the request for path `{path}`")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        warn!("{path} not founded");
        let mut response = format!("no song found with the id of {path}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn ping() -> String {
    format!("OK lorchestrectl v{}", config::VERSION)
}

async fn media(State(state): State<AppData>) -> Json<Media> {
    Json(state.media.read().await.clone())
}
