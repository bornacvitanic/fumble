use ratatui::buffer::Buffer;
use ratatui::crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::widgets::{Widget};
use crate::cli::tui::traits::{HandleInput, DisplayName, KeyBindings, IsActive};
use crate::cli::tui::widgets::bandwidth_widget::BandwidthWidget;
use crate::cli::tui::widgets::delay_widget::DelayWidget;
use crate::cli::tui::widgets::drop_widget::DropWidget;
use crate::cli::tui::widgets::duplicate_widget::DuplicateWidget;
use crate::cli::tui::widgets::reorder_widget::ReorderWidget;
use crate::cli::tui::widgets::tamper_widget::TamperWidget;
use crate::cli::tui::widgets::throttle_widget::ThrottleWidget;

pub enum CustomWidget<'a> {
    Drop(DropWidget<'a>),
    Delay(DelayWidget<'a>),
    Throttle(ThrottleWidget<'a>),
    Reorder(ReorderWidget<'a>),
    Tamper(TamperWidget<'a>),
    Duplicate(DuplicateWidget<'a>),
    Bandwidth(BandwidthWidget<'a>),
}

macro_rules! impl_widget_traits_for_enum {
    ($enum_name:ident, $($variant:ident),+) => {
        impl<'a> Widget for &mut $enum_name<'a> {
            fn render(mut self, area: Rect, buf: &mut Buffer) {
                match &mut self {
                    $( $enum_name::$variant(ref mut widget) => widget.render(area, buf), )+
                }
            }
        }

        impl<'a> HandleInput for $enum_name<'a> {
            fn handle_input(&mut self, key: KeyEvent) -> bool {
                match self {
                    $( $enum_name::$variant(widget) => widget.handle_input(key), )+
                }
            }
        }

        impl<'a> DisplayName for $enum_name<'a> {
            fn name(&self) -> &str {
                match self {
                    $( $enum_name::$variant(ref widget) => widget.name(), )+
                }
            }
        }

        impl<'a> KeyBindings for $enum_name<'a> {
            fn key_bindings(&self) -> String {
                match self {
                    $( $enum_name::$variant(ref widget) => widget.key_bindings(), )+
                }
            }
        }

        impl<'a> IsActive for $enum_name<'a> {
            fn is_active(&self) -> bool {
                match self {
                    $( $enum_name::$variant(ref widget) => widget.is_active(), )+
                }
            }

            fn set_active(&mut self, state: bool) {
                match self {
                    $( $enum_name::$variant(ref mut widget) => widget.set_active(state), )+
                }
            }
        }
    };
}

impl_widget_traits_for_enum!(CustomWidget, Drop, Delay, Throttle, Reorder, Tamper, Duplicate, Bandwidth);