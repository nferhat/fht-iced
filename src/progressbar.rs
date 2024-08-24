use iced::widget::progress_bar::{Catalog, Style};
use iced::Border;

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = Box<dyn Fn(&Theme) -> Style + 'a>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, item: &Self::Class<'_>) -> Style {
        item(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    Style {
        bar: theme.accent.into(),
        border: Border::default(),
        background: theme.background.tertiary.into(),
    }
}
