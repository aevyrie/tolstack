use crate::analysis::structures::*;
use crate::ui::style;
use iced::{button, Align, Button, Element, Length, Row, Text};
use serde_derive::*;

#[derive(Debug, Clone)]
pub enum Message {
    FilterChanged(Filter),
}

#[derive(Debug, Default, Clone)]
pub struct ToleranceFilter {
    pub filter_value: Filter,
    all_button: button::State,
    linear_button: button::State,
    float_button: button::State,
    compound_button: button::State,
}
impl ToleranceFilter {
    pub fn update(&mut self, message: Message) {
        let ToleranceFilter {
            filter_value,
            all_button,
            linear_button,
            float_button,
            compound_button,
        } = self;
        match message {
            Message::FilterChanged(filter) => {
                *filter_value = filter;
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let ToleranceFilter {
            filter_value,
            all_button,
            linear_button,
            float_button,
            compound_button,
        } = self;

        let filter_button = |state, label, filter, current_filter| {
            let label = Text::new(label).size(16);
            let button = Button::new(state, label).style(style::Button::Filter {
                selected: filter == current_filter,
            });

            button.on_press(Message::FilterChanged(filter)).padding(8)
        };

        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(10)
                    .push(filter_button(all_button, "All", Filter::All, *filter_value))
                    .push(filter_button(
                        linear_button,
                        "Linear",
                        Filter::Some(Tolerance::Linear(LinearTL::default())),
                        *filter_value,
                    ))
                    .push(filter_button(
                        float_button,
                        "Float",
                        Filter::Some(Tolerance::Float(FloatTL::default())),
                        *filter_value,
                    )),
            )
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Some(Tolerance),
}
impl Filter {
    pub fn matches(&self, tol: Tolerance) -> bool {
        match self {
            Filter::All => true,
            Filter::Some(tol_self) => {
                std::mem::discriminant(tol_self) == std::mem::discriminant(&tol)
            }
        }
    }
}
impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}
