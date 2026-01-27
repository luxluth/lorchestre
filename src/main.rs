use freya::{
    prelude::{AppComponent, Color, LaunchConfig, WindowConfig, launch},
    winit::platform::wayland::WindowAttributesExtWayland,
};
use lorchestre::{track::Orchestra, ui::start_view::StartPage};

fn main() {
    let _ = env_logger::try_init();

    let (width, height) = (600, 600);

    let mut orchestra = Orchestra::new();
    if !orchestra.load_from_cache() {
        orchestra.index("/home/luxluth/Music/".into());
        orchestra.save();
    }

    let start_view = StartPage;
    let config = WindowConfig::new(AppComponent::new(start_view))
        .with_size(width as f64, height as f64)
        .with_title("Orchestre")
        .with_background(Color::TRANSPARENT)
        .with_decorations(false)
        .with_transparency(true)
        .with_window_attributes(|attr, _ev| attr.with_name("orchestre", ""));

    launch(LaunchConfig::new().with_window(config));
}
