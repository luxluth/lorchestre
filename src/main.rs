use lorchestre::Orchestra;
use mtk::{
    Size, Style, clr,
    ui::{
        View, ViewStyleExt,
        widgets::{column, text},
    },
    windowing::{Window, WindowAttributes},
};

#[derive(Clone)]
enum OrchestraMsg {}

fn update(state: &mut Orchestra, msg: OrchestraMsg) {}

fn app_view(_state: &Orchestra) -> impl View<Orchestra, Message = OrchestraMsg> + use<> {
    column(vec![text("")]).style(
        Style::new()
            .padding(10.0)
            .width(Size::Fill)
            .height(Size::Fill)
            .bg_color(clr!(red)),
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

    let mut window = Window::with(orchestra, update, app_view);
    window.present_with(
        WindowAttributes::default()
            .with_title("Orchestre")
            .with_size((width, height).into())
            .with_app_id("orchestre")
            .with_resizable(true),
    );
}
