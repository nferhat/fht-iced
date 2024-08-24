use iced::widget::text_input::{Catalog, Status, Style};
use iced::{Background, Border};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
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
        selection: theme.background.secondary,
        background: Background::Color(theme.background.primary),
        placeholder: theme.text.tertiary,
    }
}
