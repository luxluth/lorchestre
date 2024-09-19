use super::{
    config::{self, Dir},
    global::{self, Color, Media, SearchResults, Track},
    m3u8::{PlaylistAction, PlaylistData},
    utils,
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header::CACHE_CONTROL, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use axum_extra::{extract::OptionalQuery, headers::Range, TypedHeader};
use axum_range::{KnownSize, Ranged};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use image::ImageReader;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use std::{collections::HashMap, sync::Arc};
use std::{
    io::{BufWriter, Cursor, Read},
    path::PathBuf,
};
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
         data: socketioxide::extract::State<(Arc<RwLock<Media>>, Dir)>| async move {
            let (media, dirs) = data.0;
            let m = media.read().await;
            let res = m.search(dirs.cache, &q);
            let _ = sock.emit("searchresponse", res);
        },
    );

    socket.on(
        "localsearch",
        |sock: SocketRef,
         Data::<String>(q),
         data: socketioxide::extract::State<(Arc<RwLock<Media>>, Dir)>| async move {
            let (media, dirs) = data.0;
            let m = media.read().await;
            let res = m.search(dirs.cache, &q);
            let _ = sock.emit("localsr", res);
        },
    )
}

pub async fn start(
    win: Option<tauri::Window>,
    dirs: Dir,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut host = "localhost".to_string();
    let mut port: u32 = 7700;
    dirs.check();

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
        .with_state((Arc::clone(&media_data), dirs.clone()))
        .build_layer();
    io.ns("/", on_connect);

    let app = Router::new()
        .route("/", get(ping))
        // TODO: No cache
        .route("/media", get(media))
        .route("/localsearch", get(search))
        .route("/audio", get(audio))
        .route("/lyrics", get(lyrics))
        .route("/album/:id", get(album))
        // TODO: Do not cache this at all
        .route("/playlist/:id", get(playlist))
        // ------ palylist action
        .route("/playlist/:id", delete(list_remove))
        .route("/playlist/remove_track/:id", delete(list_remove_track))
        .route("/playlist/add_track/:id", post(list_add_track))
        .route("/playlist/update_order/:id", put(list_reorder))
        .route("/playlist/remove_meta/:id", delete(list_remove_meta))
        .route("/playlist/add_meta/:id", post(list_add_meta))
        .route("/playlist/create", post(list_create))
        // ------ palylist action
        .route("/cover/:handle", get(cover))
        .route("/updatemusic", put(updatemusic))
        .route("/search/lyrics", get(search_lyrics))
        .route("/get_image", post(get_image))
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
struct ImageGet {
    path: PathBuf,
}

#[derive(serde::Serialize)]
struct ImageGetResponse {
    color: Color,
    data: String,
}

async fn get_image(Json(payload): Json<ImageGet>) -> Json<ImageGetResponse> {
    // TODO: save image in temp dir
    let mut file = std::fs::File::open(&payload.path).unwrap();
    let mut file_buf = vec![];
    file.read_to_end(&mut file_buf).unwrap();

    let img = image::load_from_memory(&file_buf).unwrap();
    // TODO: resize the image
    let pixels = global::utils::get_image_buffer(img);
    let color = color_thief::get_palette(&pixels, color_thief::ColorFormat::Rgb, 1, 2).unwrap();

    let color = Color {
        r: color[0].r,
        g: color[0].g,
        b: color[0].b,
    };

    let data = STANDARD.encode(&file_buf);

    Json(ImageGetResponse { color, data })
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

#[derive(serde::Deserialize)]
struct SearchTerm {
    term: String,
}

async fn search(State(state): State<AppData>, Query(q): Query<SearchTerm>) -> Json<SearchResults> {
    let media = state.media.read().await;
    let dirs = state.dirs.clone();
    let res = media.search(dirs.cache, &q.term);

    Json::from(res)
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
                    image = image.resize(w, h, image::imageops::FilterType::Lanczos3);

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

async fn playlist(State(state): State<AppData>, Path(id): Path<String>) -> Response {
    if let Some(playlist) = state.media.read().await.get_playlist(id.clone()) {
        Json(playlist).into_response()
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

// FIXME: Cache the media state from the client
async fn list_remove(State(state): State<AppData>, Path(id): Path<String>) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(id.clone()) {
        if playlist.delete().is_ok() {
            media.remove_playlist(playlist.path);
            "ok".into_response()
        } else {
            let mut response =
                format!("Something went wrong while removing the playlist [list:id:{id}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn list_remove_track(
    State(state): State<AppData>,
    Path(id): Path<String>,
    Json(payload): Json<Vec<String>>,
) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(id.clone()) {
        let mut playlist = playlist;
        if playlist
            .update(PlaylistAction::RemoveTracks(payload))
            .is_ok()
        {
            media.substitute_playlist(playlist.clone());
            Json(playlist).into_response()
        } else {
            let mut response =
                format!("An error occurred during the action excecution [list:id:{id}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn list_add_track(
    State(state): State<AppData>,
    Path(id): Path<String>,
    Json(payload): Json<Vec<String>>,
) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(id.clone()) {
        let mut playlist = playlist;
        if playlist.update(PlaylistAction::AddTracks(payload)).is_ok() {
            media.substitute_playlist(playlist.clone());
            Json(playlist).into_response()
        } else {
            let mut response =
                format!("Something went wrong while removing the playlist [list:id:{id}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn list_reorder(
    State(state): State<AppData>,
    Path(id): Path<String>,
    Json(payload): Json<Vec<String>>,
) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(id.clone()) {
        let mut playlist = playlist;
        if playlist
            .update(PlaylistAction::UpdateOrder(payload))
            .is_ok()
        {
            media.substitute_playlist(playlist.clone());
            Json(playlist).into_response()
        } else {
            let mut response =
                format!("Something went wrong while removing the playlist [list:id:{id}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

#[derive(serde::Deserialize)]
struct MetaKey {
    key: String,
}

async fn list_remove_meta(
    State(state): State<AppData>,
    Path(id): Path<String>,
    Json(payload): Json<MetaKey>,
) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(id.clone()) {
        let mut playlist = playlist;
        if playlist
            .update(PlaylistAction::RemoveMeta(payload.key))
            .is_ok()
        {
            media.substitute_playlist(playlist.clone());
            Json(playlist).into_response()
        } else {
            let mut response =
                format!("Something went wrong while removing the playlist [list:id:{id}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

#[derive(serde::Deserialize)]
struct MetaKeyValue {
    key: String,
    value: String,
}

async fn list_add_meta(
    State(state): State<AppData>,
    Path(id): Path<String>,
    Json(payload): Json<MetaKeyValue>,
) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(id.clone()) {
        let mut playlist = playlist;
        if playlist
            .update(PlaylistAction::AddMeta(payload.key, payload.value))
            .is_ok()
        {
            media.substitute_playlist(playlist.clone());
            Json(playlist).into_response()
        } else {
            let mut response =
                format!("Something went wrong while removing the playlist [list:id:{id}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

#[derive(serde::Deserialize)]
struct CreateList {
    meta: Vec<(String, String)>,
    tracks: Vec<String>,
}

async fn list_create(State(state): State<AppData>, Json(payload): Json<CreateList>) -> Response {
    let mut metamap = HashMap::new();
    for (k, v) in payload.meta {
        metamap.insert(k, v);
    }

    match PlaylistData::create(&state.dirs.audio, metamap, payload.tracks) {
        Ok(_) => {
            let mut response = "Unable to create a new palylist".into_response();
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
        Err(_) => "ok".into_response(),
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
