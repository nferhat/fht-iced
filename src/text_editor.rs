use iced::widget::text_editor::{Catalog, Status, Style};
use iced::{Background, Border};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(transparent)
    }

    fn style(&self, item: &Self::Class<'_>, status: Status) -> Style {
        item(self, status)
    }
}

fn primary(theme: &Theme, status: Status) -> Style {
    Style {
        icon: theme.accent,
        value: match status {
            Status::Disabled => theme.text.secondary,
            _ => theme.text.primary,
        },
        border: Border::default(),
        selection: theme.accent,
        background: Background::Color(theme.background.primary),
        placeholder: theme.text.tertiary,
    }
}

fn transparent(theme: &Theme, status: Status) -> Style {
    Style {
        icon: theme.accent,
        value: match status {
            Status::Disabled => theme.text.secondary,
            _ => theme.text.primary,
        },
        border: Border::default(),
        selection: theme.background.tertiary,
        background: Background::Color(iced::Color::TRANSPARENT),
        placeholder: theme.text.tertiary,
    }
}
