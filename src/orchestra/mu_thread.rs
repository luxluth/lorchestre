use std::{
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use mtk::windowing::WindowHandle;

use crate::orchestra::Orchestra;

pub enum MuCommand {
    StartIndexing,
}

#[derive(Clone)]
pub enum OrchestraMsg {
    Ready,
    NeedIndexing,
    Indexing(PathBuf),
}

pub struct Mu {
    sx: mpsc::Sender<MuCommand>,
    rx: mpsc::Receiver<MuCommand>,
}

impl Mu {
    pub fn new() -> Self {
        let (sx, rx) = mpsc::channel();
        Mu { sx, rx }
    }

    pub fn sender(&self) -> Sender<MuCommand> {
        self.sx.clone()
    }

    pub fn spawn(self, handle: WindowHandle<OrchestraMsg>) {
        std::thread::Builder::new()
            .name("mumanager".to_string())
            .spawn(move || {
                let mut orchestra = Orchestra::new();
                if orchestra.load_from_cache() {
                    let _ = handle.send(OrchestraMsg::Ready);
                } else {
                    let _ = handle.send(OrchestraMsg::NeedIndexing);
                }

                while let Ok(cmd) = self.rx.recv() {
                    match cmd {
                        MuCommand::StartIndexing => {
                            let music_dir = dirs::audio_dir()
                                .unwrap_or_else(|| dirs::home_dir().unwrap().join("Music"));
                            eprintln!("{music_dir:?}");
                            orchestra.index(music_dir, &handle);
                            orchestra.save();
                            let _ = handle.send(OrchestraMsg::Ready);
                        }
                    }
                }
            })
            .unwrap();
    }
}
