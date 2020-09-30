use crate::analysis::structures::*;
use crate::ui::style;
use iced::{
    button, text_input, Align, Button, Column, Command, Element, HorizontalAlignment, Length, Row,
    Text, TextInput,
};

#[derive(Debug, Clone)]
pub enum Message {
    TolTypeChanged(Tolerance),
    TolNameChanged(String),
    CreateTol(String, Tolerance),
}

#[derive(Debug, Default, Clone)]
pub struct NewToleranceEntry {
    tolerance_type: Tolerance,
    tolerance_text_value: String,
    state_linear_button: button::State,
    state_float_button: button::State,
    state_compound_button: button::State,
    state_tolerance_text: text_input::State,
}
impl NewToleranceEntry {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TolTypeChanged(value) => {
                self.tolerance_type = value;
            }
            Message::TolNameChanged(value) => {
                self.tolerance_text_value = value;
            }
            Message::CreateTol(_, _) => {
                self.tolerance_text_value.clear();
            }
        }
        Command::none()
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        let NewToleranceEntry {
            tolerance_type,
            tolerance_text_value,
            state_linear_button,
            state_float_button,
            state_compound_button: _,
            state_tolerance_text,
        } = self;

        let tolerance_label = Text::new("Add Tolerance")
            .width(Length::Fill)
            .size(24)
            .horizontal_alignment(HorizontalAlignment::Left);
        let tolerance_text = TextInput::new(
            state_tolerance_text,
            "Tolerance name, press enter to add.",
            tolerance_text_value,
            Message::TolNameChanged,
        )
        .padding(10)
        .on_submit(Message::CreateTol(
            tolerance_text_value.clone(),
            tolerance_type.clone(),
        ));

        let button = |state, label, tolerance: Tolerance, current_tol: Tolerance| {
            let label = Text::new(label).size(18);
            let active = tolerance == current_tol;
            let button = Button::new(state, label).style(iss.toggle_button(
                active,
                &iss.button_active,
                &iss.button_inactive,
            ));
            button
                .on_press(Message::TolTypeChanged(tolerance))
                .padding(8)
        };

        Row::new()
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .spacing(20)
                            .align_items(Align::Center)
                            .push(tolerance_label)
                            .push(
                                Row::new()
                                    .width(Length::Shrink)
                                    .spacing(10)
                                    .push(button(
                                        state_linear_button,
                                        "Linear",
                                        Tolerance::Linear(LinearTL::default()),
                                        self.tolerance_type,
                                    ))
                                    .push(button(
                                        state_float_button,
                                        "Float",
                                        Tolerance::Float(FloatTL::default()),
                                        self.tolerance_type,
                                    )),
                            ),
                    )
                    .push(tolerance_text)
                    .spacing(10),
            )
            .into()
    }
}
