use std::sync::Arc;

use arc_swap::ArcSwap;
use mtk::{
    AlignItems, Edges, JustifyContent, Lens, ObjectFit, Size, Style, SvgData, TextStyle,
    text_property::{Alignment, FontWeight},
    ui::{
        EventKind, View, ViewEventExt, ViewStyleExt,
        widgets::{async_image, column, container, row, svg, text, virtual_list},
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
    hsid: Option<Id>,
    orchestra: &Option<Arc<ArcSwap<Orchestra>>>,
    theme: Theme,
    index: usize,
) -> impl View<LibraryState, Message = LibraryMsg> + use<> {
    let orch = orchestra.as_ref().unwrap();
    let guard = orch.load();
    let id = song.id;
    let mut is_hovered: bool = false;
    if let Some(hsid) = hsid {
        is_hovered = hsid == id;
    }

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

    let leading = container((
        is_hovered.then_some(
            svg(SvgData::from_str(PLAY).unwrap())
                .color(theme.fg())
                .fill(theme.fg())
                .stroke_width(0.)
                .fit(ObjectFit::Contain)
                .style(Style::new().width(Size::Fixed(18)).height(Size::Fixed(18))),
        ),
        (!is_hovered).then_some(
            text(&format!("{}", index + 1)).style(
                Style::new().set_text_style(
                    TextStyle::new()
                        .font_size(14.)
                        .color(theme.fg().with_alpha(180))
                        .font_weight(FontWeight::BOLD)
                        .font_family("Iosevka"),
                ),
            ),
        ),
    ))
    .style(
        Style::new()
            .width(Size::Fixed(28))
            .height(Size::Fixed(18))
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center),
    );

    row((
        leading,
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
                    .color(theme.fg().with_alpha(180))
                    .italic()
                    .font_family("Inter Variable"),
            ),
        ),
        text(&format!("{:?}", song.duration)).style(
            Style::new()
                .set_text_style(
                    TextStyle::new()
                        .font_size(14.)
                        .color(theme.fg().with_alpha(180))
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
            async_image(cover.get_path()).fit(ObjectFit::Cover).style(
                Style::new()
                    .width(Size::Fill)
                    .aspect_ratio(1.0)
                    .corner_radius(8.),
            ),
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

    let mut songs: Vec<Song> = guard.collection.songs.values().cloned().collect();
    songs.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

    let orch_clone = orchestra.clone();
    let hsid = state.hovered_song;
    const ITEM_HEIGHT: f32 = 45.0;

    let songs_list = virtual_list(songs, ITEM_HEIGHT, move |i, song| {
        container((song_pill(song, hsid, &orch_clone, theme, i),)).style(
            Style::new()
                .width(Size::Percent(1.0))
                .height(Size::Fixed(ITEM_HEIGHT as u32))
                .min_height(ITEM_HEIGHT)
                .max_height(ITEM_HEIGHT)
                .flex_shrink(0.0)
                .padding_edges(Edges::tb(4.5)),
        )
    })
    .buffer(5)
    .style(
        Style::new()
            .width(Size::Percent(0.6))
            .height(Size::Percent(1.0)),
    );

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
            songs_list,
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
