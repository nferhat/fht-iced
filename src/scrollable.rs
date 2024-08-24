use iced::widget::container;
use iced::widget::scrollable::{Catalog, Rail, Scroller, Status, Style};
use iced::Border;

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
    let rail = Rail {
        background: Some(theme.background.secondary.into()),
        border: Border::default().rounded(2),
        scroller: Scroller {
            color: theme.background.tertiary,
            border: Border::default().rounded(2),
        },
    };

    match status {
        Status::Active | Status::Hovered { .. } => Style {
            container: container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
        },
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
        } => {
            let dragged_rail = Rail {
                scroller: Scroller {
                    color: theme.accent,
                    border: Border::default().rounded(2),
                },
                ..rail
            };

            Style {
                container: container::Style::default(),
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_rail
                } else {
                    rail
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_rail
                } else {
                    rail
                },
                gap: None,
            }
        }
    }
}
