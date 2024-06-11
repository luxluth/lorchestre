use mu::Track;
use rodio::source::Source;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{channel, Sender};
use std::sync::{
    atomic::{AtomicUsize, Ordering::SeqCst},
    Arc, Mutex,
};
use std::time::Duration;
use tauri::Manager;

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
    // Shuffle,
}

pub enum QueueMode {
    Continue,
    // Repeat,
    // RepeatAll,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum QueueAddMode {
    Top,
    Bottom,
}
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct QueueAdd {
    pub tracks: Vec<Track>,
    pub mode: QueueAddMode,
}

pub enum PlayerMessage {
    Stop,
    Play(Track),
    AddToQueue(QueueAdd),
    RemoveFromQueue(String),
    ClearQueue,
    PrevTrack,
    NextTrack,
    RequestPause,
    RequestPlay,

    TimeUpdate(f32),

    VolumeTo(f32),
    SeekTo(f32),
}

pub enum SinkMessage {
    SeekTo(Duration),
    Stop,
    VolumeTo(f32),
    Pause,
    Play,
}

pub struct InnerPlayer {
    pub queue: VecDeque<Track>,
    pub current_track: Option<Track>,
    pub current_time: f32,

    pub volume: f32,

    pub paused: bool,
    pub duration: usize,
    pub history: Vec<Track>,
    pub qmode: QueueMode,
    pub pmode: PlayingMode,

    pub sink_sx: Option<Sender<SinkMessage>>,
    pub window: Option<tauri::Window>,
}

pub struct Player;

impl Player {
    pub fn start(&self, window: tauri::Window) -> Sender<PlayerMessage> {
        let (sx, rx) = channel::<PlayerMessage>();
        let sx2 = sx.clone();
        std::thread::spawn(move || {
            let mut player = InnerPlayer::default();
            player.window = Some(window);
            player.adding_window_event(sx2.clone());
            'l: while let Ok(recv) = rx.recv() {
                match recv {
                    PlayerMessage::Stop => {
                        eprintln!("[player:stop] stopped");
                        player.reset();
                        break 'l;
                    }
                    PlayerMessage::Play(track) => {
                        player.play(track.clone(), sx2.clone());
                        player.emit("newtrack", track);
                    }
                    PlayerMessage::AddToQueue(addition) => {
                        match addition.mode {
                            QueueAddMode::Top => {
                                let mut new_queue = addition.tracks;
                                new_queue.extend(player.queue.clone());
                                player.queue = VecDeque::from_iter(new_queue);
                            }
                            QueueAddMode::Bottom => {
                                player.queue.extend(addition.tracks);
                            }
                        };
                        player.emit("queueupdate", player.queue.clone());
                    }
                    PlayerMessage::RemoveFromQueue(id) => {
                        player.remove_track_from_queue(id);
                        player.emit("queueupdate", player.queue.clone());
                    }
                    PlayerMessage::ClearQueue => {
                        player.queue.clear();
                        player.emit("queueupdate", player.queue.clone());
                    }
                    PlayerMessage::PrevTrack => {
                        if let Some(track) = player.history.pop() {
                            if let Some(ct) = player.current_track.clone() {
                                player.queue.insert(0, ct);
                            }
                            player.play(track.clone(), sx2.clone());
                            player.emit("newtrack", track);
                        }
                        player.emit("queueupdate", player.queue.clone());
                    }
                    PlayerMessage::NextTrack => {
                        if let Some(track) = player.queue.pop_front() {
                            if let Some(ct) = player.current_track.clone() {
                                player.history.push(ct);
                            }
                            player.play(track.clone(), sx2.clone());
                            player.emit("newtrack", track);
                        } else {
                            player.emit::<Option<Track>>("newtrack", None);
                            player.reset();
                        }

                        player.emit("queueupdate", player.queue.clone());
                    }
                    PlayerMessage::TimeUpdate(time) => {
                        player.update_time(time);
                    }
                    PlayerMessage::VolumeTo(value) => {
                        player.volume_to(value);
                    }
                    PlayerMessage::SeekTo(time) => {
                        player.seek_to(time);
                    }
                    PlayerMessage::RequestPause => player.pause(),
                    PlayerMessage::RequestPlay => player.unpause(),
                }
            }
        });

        sx
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct SinkInfo {
    pub current_time: usize,
    pub is_paused: bool,
}

impl InnerPlayer {
    #[inline]
    pub fn update_time(&mut self, value: f32) {
        if self.current_time != value {
            self.current_time = value;
            self.emit("timeupdate", value);
        }
    }

    #[inline]
    pub fn pause(&mut self) {
        self.paused = true;
        if let Some(sx) = self.sink_sx.clone() {
            let _ = sx.send(SinkMessage::Pause);
            self.emit::<Option<String>>("pause", None);
        }
    }

    #[inline]
    pub fn unpause(&mut self) {
        self.paused = false;
        if let Some(sx) = self.sink_sx.clone() {
            let _ = sx.send(SinkMessage::Play);
            self.emit::<Option<String>>("play", None);
        }
    }

    #[inline]
    pub fn volume_to(&mut self, value: f32) {
        if let Some(sx) = self.sink_sx.clone() {
            let _ = sx.send(SinkMessage::VolumeTo(value));
        }
    }

