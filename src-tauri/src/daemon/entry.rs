use super::{
    config::{self, Dir},
    global::{Media, Track},
    utils,
};
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
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
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

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LRCLIB: &str = "https://lrclib.net/api";

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

pub async fn start(win: Option<tauri::Window>) -> Result<(), Box<dyn std::error::Error>> {
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
    if response.is_ok() {
        tracing::error!("Daemon already running");
        return Err("Daemon already running".into());
    } else {
        drop(req_client);
        drop(response);
    }

    let m = utils::cache_resolve(&dirs.cache, win).await;
    let media_data = Arc::new(RwLock::new(m));

    let (layer, io) = SocketIo::builder()
        .with_state(Arc::clone(&media_data))
        .build_layer();
    io.ns("/", on_connect);

    let app = Router::new()
        .route("/", get(ping))
        .route("/media", get(media))
        .route("/audio", get(audio))
        .route("/lyrics", get(lyrics))
        .route("/album/:id", get(album))
        .route("/cover/:handle", get(cover))
        .route("/updatemusic", put(updatemusic))
        .route("/search/lyrics", get(search_lyrics))
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

#[derive(serde::Serialize)]
pub struct Lyrics {
    lyrics: Vec<alrc::Line>,
}

async fn lyrics(State(state): State<AppData>, Query(music_path): Query<MusicPath>) -> Json<Lyrics> {
    let path = String::from_utf8_lossy(&URL_SAFE.decode(music_path.path).unwrap()).to_string();

    if let Some(track) = state.media.read().await.get_song(&path) {
        Json(Lyrics {
            lyrics: track.get_lyrics(),
        })
    } else {
        Json(Lyrics { lyrics: vec![] })
    }
}

#[allow(non_snake_case)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct LyricsSearchResponse {
    id: usize,
    trackName: String,
    artistName: String,
    albumName: String,
    duration: f64,
    instrumental: bool,
    plainLyrics: Option<String>,
    syncedLyrics: Option<String>,
}

#[derive(serde::Serialize)]
struct Lrc {
    parsed: Vec<alrc::Line>,
    raw: String,
}

#[derive(serde::Serialize)]
struct LyricsResponse {
    lyrics: Vec<Lrc>,
}

async fn search_lyrics(
    State(state): State<AppData>,
    Query(music_path): Query<MusicPath>,
) -> Result<Json<LyricsResponse>, String> {
    let path = String::from_utf8_lossy(&URL_SAFE.decode(music_path.path).unwrap()).to_string();

    if let Some(track) = state.media.read().await.get_song(&path) {
        let client = reqwest::Client::new();
        match client
            .get(format!(
                "{LRCLIB}/search?track_name={}&artist_name={}",
                track.title, track.artists[0]
            ))
            .header(
                "User-Agent",
                format!("L'orchestre v{VERSION} (https://github.com/luxluth/lorchestre)"),
            )
            .send()
            .await
        {
            Ok(e) => {
                let mut lyrics = vec![];
                for lyric in e.json::<Vec<LyricsSearchResponse>>().await.unwrap() {
                    if let Some(synched) = lyric.syncedLyrics {
                        lyrics.push(Lrc {
                            parsed: Track::parse_lyrics(&synched).lines,
                            raw: synched,
                        });
                    }
                }

                Ok(Json(LyricsResponse { lyrics }))
            }
            Err(e) => Err(format!("{e:?}")),
        }
    } else {
        Err(String::from("Track not found"))
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
            if let Ok(reader) = ImageReader::open(&path) {
                if let Ok(mut image) = reader.decode() {
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
    }

    if let Ok(mut file) = std::fs::File::open(&path) {
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf);

        let body = Body::from(buf);
        Response::new(body)
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
    let m = utils::cache_resolve(&state.dirs.cache, None).await;
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
    let path = String::from_utf8_lossy(&URL_SAFE.decode(music_path.path).unwrap()).to_string();
    if let Some(track) = state.media.read().await.get_song(&path) {
        let file = File::open(&track.file_path).await.unwrap();
        let body = KnownSize::file(file).await.unwrap();
        let r = range.clone().map(|TypedHeader(range)| range);
        let response = Ranged::new(r, body).try_respond();
        if let Ok(response) = response {
            response.into_response()
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
