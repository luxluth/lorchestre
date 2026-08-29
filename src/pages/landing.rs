use mtk::{
    AlignItems, AlignSelf, Color, JustifyContent, PositionStrategy, Size, Style, SvgData,
    TextStyle, clr,
    ui::{
        View, ViewStyleExt,
        widgets::{column, progress_bar, svg, text},
    },
};

use crate::{Supervisor, orchestra::mu_thread::OrchestraMsg};

const BANNER: &'static str = include_str!("../assets/banner.svg");
const BG: Color = clr!(0x181818FF);

pub fn render(state: &Supervisor) -> impl View<Supervisor, Message = OrchestraMsg> + use<> {
    column((
        svg(SvgData::from_str(BANNER).unwrap())
            .color(clr!(white))
            .fit(mtk::ObjectFit::Contain)
            .style(
                Style::new()
                    .width(Size::Percent(0.70))
                    .height(Size::Fixed(120))
                    .align_self(AlignSelf::Center),
            ),
        column((progress_bar(0.0)
            .indeterminate(true)
            .fill_color(clr!(white))
            .track_color(BG),))
        .style(
            Style::new()
                .border(1., clr!(0x222222FF))
                .corner_radius(20.)
                .width(Size::Fixed(200))
                .height(Size::Fixed(10)),
        ),
        text(if state.log.is_some() {
            state.log.as_ref().unwrap()
        } else {
            ""
        })
        .style(
            Style::new()
                .set_text_style(
                    TextStyle::new()
                        .color(clr!(white).with_alpha(125))
                        .font_family("Iosevka"),
                )
                .position(PositionStrategy::absolute().bottom(10.).left(10.).build()),
        ),
    ))
    .style(
        Style::new()
            .width(Size::Fill)
            .height(Size::Fill)
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .bg_color(BG),
    )
}