    #[inline]
    pub fn seek_to(&mut self, time: f32) {
        if let Some(sx) = self.sink_sx.clone() {
            let _ = sx.send(SinkMessage::SeekTo(Duration::from_secs_f32(time)));
        }
    }

    pub fn adding_window_event(&self, sx: Sender<PlayerMessage>) {
        if let Some(window) = self.window.clone() {
            let sx2 = sx.clone();
            window.on_window_event(move |ev| match ev {
                tauri::WindowEvent::CloseRequested { .. } => {
                    let _ = sx2.send(PlayerMessage::Stop);
                }
                _ => {}
            });

            let sx2 = sx.clone();
            window.listen("play", move |ev| {
                if let Ok(track) = serde_json::from_str::<Track>(ev.payload()) {
                    let _ = sx2.send(PlayerMessage::Play(track));
                }
            });

            let sx2 = sx.clone();
            window.listen("queueadd", move |ev| {
                match serde_json::from_str::<QueueAdd>(ev.payload()) {
                    Ok(add) => {
                        let _ = sx2.send(PlayerMessage::AddToQueue(add));
                    }
                    Err(e) => {
                        println!("{e:?}");
                    }
                }
            });

            let sx2 = sx.clone();
            window.listen("queueremove", move |ev| {
                let _ = sx2.send(PlayerMessage::RemoveFromQueue(ev.payload().to_string()));
            });

            let sx2 = sx.clone();
            window.listen("queueclear", move |_ev| {
                let _ = sx2.send(PlayerMessage::ClearQueue);
            });

            let sx2 = sx.clone();
            window.listen("nexttrack", move |_ev| {
                let _ = sx2.send(PlayerMessage::NextTrack);
            });

            let sx2 = sx.clone();
            window.listen("prevtrack", move |_ev| {
                let _ = sx2.send(PlayerMessage::PrevTrack);
            });

            let sx2 = sx.clone();
            window.listen("volumeto", move |ev| {
                if let Ok(volume) = ev.payload().parse::<f32>() {
                    let _ = sx2.send(PlayerMessage::VolumeTo(volume));
                }
            });

            let sx2 = sx.clone();
            window.listen("seekto", move |ev| {
                if let Ok(time) = ev.payload().parse::<f32>() {
                    let _ = sx2.send(PlayerMessage::SeekTo(time));
                }
            });

            let sx2 = sx.clone();
            window.listen("requestplay", move |_ev| {
                let _ = sx2.send(PlayerMessage::RequestPlay);
            });

            let sx2 = sx.clone();
            window.listen("requestpause", move |_ev| {
                let _ = sx2.send(PlayerMessage::RequestPause);
            });
        }
    }

    #[inline]
    pub fn emit<T>(&self, event: &str, payload: T)
    where
        T: serde::Serialize + Clone,
    {
        if let Some(window) = self.window.clone() {
            let _ = window.emit(event, payload);
        }
    }

    #[inline]
    pub fn stop(&mut self) {
        if let Some(sd) = self.sink_sx.clone() {
            let _ = sd.send(SinkMessage::Stop);
            self.sink_sx = None;
        }
    }

    pub fn play(&mut self, t: Track, sxp: Sender<PlayerMessage>) {
        self.stop();

        let (sx, rx) = channel::<SinkMessage>();
        self.sink_sx = Some(sx.clone());
        self.current_track = Some(t.clone());

        std::thread::spawn(move || {
            let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
            let sink = rodio::Sink::try_new(&handle).unwrap();
            let file = File::open(&t.file_path).unwrap();

            let counter = Arc::from(AtomicUsize::new(0));
            let periodic_counter = counter.clone();
            let access_time = Duration::from_millis(1);

            let source = rodio::Decoder::new(BufReader::new(file))
                .unwrap()
                .periodic_access(access_time, move |_| {
                    let _ = periodic_counter.fetch_add(1, SeqCst);
                    let secs = periodic_counter.load(SeqCst) as f32 / 1000f32;
                    let secs = (secs * 100f32).trunc() / 100f32;
                    let _ = sxp.send(PlayerMessage::TimeUpdate(secs));
                });
            sink.append(source);
            eprintln!("playing");

            'sinkplaying: while let Ok(recv) = rx.recv() {
                match recv {
                    SinkMessage::SeekTo(time) => {
                        println!("seekTo: {}s", time.as_secs());
                        let _ = sink.try_seek(time);
                    }
                    SinkMessage::Stop => {
                        sink.stop();
                        break 'sinkplaying;
                    }
                    SinkMessage::VolumeTo(level) => {
                        println!("vol: {}", level);
                        sink.set_volume(level);
                    }
                    SinkMessage::Pause => {
                        println!("pause");
                        sink.pause();
                    }
                    SinkMessage::Play => {
                        println!("play");
                        sink.play();
                    }
                }
            }
        });
    }

    #[inline]
    pub fn reset(&mut self) {
        self.current_track = None;
        self.pause();
        self.stop();
    }

    #[inline]
    pub fn remove_track_from_queue(&mut self, id: String) {
        self.queue.retain(|t| t.id != id)
    }
}

impl Default for InnerPlayer {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            current_track: None,
            current_time: 0.0,
            volume: 1.0,
            paused: true,
            duration: 0,
            history: vec![],
            qmode: QueueMode::Continue,
            pmode: PlayingMode::Normal,
            sink_sx: None,
            window: None,
        }
    }
}
