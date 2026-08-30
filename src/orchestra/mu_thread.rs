use std::{
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use mtk::windowing::WindowHandle;

use crate::{orchestra::Orchestra, pages::landing::LandingMsg};

pub enum MuCommand {
    StartIndexing(PathBuf),
    PickFolder(fn(PathBuf) -> AppMsg),
}

#[derive(Clone)]
pub enum OrchestraMsg {
    Ready,
    NeedIndexing,
    Indexing(PathBuf),
}

#[derive(Clone)]
pub enum AppMsg {
    Orchestra(OrchestraMsg),
    Landing(LandingMsg),
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

    pub fn spawn(self, handle: WindowHandle<AppMsg>) {
        std::thread::Builder::new()
            .name("mumanager".to_string())
            .spawn(move || {
                let mut orchestra = Orchestra::new();
                if orchestra.load_from_cache() {
                    let _ = handle.send(AppMsg::Orchestra(OrchestraMsg::Ready));
                } else {
                    let _ = handle.send(AppMsg::Orchestra(OrchestraMsg::NeedIndexing));
                }

                while let Ok(cmd) = self.rx.recv() {
                    match cmd {
                        MuCommand::StartIndexing(dir) => {
                            orchestra.index(dir, &handle);
                            orchestra.save();
                            let _ = handle.send(AppMsg::Orchestra(OrchestraMsg::Ready));
                        }
                        MuCommand::PickFolder(to_msg) => {
                            let h = handle.clone();
                            std::thread::spawn(move || {
                                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                    let _ = h.send(to_msg(folder));
                                }
                            });
                        }
                    }
                }
            })
            .unwrap();
    }
}
