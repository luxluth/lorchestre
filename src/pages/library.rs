use std::sync::Arc;

use arc_swap::ArcSwap;
use mtk::{
    AlignItems, Edges, Lens, ObjectFit, Size, Style, SvgData, TextStyle,
    text_property::Alignment,
    ui::{
        EventKind, View, ViewEventExt, ViewStyleExt,
        memoize::memoize,
        widgets::{ScrollAxis, async_image, column, row, scroll_view, svg, text},
    },
};

use crate::{
    icons::PLAY,
    orchestra::{
        Orchestra,
        track::{Id, Song},
    },
    pages::Theme,
};

#[derive(Lens, Clone, Debug, Default)]
pub struct LibraryState {
    pub hovered_song: Option<Id>,
}

#[derive(Clone, Debug)]
pub enum LibraryMsg {
    HoverSong(Id),
}

pub fn song_pill(
    song: &Song,
    orchestra: &Option<Arc<ArcSwap<Orchestra>>>,
    theme: Theme,
) -> impl View<LibraryState, Message = LibraryMsg> + use<> {
    let orch = orchestra.as_ref().unwrap();
    let guard = orch.load();
    let id = song.id;
    let artists: Vec<_> = song
        .artists
        .iter()
        .filter_map(|sid| guard.get_artist(&sid))
        .collect();

    let mut artist_names = String::new();

    for (i, artist) in artists.iter().enumerate() {
        artist_names.push_str(&artist.name);
        if i < artists.len() - 1 {
            artist_names.push_str(", ");
        }
    }

    row((
        svg(SvgData::from_str(PLAY).unwrap())
            .color(theme.fg())
            .fill(theme.fg())
            .stroke_width(0.)
            .fit(ObjectFit::Contain)
            .style(Style::new().width(Size::Fixed(18)).height(Size::Fixed(18))),
        text(&song.title).style(
            Style::new().set_text_style(
                TextStyle::new()
                    .font_size(14.)
                    .color(theme.fg())
                    .font_family("Inter Variable"),
            ),
        ),
        text(&artist_names).style(
            Style::new().set_text_style(
                TextStyle::new()
                    .font_size(14.)
                    .color(theme.fg().with_alpha(120))
                    .italic()
                    .font_family("Inter Variable"),
            ),
        ),
        text(&format!("{:?}", song.duration)).style(
            Style::new()
                .set_text_style(
                    TextStyle::new()
                        .font_size(14.)
                        .color(theme.fg().with_alpha(90))
                        .alignment(Alignment::End)
                        .italic()
                        .font_family("Iosevka"),
                )
                .flex_grow(1.),
        ),
    ))
    .style(
        Style::new()
            .border(1.0, theme.teal_gray())
            .corner_radius(4.0)
            .width(Size::Percent(1.0))
            .align_items(AlignItems::Center)
            // .justify_content(JustifyContent::SpaceBetween)
            .gap(14.0)
            .padding(7.0)
            .on_hover(|s| s.border(1.0, theme.teal_gray_accent())),
    )
    .on_event(EventKind::HoverIn, move |_| Some(LibraryMsg::HoverSong(id)))
}

fn hovered_song_card(
    state: &LibraryState,
    orchestra: &Option<Arc<ArcSwap<Orchestra>>>,
    _theme: Theme,
) -> impl View<LibraryState, Message = LibraryMsg> + use<> {
    let orch = orchestra.as_ref().unwrap();
    let guard = orch.load();

    column((state.hovered_song.as_ref().and_then(|id| {
        let song = guard.get_song(id).unwrap();
        let album_id = song.album?;
        let album = guard.get_album(&album_id)?;
        let cover_id = album.cover.as_ref()?;
        let cover = guard.get_cover(cover_id)?;

        Some(
            async_image(cover.get_path())
                .fit(ObjectFit::Cover)
                .style(Style::new().width(Size::Fill).aspect_ratio(1.0)),
        )
    }),))
    .style(Style::new().width(Size::Percent(0.4)))
}

pub fn render(
    state: &LibraryState,
    orchestra: Option<Arc<ArcSwap<Orchestra>>>,
    theme: Theme,
) -> impl View<LibraryState, Message = LibraryMsg> + use<> {
    let orch = orchestra.as_ref().unwrap();
    let guard = orch.load();

    let song_count = guard.collection.songs.len();

    let mut songs_items = Vec::new();

    for (_, song) in &guard.collection.songs {
        let orch = orchestra.clone();
        songs_items.push(memoize(song.clone(), move |song| {
            song_pill(song, &orch, theme)
        }));
    }

    column((
        column((
            text("Library").style(Style::new().apply(theme.heading())),
            text(if song_count > 1 {
                format!("{} songs", song_count)
            } else {
                format!("{} song", song_count)
            })
            .style(Style::new().apply(theme.subtitle())),
        )),
        row((
            scroll_view(column(songs_items).style(Style::new().width(Size::Percent(1.)).gap(9.)))
                .axis(ScrollAxis::Vertical)
                .style(
                    Style::new()
                        .width(Size::Percent(0.6))
                        .height(Size::Percent(1.0))
                        .padding_edges(Edges::tb(10.)),
                ),
            hovered_song_card(state, &orchestra, theme),
        ))
        .style(
            Style::new()
                .width(Size::Percent(1.0))
                .height(Size::Fill)
                .gap(30.),
        ),
    ))
    .style(
        Style::new()
            .width(Size::Fill)
            .height(Size::Fill)
            .gap(28.)
            .padding_edges(Edges::lr(45.).top(40.))
            .bg_color(theme.bg()),
    )
}
