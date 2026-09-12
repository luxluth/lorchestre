use std::sync::Arc;

use arc_swap::ArcSwap;
use mtk::{
    Edges, Lens, Size, Style, ViewStyleExt,
    ui::{
        View, ViewEventExt,
        widgets::{column, text},
    },
};

use crate::{
    orchestra::{Orchestra, track::Id},
    pages::Theme,
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

    column((column((
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
    .style(Style::new().width(Size::Fill).height(Size::Fit)),))
    .style(
        Style::new()
            .bg_color(main_bg)
            .width(Size::Fill)
            .height(Size::Fill)
            .gap(28.)
            .padding_edges(Edges::lr(30.).top(20.).right(60.)),
    )
    .on_global_key_down(|_, k| {
        if k.is_escape() {
            Some(AlbumMsg::GotoLibrary)
        } else {
            None
        }
    })
}
