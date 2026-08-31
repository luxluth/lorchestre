use mtk::{Color, Style, clr, hsl, text_property::FontWeight};

pub mod landing;
pub mod library;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

macro_rules! colored {
    ($name:ident, $dark:expr, $light:expr) => {
        fn $name(&self) -> Color {
            match self {
                Theme::Dark => $dark,
                Theme::Light => $light,
            }
        }
    };
}

impl Theme {
    colored!(bg, clr!(0x181818FF), clr!(white));
    colored!(fg, clr!(white), clr!(black));
    colored!(border, clr!(0x222222FF), clr!(0xe8e8e8FF));
    colored!(border_accent, clr!(0x2a2a2aFF), clr!(0xf0f0f0FF));
    colored!(
        error,
        hsl!(0, 58.6 / 100.0, 49.2 / 100.0),
        hsl!(360, 79.0 / 100.0, 49.2 / 100.0)
    );
    colored!(teal_gray, clr!(0x2f2f2fff), clr!(0xddddddff));
    colored!(teal_gray_accent, clr!(0x222222ff), clr!(0x9a9a9aff));

    pub fn heading(&self) -> impl Fn(Style) -> Style + '_ {
        return move |s| {
            s.update_text_style(|t| {
                t.font_size = 48.0;
                t.color = self.fg();
                t.font_family = "Inter Variable".to_string();
                t.font_weight = FontWeight::BOLD;
            })
        };
    }

    pub fn subtitle(&self) -> impl Fn(Style) -> Style + '_ {
        return move |s| {
            s.update_text_style(|t| {
                t.font_size = 14.0;
                t.color = self.fg();
                t.font_family = "Inter Variable".to_string();
            })
            .opacity(0.5)
        };
    }
}

macro_rules! view_enum {
    ($enum_name:ident, $el_name:ident, { $($variant:ident($ty:ident)),* $(,)? }) => {
        pub enum $enum_name<$($ty),*> {
            $($variant($ty)),*
        }

        pub enum $el_name<$($ty),*> {
            $($variant($ty)),*
        }

        impl<State, Msg, $($ty),*> mtk::ui::View<State> for $enum_name<$($ty),*>
        where
            $($ty: mtk::ui::View<State, Message = Msg>),*
        {
            type Element = $el_name<$($ty::Element),*>;
            type Message = Msg;

            fn build(&self, ctx: &mut mtk::Context) -> Self::Element {
                match self {
                    $($enum_name::$variant(v) => $el_name::$variant(v.build(ctx))),*
                }
            }

            fn rebuild(&self, prev: &Self, ctx: &mut mtk::Context, element: &mut Self::Element) {
                match (self, prev, element) {
                    $(($enum_name::$variant(new_v), $enum_name::$variant(old_v), $el_name::$variant(el)) => {
                        new_v.rebuild(old_v, ctx, el);
                    })*
                    _ => {}
                }
            }

            fn teardown(&self, ctx: &mut mtk::Context, element: &mut Self::Element) {
                match (self, element) {
                    $(($enum_name::$variant(v), $el_name::$variant(el)) => v.teardown(ctx, el),)*
                    _ => {}
                }
            }

            fn get_node(&self, element: &Self::Element) -> mtk::Node {
                match (self, element) {
                    $(($enum_name::$variant(v), $el_name::$variant(el)) => v.get_node(el),)*
                    _ => mtk::Node::get_invalid(),
                }
            }

            fn handle_event(
                &self,
                element: &mut Self::Element,
                state: &State,
                event: mtk::ui::Event,
                ctx: &mut mtk::Context,
            ) -> (mtk::ui::event::EventResult, Option<Self::Message>) {
                match (self, element) {
                    $(($enum_name::$variant(v), $el_name::$variant(el)) => {
                        v.handle_event(el, state, event, ctx)
                    })*
                    _ => (mtk::ui::event::EventResult::Ignored, None),
                }
            }
        }
    };
}

view_enum!(PageView, PageElement, {
    Landing(A),
    Library(B),
});
