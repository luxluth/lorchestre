use freya::prelude::*;

pub struct StartPage;

impl Component for StartPage {
    fn render(&self) -> impl IntoElement {
        let mut count = use_state(|| 0);

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .color(Color::WHITE)
            .padding(Gaps::new_all(12.))
            .on_mouse_up(move |_| *count.write() += 1)
            .child(format!("Click to increase -> {}", count.read()))
    }
}
