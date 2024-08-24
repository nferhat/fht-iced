use iced::widget::button::{Catalog, Status, Style};
use iced::Color;

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, item: &Self::Class<'_>, status: Status) -> Style {
        item(self, status)
    }
}

/// Create a new default button style.
pub fn default(theme: &Theme, status: Status) -> Style {
    let base = Style {
        background: Some(theme.background.secondary.scale_alpha(0.8).into()),
        ..Default::default()
    };
    match status {
        Status::Disabled => Style {
            background: Some(theme.background.secondary.scale_alpha(0.45).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(theme.background.tertiary.into()),
            ..base
        },
        Status::Hovered => Style {
            background: Some(theme.background.secondary.into()),
            ..base
        },
        _ => base,
    }
}

/// Create a new default button style.
pub fn tertiary(theme: &Theme, status: Status) -> Style {
    let base = Style {
        background: Some(theme.background.tertiary.scale_alpha(0.6).into()),
        ..Default::default()
    };
    match status {
        Status::Disabled => Style {
            background: Some(theme.background.tertiary.scale_alpha(0.45).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(theme.background.tertiary.into()),
            ..base
        },
        Status::Hovered => Style {
            background: Some(theme.background.tertiary.scale_alpha(0.8).into()),
            ..base
        },
        _ => base,
    }
}

/// Create a new default button style.
pub fn transparent(theme: &Theme, status: Status) -> Style {
    let base = Style {
        background: Some(Color::TRANSPARENT.into()),
        ..Default::default()
    };
    match status {
        Status::Disabled => Style {
            background: Some(theme.background.secondary.scale_alpha(0.45).into()),
            ..base
        },
        Status::Pressed => Style {
            background: Some(theme.background.tertiary.into()),
            ..base
        },
        Status::Hovered => Style {
            background: Some(theme.background.secondary.into()),
            ..base
        },
        _ => base,
    }
}
