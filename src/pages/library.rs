use mtk::{
    Size, Style, clr,
    ui::{
        View, ViewStyleExt,
        widgets::{column, text},
    },
};

use crate::{Supervisor, orchestra::mu_thread::AppMsg, pages::Theme};

pub fn render(
    _state: &Supervisor,
    theme: Theme,
) -> impl View<Supervisor, Message = AppMsg> + use<> {
    column((text(""),)).style(
        Style::new()
            .width(Size::Fill)
            .height(Size::Fill)
            .bg_color(theme.bg()),
    )
}
