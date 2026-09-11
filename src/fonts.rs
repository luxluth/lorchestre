use mtk::{ui::View, windowing::Window};

pub const IOSEVKA_REGULAR_BYTES: &[u8] = include_bytes!("./assets/fonts/Iosevka-Regular.ttf");
pub const IOSEVKA_ITALIC_BYTES: &[u8] = include_bytes!("./assets/fonts/Iosevka-Italic.ttf");
pub const IOSEVKA_BOLD_BYTES: &[u8] = include_bytes!("./assets/fonts/Iosevka-Bold.ttf");
pub const IOSEVKA_BOLDITALIC_BYTES: &[u8] = include_bytes!("./assets/fonts/Iosevka-BoldItalic.ttf");

pub const INTER_VARIABLE_REGULAR_BYTES: &[u8] = include_bytes!("./assets/fonts/InterVariable.ttf");
pub const INTER_VARIABLE_ITALIC_BYTES: &[u8] =
    include_bytes!("./assets/fonts/InterVariable-Italic.ttf");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Font {
    Iosevka,
    InterVariable,
}

impl Font {
    pub fn name(&self) -> String {
        match self {
            Font::Iosevka => "Iosevka".to_string(),
            Font::InterVariable => "Inter Variable".to_string(),
        }
    }
}

impl Font {
    pub fn load<S: 'static, V: 'static>(&self, window: Window<S, V>) -> Window<S, V>
    where
        V: View<S>,
        V::Message: 'static + Send,
    {
        match self {
            Font::Iosevka => window
                .with_font_bytes(IOSEVKA_REGULAR_BYTES)
                .with_font_bytes(IOSEVKA_BOLD_BYTES)
                .with_font_bytes(IOSEVKA_ITALIC_BYTES)
                .with_font_bytes(IOSEVKA_BOLDITALIC_BYTES),
            Font::InterVariable => window
                .with_font_bytes(INTER_VARIABLE_REGULAR_BYTES)
                .with_font_bytes(INTER_VARIABLE_ITALIC_BYTES),
        }
    }
}
