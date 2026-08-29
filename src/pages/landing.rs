use mtk::{
    AlignItems, AlignSelf, JustifyContent, Size, Style, SvgData, clr,
    ui::{
        View, ViewStyleExt,
        widgets::{column, svg},
    },
};

use crate::{Supervisor, orchestra::mu_thread::OrchestraMsg};

const BANNER: &'static str = include_str!("../assets/banner.svg");

pub fn render(_state: &Supervisor) -> impl View<Supervisor, Message = OrchestraMsg> + use<> {
    column((svg(SvgData::from_str(BANNER).unwrap())
        .color(clr!(white))
        .fit(mtk::ObjectFit::Contain)
        .style(
            Style::new()
                .width(Size::Percent(0.70))
                .height(Size::Fill)
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .align_self(AlignSelf::Center),
        ),))
    .style(
        Style::new()
            .width(Size::Fill)
            .height(Size::Fill)
            .bg_color(clr!(0x242424FF)),
    )
}
