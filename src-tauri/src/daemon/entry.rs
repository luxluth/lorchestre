use super::{
    config::{self, Dir},
    global::{self, Color, Media, SearchResults, Track},
    list::PlaylistData,
    utils,
};
use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::{header::CACHE_CONTROL, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, delete, get, post, put},
    Json, Router,
};
use axum_extra::{
    extract::OptionalQuery,
    headers::{AcceptRanges, HeaderMapExt, Range},
    TypedHeader,
};
use axum_range::{KnownSize, Ranged, RangedStream};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine as _,
};
use futures::{sink::SinkExt, stream::StreamExt};
use image::ImageReader;
use std::{collections::HashMap, sync::Arc, time::Duration};
use std::{
    io::{BufWriter, Cursor, Read},
    path::PathBuf,
};
use tokio::sync::RwLock;
use tokio::{
    fs::File,
    sync::mpsc::{channel, Receiver, Sender},
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{info, warn};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const LRCLIB: &str = "https://lrclib.net/api";

#[derive(Debug, Clone)]
struct AppData {
    media: Arc<RwLock<Media>>,
    dirs: Dir,
    sx: Sender<AppMessage>,
    tx: Arc<RwLock<Receiver<AppMessage>>>,
}

enum AppMessage {
    NewMedia(Media),
    Search(String),
    LocalSearch(String),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct SearchQuery {
    q: String,
}

async fn on_connect(ws: WebSocketUpgrade, State(state): State<AppData>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppData) {
    let sx = state.sx.clone();
    let tx = state.tx.clone();
    let (mut sender, mut receiver) = socket.split();
    tokio::spawn(async move {
        while let Some(msg) = tx.write().await.recv().await {
            match msg {
                AppMessage::NewMedia(media) => {
                    let _ = sender
                        .send(Message::Text(
                            format!("newmedia\n{}", serde_json::to_string(&media).unwrap()).into(),
                        ))
                        .await;
                }
                AppMessage::Search(query) => {
                    let media = state.media.read().await.clone();
                    let dirs = state.dirs.clone();
                    let res = media.search(dirs.cache, &query);
                    let _ = sender
                        .send(Message::Text(
                            format!("searchresponse\n{}", serde_json::to_string(&res).unwrap())
                                .into(),
                        ))
                        .await;
                }
                AppMessage::LocalSearch(query) => {
                    let media = state.media.read().await.clone();
                    let dirs = state.dirs.clone();
                    let res = media.search(dirs.cache, &query);
                    let _ = sender
                        .send(Message::Text(
                            format!("localsr\n{}", serde_json::to_string(&res).unwrap()).into(),
                        ))
                        .await;
                }
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            if let Some((event, body)) = text.split_once('\n') {
                match event {
                    "localsearch" => {
                        let _ = sx
                            .send(AppMessage::LocalSearch(body.trim().to_string()))
                            .await;
                    }
                    "search" => {
                        let _ = sx.send(AppMessage::Search(body.trim().to_string())).await;
                    }
                    _ => {}
                }
            }
        }
    }
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
    let response = req_client
        .get(format!("http://{host}:{port}"))
        .timeout(Duration::from_secs(10))
        .send()
        .await;
    if response.is_ok() {
        tracing::error!("Daemon already running");
        return Err("Daemon already running".into());
    } else {
        drop(req_client);
        drop(response);
    }

    let m = utils::cache_resolve(&dirs.cache, win).await;
    let media_data = Arc::new(RwLock::new(m));
    let (sx, tx) = channel(10);

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let app = Router::new()
        .route("/ws", any(on_connect))
        .route("/", get(ping))
        // TODO: No cache
        .route("/media", get(media))
        .route("/localsearch", get(search))
        .route("/audio", get(audio))
        .route("/lyrics", get(lyrics))
        .route("/album/{id}", get(album))
        // TODO: Do not cache this at all
        .route("/playlist/{path}", get(playlist))
        // ------ palylist action
        .route("/playlist/delete/{path}", delete(list_remove))
        .route("/playlist/create", post(list_create))
        .route("/playlist/update/{path}", put(list_update))
        // ------ palylist action
        .route("/cover/{handle}", get(cover))
        .route("/updatemusic", put(updatemusic))
        .route("/search/lyrics", get(search_lyrics))
        .route("/get_image", post(get_image))
        .with_state(AppData {
            media: media_data,
            dirs: dirs.clone(),
            sx,
            tx: Arc::new(RwLock::new(tx)),
        })
        .layer(ServiceBuilder::new().layer(cors_layer))
        .layer(TraceLayer::new_for_http());

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
            .timeout(Duration::from_secs(10))
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
    let _ = state.sx.clone().send(AppMessage::NewMedia(m)).await;
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

async fn playlist(State(state): State<AppData>, Path(path): Path<String>) -> Response {
    if let Some(playlist) = state.media.read().await.get_playlist(path.clone()) {
        let mut response = Json(playlist).into_response();
        response
            .headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));

        response
    } else {
        let mut response = format!("no playlist found with the path of {path}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
            .headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        response
    }
}

// FIXME: Cache the media state from the client
async fn list_remove(State(state): State<AppData>, Path(path): Path<String>) -> Response {
    let mut media = state.media.write().await;
    if let Some(playlist) = media.get_playlist(path.clone()) {
        if playlist.delete().is_ok() {
            media.remove_playlist(playlist.path);
            "ok".into_response()
        } else {
            let mut response =
                format!("Something went wrong while removing the playlist [list:path:{path}]")
                    .into_response();
            *response.status_mut() = StatusCode::NOT_FOUND;
            response
        }
    } else {
        let mut response = format!("no playlist found with the id of {path}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn list_update(
    State(state): State<AppData>,
    Path(path): Path<String>,
    Json(payload): Json<List>,
) -> Response {
    let mut metamap = HashMap::new();
    for (k, v) in payload.meta {
        metamap.insert(k, v);
    }

    let mut media = state.media.write().await;
    let dirs = state.dirs.clone();
    if let Some(mut playlist) = media.get_playlist(path) {
        match playlist.update(metamap, payload.tracks) {
            Ok(_) => {
                media.substitute_playlist(playlist);
                media.cache(dirs.cache, None);
                let _ = state
                    .sx
                    .clone()
                    .send(AppMessage::NewMedia(media.clone()))
                    .await;
                "ok".into_response()
            }
            Err(e) => {
                let error = format!("{e}");
                let mut response = error.into_response();
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                response
            }
        }
    } else {
        "Playlist not found".into_response()
    }
}

#[derive(serde::Deserialize)]
struct List {
    meta: Vec<(String, String)>,
    tracks: Vec<String>,
}

#[derive(serde::Serialize)]
struct ResponsePath {
    path: String,
}

async fn list_create(State(state): State<AppData>, Json(payload): Json<List>) -> Response {
    let mut metamap = HashMap::new();
    for (k, v) in payload.meta {
        metamap.insert(k, v);
    }

    match PlaylistData::create(&state.dirs.audio, metamap, payload.tracks) {
        Err(e) => {
            let error = format!("{e}");
            let mut response = error.into_response();
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
        Ok(path) => {
            let m = utils::cache_resolve(&state.dirs.cache, None).await;
            let mut binding = state.media.write().await;
            binding.swap_with(m.clone());
            let _ = state.sx.clone().send(AppMessage::NewMedia(m)).await;

            Json(ResponsePath { path }).into_response()
        }
    }
}
async fn audio(
    range: Option<TypedHeader<Range>>,
    State(state): State<AppData>,
    Query(music_path): Query<MusicPath>,
) -> Result<Response<RangedStream<KnownSize<File>>>, Response<Body>> {
    let path = String::from_utf8_lossy(&URL_SAFE.decode(music_path.path).unwrap()).to_string();
    if let Some(track) = state.media.read().await.get_song(&path) {
        let file = File::open(&track.file_path).await.unwrap();
        let body = KnownSize::file(file).await.unwrap();
        let r = range.clone().map(|TypedHeader(range)| range);
        let body = Ranged::new(r, body).try_respond().unwrap();
        let content_range = body.content_range.map(TypedHeader);
        let content_length = TypedHeader(body.content_length);
        let accept_ranges = TypedHeader(AcceptRanges::bytes());
        let stream = body.stream;
        let status = match content_range {
            Some(_) => StatusCode::PARTIAL_CONTENT,
            None => StatusCode::OK,
        };

        let mut resp = Response::new(stream);
        *resp.status_mut() = status;
        resp.headers_mut().typed_insert(content_range.unwrap().0);
        resp.headers_mut().typed_insert(content_length.0);
        resp.headers_mut().typed_insert(accept_ranges.0);

        Ok(resp)
    } else {
        warn!("{path} not founded");
        let mut response = format!("no song found with the id of {path}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        Err(response)
    }
}

async fn ping() -> String {
    format!("OK lorchestre v{}", config::VERSION)
}

async fn media(State(state): State<AppData>) -> Json<Media> {
    Json(state.media.read().await.clone())
}
