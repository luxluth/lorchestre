use std::{path::PathBuf, sync::mpsc::Sender};

mod fonts;
mod icons;
mod orchestra;
mod pages;

use mtk::{
    Lens,
    animation::Curve,
    ui::{Transition, View, ViewAdaptExt, router},
    windowing::{Window, WindowAttributes},
};

use crate::{
    orchestra::mu_thread::{AppMsg, Mu, MuCommand, OrchestraMsg},
    pages::{
        PageView, Theme,
        landing::{LandingMsg, LandingState},
    },
};

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Landing,
    Library,
}

#[derive(Lens)]
pub struct Supervisor {
    current_page: Page,
    pub mu_sx: Sender<MuCommand>,
    pub landing: LandingState,
    pub theme: Theme,
}

fn update(state: &mut Supervisor, msg: AppMsg) {
    match msg {
        AppMsg::Orchestra(omsg) => match omsg {
            OrchestraMsg::Ready => {
                state.landing.log = None;
                state.landing.is_indexing = true;
                state.current_page = Page::Library;
            }
            OrchestraMsg::NeedIndexing => {
                state.landing.is_indexing = false;
                // let _ = state.mu_sx.send(MuCommand::StartIndexing);
            }
            OrchestraMsg::Indexing(file_path) => {
                state.landing.log = Some(format!("+ {}", file_path.as_os_str().to_string_lossy()))
            }
        },
        AppMsg::Landing(lmsg) => match lmsg {
            LandingMsg::StartIndexing => {
                let path = PathBuf::from(&state.landing.music_dir);
                if path.exists() && path.is_dir() {
                    state.landing.is_indexing = true;
                    state.landing.error = None;
                    let _ = state.mu_sx.send(MuCommand::StartIndexing(path));
                } else {
                    state.landing.error = Some("Directory does not exist or is invalid".into());
                }
            }
            LandingMsg::FolderPicked(folder) => {
                state.landing.music_dir = folder.as_os_str().to_string_lossy().to_string();
                state.landing.error = None;
            }
            LandingMsg::PickFolder => {
                let _ = state.mu_sx.send(MuCommand::PickFolder(|path| {
                    AppMsg::Landing(LandingMsg::FolderPicked(path))
                }));
            }
        },
    }
}

fn app(state: &Supervisor) -> impl View<Supervisor, Message = AppMsg> + use<> {
    router(state.current_page, render_page(state)).transition(Transition::Fade {
        duration_ms: 220.0,
        curve: Curve::ease_out(),
    })
}

fn render_page(state: &Supervisor) -> impl View<Supervisor, Message = AppMsg> + use<> {
    match state.current_page {
        Page::Landing => PageView::Landing(
            pages::landing::render(&state.landing, state.theme)
                .adapt(Supervisor::landing, AppMsg::Landing),
        ),
        Page::Library => PageView::Library(pages::library::render(state, state.theme)),
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
        landing: LandingState::default(),
        theme: Theme::Light,
    };

    let mut window = Window::with(orchestra_mgr, update, app)
        .with_font_bytes(fonts::IOSEVKA_REGULAR_BYTES)
        .with_font_bytes(fonts::IOSEVKA_BOLD_BYTES)
        .with_font_bytes(fonts::IOSEVKA_ITALIC_BYTES)
        .with_font_bytes(fonts::IOSEVKA_BOLDITALIC_BYTES)
        .with_font_bytes(fonts::INTER_VARIABLE_REGULAR_BYTES)
        .with_font_bytes(fonts::INTER_VARIABLE_ITALIC_BYTES);

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
