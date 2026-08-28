use std::sync::{Arc, Mutex, mpsc::Sender};

use lorchestre::Orchestra;
use mtk::{
    Size, Style, clr,
    ui::{
        View, ViewStyleExt,
        widgets::{column, text},
    },
    windowing::{Window, WindowAttributes, WindowHandle},
};

#[derive(Clone)]
enum OrchestraMsg {
    Init,
}

struct OrchestraManager {
    orchestra: Orchestra,
    win_handle: Mutex<Option<WindowHandle<OrchestraMsg>>>,
    // sender_to_orchestrator: Sender<String>,
}

impl std::fmt::Display for OrchestraManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OrchestraManager(index@{})",
            self.orchestra.collection.index.values.len()
        )
    }
}

fn update(state: &mut Arc<OrchestraManager>, msg: OrchestraMsg) {
    println!("{state}");
}

fn app_view(
    _state: &Arc<OrchestraManager>,
) -> impl View<Arc<OrchestraManager>, Message = OrchestraMsg> + use<> {
    column(vec![text("")]).style(
        Style::new()
            .padding(10.0)
            .width(Size::Fill)
            .height(Size::Fill)
            .bg_color(clr!(0x242424FF)),
    )
}

fn main() {
    let _ = env_logger::try_init();

    let (width, height) = (600, 600);

    let mut orchestra = Orchestra::new();
    if !orchestra.load_from_cache() {
        let music_dir =
            dirs::audio_dir().unwrap_or_else(|| dirs::home_dir().unwrap().join("Music"));
        eprintln!("{music_dir:?}");
        orchestra.index(music_dir);
        orchestra.save();
    }

    let orchestra_mgr = Arc::new(OrchestraManager {
        orchestra,
        win_handle: Mutex::new(None),
    });

    let mut window = Window::with(orchestra_mgr.clone(), update, app_view);

    if let Ok(mut handle_lock) = orchestra_mgr.win_handle.lock() {
        let handle = window.handle();
        let _ = handle.send(OrchestraMsg::Init);

        *handle_lock = Some(handle);
    }

    window.present_with(
        WindowAttributes::default()
            .with_title("Orchestre")
            .with_size((width, height).into())
            .with_app_id("orchestre")
            .with_resizable(true),
    );
}
