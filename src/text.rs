use iced::widget::text::{Catalog, Style};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, item: &Self::Class<'_>) -> iced::widget::text::Style {
        item(self)
    }
}

/// Create a new text field with the accent text color.
pub const fn accent(theme: &Theme) -> Style {
    Style {
        color: Some(theme.accent),
    }
}

/// Create a new text field with the error text color.
pub const fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.error),
    }
}

/// Create a new text field with the info text color.
pub const fn info(theme: &Theme) -> Style {
    Style {
        color: Some(theme.info),
    }
}

/// Create a new text field with the primary text color.
pub const fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.text.primary),
    }
}

/// Create a new text field with the secondary text color.
pub const fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.text.secondary),
    }
}

/// Create a new text field with the tertiary text color.
pub const fn tertiary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.text.tertiary),
    }
}
