use std::sync::Arc;

use arc_swap::ArcSwap;
use mtk::{
    Edges, JustifyContent, Lens, ObjectFit, ScrollbarStyle, Size, Style, SvgData,
    TransitionProperty, ViewStyleExt,
    animation::Curve,
    ui::{
        EventKind, View, ViewEventExt,
        widgets::{column, container, row, scroll_view, svg, text},
    },
};

use crate::{
    icons,
    orchestra::{Orchestra, track::Id},
    pages::Theme,
    svg_display,
};

#[derive(Clone, Debug)]
pub enum AlbumMsg {
    GotoLibrary,
}

#[derive(Lens, Clone, Debug, Default)]
pub struct AlbumState {
    pub album_id: Id,
}

pub fn render(
    state: &AlbumState,
    orchestra: Option<Arc<ArcSwap<Orchestra>>>,
    theme: Theme,
) -> impl View<AlbumState, Message = AlbumMsg> + use<> {
    let orch = orchestra.as_ref().unwrap();
    let guard = orch.load();

    let album = guard.get_album(&state.album_id).unwrap();
    let cover = guard.get_cover(&album.cover.unwrap_or_default());
    let (main_bg, main_fg) = cover
        .clone()
        .map(|cover| {
            cover
                .swatches
                .first()
                .map(|e| {
                    (
                        e.to_color(),
                        e.to_color().get_tinted_contrast_text(4.5, 0.12),
                    )
                })
                .unwrap_or((theme.bg(), theme.fg()))
        })
        .unwrap();

    let scrollbar_style = ScrollbarStyle {
        thumb_color: main_fg,
        track_color: Some(main_bg.with_alpha(38)),
        ..Default::default()
    };

    column((
        row((
            column((
                text(&album.name).style(
                    Style::new()
                        .width(Size::Fill)
                        .apply(theme.heading())
                        .update_text_style(|t| {
                            t.color = main_fg;
                            t.wrap = true;
                        }),
                ),
                text("ALBUM").style(
                    Style::new()
                        .apply(theme.subtitle())
                        .update_text_style(|t| t.color = main_fg),
                ),
            ))
            .style(Style::new().width(Size::Fill).height(Size::Fit)),
            svg_display!(SvgData::from_str(icons::X).unwrap(), main_fg, 38, 2.)
                .style(
                    Style::new()
                        .opacity(0.7)
                        .on_hover(|s| s.opacity(1.))
                        .on_active(|s| s.scale(0.98))
                        .transition(TransitionProperty::Opacity, 150., Curve::ease_in_out()),
                )
                .on_event(EventKind::Click, |_| Some(AlbumMsg::GotoLibrary)),
        ))
        .style(
            Style::new()
                .width(Size::Fill)
                .gap(10.)
                .padding_edges(Edges::lr(30.).top(20.))
                // .align_items(AlignItems::Center)
                .justify_content(JustifyContent::SpaceBetween),
        ),
        scroll_view(text(""))
            .scrollbar(scrollbar_style)
            .style(Style::new().height(Size::Fill).width(Size::Fill)),
    ))
    .style(
        Style::new()
            .bg_color(main_bg)
            .width(Size::Fill)
            .height(Size::Fill)
            .gap(28.),
    )
    .on_global_key_down(|_, k| {
        if k.is_escape() {
            Some(AlbumMsg::GotoLibrary)
        } else {
            None
        }
    })
}
