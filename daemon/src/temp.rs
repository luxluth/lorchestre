mod config;
mod state;

use axum::extract::State;
use axum::{routing::get, Router};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<state::Message>,
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on(
        "join",
        |socket: SocketRef,
         Data::<String>(room),
         store: socketioxide::extract::State<state::MessageStore>| async move {
            info!("Recieved join: {:?}", room);
            let _ = socket.leave_all();
            let messages = store.get(&room).await;
            let _ = socket.join(room);
            let _ = socket.emit("messages", Messages { messages });
        },
    );

    socket.on(
        "message",
        |socket: SocketRef,
         Data::<MessageIn>(data),
         store: socketioxide::extract::State<state::MessageStore>| async move {
            info!("Recieved message: {:?}", data);

            let response = state::Message {
                text: data.text,
                user: format!("anonymus-{}", socket.id),
                date: chrono::Utc::now(),
            };

            store.insert(&data.room, response.clone()).await;

            let _ = socket.within(data.room).emit("message", response);
        },
    )
}

async fn fake_main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let messages = state::MessageStore::default();

    let (layer, io) = SocketIo::builder().with_state(messages).build_layer();

    io.ns("/", on_connect);

    let app = Router::new().route("/", get(ping)).with_state(io).layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer),
    );

    info!("MUD has started");

    let listener = tokio::net::TcpListener::bind("localhost:7700").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ping(State(io): State<SocketIo>) -> String {
    let _ = io.emit("hello", "world");
    format!("OK mud v{}", config::VERSION)
}
