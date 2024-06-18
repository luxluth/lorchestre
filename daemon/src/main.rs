mod config;
mod utils;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use axum_extra::{headers::Range, TypedHeader};
use axum_range::{KnownSize, Ranged};
use mud::Media;
use tokio::fs::File;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let dirs = config::get_dirs();
    let media = utils::cache_resolve(&dirs.cache).await;

    let app = Router::new()
        .route("/", get(ping))
        .route("/media", get(get_media))
        .route("/audio/:id", get(get_audio))
        .with_state(media)
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()));

    let listener = tokio::net::TcpListener::bind("localhost:7700").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_audio(
    range: Option<TypedHeader<Range>>,
    State(media): State<Media>,
    Path(id): Path<String>,
) -> Response {
    if let Some(track) = media.get_song(&id) {
        info!("{:?}", range);
        let file = File::open(&track.file_path).await.unwrap();
        let body = KnownSize::file(file).await.unwrap();
        let range = range.map(|TypedHeader(range)| range);
        let mut response = Ranged::new(range, body).into_response();
        *response.status_mut() = StatusCode::PARTIAL_CONTENT;
        info!("{:?}", response);
        response
    } else {
        let mut response = format!("no song founded with the id of {id}").into_response();
        *response.status_mut() = StatusCode::NOT_FOUND;
        response
    }
}

async fn ping() -> String {
    format!("OK mud v{}", config::VERSION)
}

async fn get_media(State(media): State<Media>) -> Json<Media> {
    Json(media)
}
