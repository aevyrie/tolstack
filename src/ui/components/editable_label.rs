use serde_derive::*;
use iced::{
    button, text_input, Align, Button, Container, Element, HorizontalAlignment, Length,
    Row, Text, TextInput,
};
use crate::ui::{ style, icons };

#[derive(Debug, Clone)]
pub enum State {
    Idle {
        edit_button: button::State,
    },
    Editing {
        text_input: text_input::State,
    },
}
impl Default for State {
    fn default() -> Self {
        State::Idle {
            edit_button: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit,
    TextEdited(String),
    FinishEditing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditableLabel {
    pub text: String,
    pub placeholder: String,
    #[serde(skip)]
    state: State,
}
impl EditableLabel {
    pub fn new<T: Into<String>,U: Into<String>>(text: T, placeholder: U) -> Self {
        EditableLabel {
            text: text.into(),
            placeholder: placeholder.into(),
            state: State::Idle {
                edit_button: button::State::new(),
            },
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit => {
                self.state = State::Editing {
                    text_input: text_input::State::focused(),
                };
            }
            Message::TextEdited(new_text) => {
                self.text = new_text;
            }
            Message::FinishEditing => {
                if !self.text.is_empty() {
                    self.state = State::Idle {
                        edit_button: button::State::new(),
                    }
                }
            }
        }
    }

    pub fn view(&mut self, stylesheet: &style::StyleSheet) -> Element<Message> {
        match &mut self.state {
            State::Idle { edit_button } => {
                let label = Text::new(self.text.clone())
                    .width(Length::Shrink)
                    .size(stylesheet.text_size(&stylesheet.text_size_editable_label_label))
                    .color(stylesheet.color(&stylesheet.color_editable_label_label))
                    .horizontal_alignment(HorizontalAlignment::Left);

                let row_contents = Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(label)
                    .push(
                        Button::new(edit_button, icons::edit())
                            .on_press(Message::Edit)
                            .padding(10)
                            .style(style::Button::Icon),
                    );

                Container::new(row_contents)
                    .into()
            }
            State::Editing {
                text_input,
            } => {
                let text_input = TextInput::new(
                    text_input,
                    &self.placeholder,
                    &self.text,
                    Message::TextEdited,
                )
                .on_submit(Message::FinishEditing)
                .padding(10)
                .width(Length::Fill);   

                let row_contents = Row::new()
                    .padding(10)
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(text_input);
                Container::new(row_contents)
                    .into()
            }
        }
    }
}
impl Default for EditableLabel {
    fn default() -> Self {
        EditableLabel {
            text: String::from(""),
            placeholder: String::from("Enter some text..."),
            state: State::default(),
        }
    }
}

