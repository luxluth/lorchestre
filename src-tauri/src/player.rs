use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

use mu::Track;

pub struct PlayerState {
    pub player: Arc<Mutex<Player>>,
    pub running: Arc<Mutex<bool>>,
    pub sx: Arc<Mutex<Option<Sender<PlayerMessage>>>>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            player: Arc::new(Mutex::new(Player)),
            running: Arc::new(Mutex::new(false)),
            sx: Arc::new(Mutex::new(None)),
        }
    }
}

pub enum PlayingMode {
    Normal,
    Shuffle,
}

pub enum QueueMode {
    Continue,
    Repeat,
    RepeatAll,
}

pub enum PlayerMessage {
    Stop,
    Play(Track),
}

pub struct InnerPlayer {
    pub queue: Vec<Track>,
    pub current_track: Option<Track>,
    pub current_time: usize,

    pub volume: f32,

    pub paused: bool,
    pub duration: usize,
    pub history: Vec<Track>,
    pub qmode: QueueMode,
    pub pmode: PlayingMode,
}

pub struct Player;

impl Player {
    pub fn start(&self, window: tauri::Window) -> Sender<PlayerMessage> {
        let (sx, rx) = channel::<PlayerMessage>();
        let sx2 = sx.clone();
        std::thread::spawn(move || {
            let player = InnerPlayer::default();
            player.adding_window_event(sx2, window.clone());
            'l: while let Ok(recv) = rx.recv() {
                match recv {
                    PlayerMessage::Stop => {
                        eprintln!("[player:stop] stopped");
                        break 'l;
                    }
                    PlayerMessage::Play(track) => {
                        eprintln!("[player:play] {:?}", track);
                    }
                }
            }
        });

        sx
    }
}

impl InnerPlayer {
    pub fn adding_window_event(&self, sx: Sender<PlayerMessage>, window: tauri::Window) {
        let sx2 = sx.clone();
        window.on_window_event(move |ev| match ev {
            tauri::WindowEvent::CloseRequested { .. } => {
                let _ = sx.send(PlayerMessage::Stop);
            }
            _ => {}
        });
        window.listen("play", move |ev| {
            if let Ok(track) = serde_json::from_str::<Track>(ev.payload()) {
                let _ = sx2.send(PlayerMessage::Play(track));
            }
        });
    }
}

impl Default for InnerPlayer {
    fn default() -> Self {
        Self {
            queue: vec![],
            current_track: None,
            current_time: 0,
            volume: 1.0,
            paused: true,
            duration: 0,
            history: vec![],
            qmode: QueueMode::Continue,
            pmode: PlayingMode::Normal,
        }
    }
}
