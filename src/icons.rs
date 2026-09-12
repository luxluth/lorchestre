#![allow(dead_code)]

pub const FOLDER: &str = include_str!("./assets/icons/folder.svg");
pub const CHECK: &str = include_str!("./assets/icons/check.svg");
pub const PLAY: &str = include_str!("./assets/icons/play.svg");

pub const LIST_SORT_DESCENDING: &str = include_str!("./assets/icons/list-sort-descending.svg");
pub const LIST_SORT_ASCENDING: &str = include_str!("./assets/icons/list-sort-ascending.svg");

pub const DECIMALS_ARROW_RIGHT: &str = include_str!("./assets/icons/decimals-arrow-right.svg");

pub const CALENDAR: &str = include_str!("./assets/icons/calendar.svg");
pub const A_LARGE_SMALL: &str = include_str!("./assets/icons/a-large-small.svg");

pub const DISC_ALBUM: &str = include_str!("./assets/icons/disc-album.svg");
pub const MIC_VOCAL: &str = include_str!("./assets/icons/mic-vocal.svg");

pub const X: &str = include_str!("./assets/icons/x.svg");

pub const IMAGE_FRAME_39: &[u8] = include_bytes!("./assets/images/Frame 39.png");

#[macro_export]
macro_rules! svg_display {
    ($data:expr, $color:expr, $size:expr) => {
        container((svg($data).color($color).fit(ObjectFit::Contain).style(
            Style::new()
                .width(Size::Fixed($size))
                .height(Size::Fixed($size)),
        ),))
    };

    ($data:expr, $color:expr, $size:expr, $stroke_w:expr) => {
        container((svg($data)
            .color($color)
            .fit(ObjectFit::Contain)
            .stroke_width($stroke_w)
            .style(
                Style::new()
                    .width(Size::Fixed($size))
                    .height(Size::Fixed($size)),
            ),))
    };
}
