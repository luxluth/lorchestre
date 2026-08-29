use std::sync::mpsc::Sender;

mod orchestra;
mod pages;

use mtk::{
    animation::Curve,
    ui::{Transition, View, router},
    windowing::{Window, WindowAttributes},
};

use crate::orchestra::mu_thread::{Mu, MuCommand, OrchestraMsg};

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Landing,
}

pub struct Supervisor {
    pub mu_sx: Sender<MuCommand>,
    current_page: Page,
}

fn update(state: &mut Supervisor, msg: OrchestraMsg) {
    match msg {
        OrchestraMsg::Ready => {
            eprintln!("(ready)");
        }
        OrchestraMsg::NeedIndexing => {
            let _ = state.mu_sx.send(MuCommand::StartIndexing);
        }
        OrchestraMsg::Indexing(file_path) => {
            println!("+ {}", file_path.as_os_str().to_string_lossy())
        }
    }
}

fn app(state: &Supervisor) -> impl View<Supervisor, Message = OrchestraMsg> + use<> {
    router(state.current_page, render_page(state)).transition(Transition::Fade {
        duration_ms: 220.0,
        curve: Curve::ease_out(),
    })
}

fn render_page(state: &Supervisor) -> impl View<Supervisor, Message = OrchestraMsg> + use<> {
    match state.current_page {
        Page::Landing => pages::landing::render(state),
    }
}

fn main() {
    let _ = env_logger::try_init();

    let (width, height) = (600, 600);

    let mu = Mu::new();
    let mu_sx = mu.sender();

    let orchestra_mgr = Supervisor {
        mu_sx,
        current_page: Page::Landing,
    };

    let mut window = Window::with(orchestra_mgr, update, app);
    mu.spawn(window.handle());

    #[cfg(feature = "debug")]
    window.enable_terminal_debugger();

    window.present_with(
        WindowAttributes::default()
            .with_title("Orchestre")
            .with_size((width, height).into())
            .with_app_id("orchestre")
            .with_min_size(Some((970, 630).into()))
            .with_resizable(true),
    );
}
