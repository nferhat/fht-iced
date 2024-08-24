use iced::widget::container::{Catalog, Style};
use iced::{Border, Color};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, item: &Self::Class<'_>) -> Style {
        item(self)
    }
}

/// Create a new transparent container.
pub fn transparent(_: &Theme) -> Style {
    Default::default()
}

/// Create a new container with the primary background color.
pub fn primary(theme: &Theme) -> Style {
    Style::default().background(theme.background.primary)
}

/// Create a new container with the secondary background color.
pub fn secondary(theme: &Theme) -> Style {
    Style::default().background(theme.background.secondary)
}

/// Create a new container with the tertiary background color.
pub fn tertiary(theme: &Theme) -> Style {
    Style::default().background(theme.background.tertiary)
}

/// Create a new container with a transparent background and a border.
pub fn bordered(theme: &Theme) -> Style {
    Style::default()
        .background(Color::TRANSPARENT)
        .border(Border {
            color: theme.separator,
            width: 2f32,
            radius: 0.into(),
        })
}
