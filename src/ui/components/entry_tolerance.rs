use serde_derive::*;
use iced::{
    button, text_input, Align, Button, Container, Element, HorizontalAlignment, Length, Checkbox,
    Row, Column, Text, TextInput,
};
use crate::ui::{ style, icons };
use crate::analysis::{
    structures::*,
};

#[derive(Debug, Clone)]
pub enum State {
    Idle {
        button_edit: button::State,
    },
    Editing {
        form_tolentry: FormState,
    },
}
impl Default for State {
    fn default() -> Self {
        State::Idle {
            button_edit: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FormState {
    Linear {
        button_save: button::State,
        button_delete: button::State,
        description: text_input::State,
        dimension: text_input::State,
        tolerance: text_input::State,
    },
    Float {
        button_save: button::State,
        button_delete: button::State,
        description: text_input::State,
        tolerance_hole: text_input::State,
        tolerance_pin: text_input::State,
    },
    Compound {},
}

#[derive(Debug, Clone)]
pub enum Message {
    // Entry messages
    EntryActive(bool),
    EntryEdit,
    EntryDelete,
    EntryFinishEditing,
    // Shared Field messages
    EditedDescription(String),
    // Linear entry messages
    EditedLinearDimension(String),
    EditedLinearTolerance(String),
    // Float entry messages
    EditedFloatTolHole(String),
    EditedFloatTolPin(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormValues {
    Linear { 
        description: String,
        dimension: String,
        tolerance: String,
    },
    Float {
        description: String,
        tolerance_hole: String,
        tolerance_pin: String,
    },
    Compound {
        description: String,
        tolerance_hole_1: String,
        tolerance_pin_1: String,
        tolerance_hole_2: String,
        tolerance_pin_2: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToleranceEntry {
    pub input: FormValues,
    pub analysis_model: Tolerance,
    pub active: bool,
    pub valid: bool,

    #[serde(skip)]
    state: State,
}
impl ToleranceEntry {
    pub fn new(description: String, tolerance: Tolerance) -> Self {
        ToleranceEntry {
            input: match tolerance {
                Tolerance::Linear(_) => {
                    FormValues::Linear{
                        description: description,
                        dimension: String::from(""),
                        tolerance: String::from(""),
                    }
                }
                Tolerance::Float(_) => {
                    FormValues::Float{
                        description: description,
                        tolerance_hole: String::from(""),
                        tolerance_pin: String::from(""),
                    }
                }
                Tolerance::Compound(_) => {
                    FormValues::Compound{
                        description: description,
                        tolerance_hole_1: String::from(""),
                        tolerance_pin_1: String::from(""),
                        tolerance_hole_2: String::from(""),
                        tolerance_pin_2: String::from(""),
                    }
                }
            },
            analysis_model: tolerance,
            active: false,
            valid: false,
            state: State::Idle {
                button_edit: button::State::new(),
            },
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EntryActive(is_active) => {
                if self.valid { 
                    self.active = is_active 
                } else {
                    self.active = false;
                }
            }
            Message::EntryEdit => {
                self.state = match self.analysis_model {
                    Tolerance::Linear(_) => {
                        State::Editing {
                            form_tolentry: FormState::Linear {
                                button_save: button::State::new(),
                                button_delete: button::State::new(),
                                description: text_input::State::focused(),
                                dimension: text_input::State::new(),
                                tolerance: text_input::State::new(),
                            }
                        }
                    }
                    Tolerance::Float(_) => {
                        State::Editing {
                            form_tolentry: FormState::Float {
                                button_save: button::State::new(),
                                button_delete: button::State::new(),
                                description: text_input::State::focused(),
                                tolerance_hole: text_input::State::new(),
                                tolerance_pin: text_input::State::new(),
                            }
                        }
                    }
                    Tolerance::Compound(_) => {
                        State::Editing {
                            form_tolentry: FormState::Compound {
                            }
                        }
                    }
                };
            }
            Message::EntryFinishEditing => {
                if match &self.input {
                    FormValues::Linear{description,..} => {
                        !description.is_empty()
                    },
                    FormValues::Float{description,..} => {
                        !description.is_empty()
                    },
                    FormValues::Compound{description,..} => {
                        !description.is_empty()
                    },
                } {
                    self.state = State::Idle {
                        button_edit: button::State::new(),
                    }
                }
            }
            Message::EntryDelete => {}
            Message::EditedDescription(input) => {
                match &mut self.input {
                    FormValues::Linear{description,..} => {
                        *description = input
                    },
                    FormValues::Float{description,..} => {
                        *description = input
                    },
                    FormValues::Compound{description,..} => {
                        *description = input
                    },
                };
            }
            Message::EditedLinearDimension(input) => {
                match &mut self.input {
                    FormValues::Linear{dimension,..} => {
                        *dimension = NumericString::eval(
                            dimension,
                            &input,
                            NumericString::Number
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedLinearTolerance(input) => {
                match &mut self.input {
                    FormValues::Linear{tolerance,..} => {
                        *tolerance = NumericString::eval(
                            tolerance,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatTolHole(input) => {
                match &mut self.input {
                    FormValues::Float{tolerance_hole,..} => {
                        *tolerance_hole = NumericString::eval(
                            tolerance_hole,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatTolPin(input) => {
                match &mut self.input {
                    FormValues::Float{tolerance_pin,..} => {
                        *tolerance_pin = NumericString::eval(
                            tolerance_pin,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
        }
    }

    pub fn  view(&mut self) -> Element<Message> {
        match &mut self.state {
            State::Idle { button_edit } => {
                let checkbox = Checkbox::new(
                    self.active,
                    match &self.input {
                        FormValues::Linear{description,..} => {
                            description
                        },
                        FormValues::Float{description,..} => {
                            description
                        },
                        FormValues::Compound{description,..} => {
                            description
                        },
                    },
                    Message::EntryActive,
                )
                .width(Length::Fill);

                let row_contents = Row::new()
                    .padding(10)    
                    .spacing(20)
                    .align_items(Align::Center)
                    .push( checkbox )
                    .push(
                        Button::new(button_edit, icons::edit())
                            .on_press(Message::EntryEdit)
                            .padding(10)
                            .style(style::Button::Icon),
                    );

                Container::new(row_contents)
                    .style(style::Container::Entry)
                    .into()
            }
            State::Editing { form_tolentry } => {
                match form_tolentry {
                    FormState::Linear {
                        button_save,
                        button_delete,
                        description,
                        dimension,
                        tolerance,
                    } => {
                        
                        let view_button_save =
                            Button::new(
                                button_save,
                                Row::new()
                                    .spacing(10)
                                    .push(icons::check())
                                    .push(Text::new("Save")),
                            )
                            .on_press(Message::EntryFinishEditing)
                            .padding(10)
                            .style(style::Button::Constructive);
                        
                        let view_button_delete =
                            Button::new(
                                button_delete,
                                Row::new()
                                    .spacing(10)
                                    .push(icons::delete())
                                    .push(Text::new("Delete")),
                            )
                            .on_press(Message::EntryDelete)
                            .padding(10)
                            .style(style::Button::Destructive);

                        let view_description = 
                            TextInput::new(
                                description,
                                "Enter a description",
                                match &self.input {
                                    FormValues::Linear{description,..} => {
                                        description
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedDescription,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_dimension = 
                            TextInput::new(
                                dimension,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Linear{dimension,..} => {
                                        dimension
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedLinearDimension,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_tolerance = 
                            TextInput::new(
                                tolerance,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Linear{tolerance,..} => {
                                        tolerance
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedLinearTolerance,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);

                        let row_header = Row::new()
                            .push(Text::new("Editing Linear Tolerance")
                                .size(24)
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Left)
                            )
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_description = Row::new()
                            .push(Text::new("Description:"))
                            .push(view_description)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_dimension = Row::new()
                            .push(Text::new("Dimension:"))
                            .push(view_dimension)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance = Row::new()
                            .push(Text::new("Tolerance:"))
                            .push(view_tolerance)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_buttons = Row::new()
                            .push(view_button_delete)
                            .push(view_button_save)
                            .spacing(20)
                            .align_items(Align::Center);
        
                        let entry_contents = Column::new()
                            .push(row_header)
                            .push(Row::new().height(Length::Units(5)))
                            .push(row_description)
                            .push(row_dimension)
                            .push(row_tolerance)
                            .push(Row::new().height(Length::Units(5)))
                            .push(row_buttons)
                            .spacing(10)
                            .padding(20);
                        
                        Container::new(entry_contents)
                            .style(style::Container::Entry)
                            .into()
                    },
                    FormState::Float {
                        button_save,
                        button_delete,
                        description,
                        tolerance_hole,
                        tolerance_pin,
                    } => {
                        
                        let view_button_save =
                            Button::new(
                                button_save,
                                Row::new()
                                    .spacing(10)
                                    .push(icons::check())
                                    .push(Text::new("Save")),
                            )
                            .on_press(Message::EntryFinishEditing)
                            .padding(10)
                            .style(style::Button::Constructive);
                        
                        let view_button_delete =
                            Button::new(
                                button_delete,
                                Row::new()
                                    .spacing(10)
                                    .push(icons::delete())
                                    .push(Text::new("Delete")),
                            )
                            .on_press(Message::EntryDelete)
                            .padding(10)
                            .style(style::Button::Destructive);

                        let view_description = 
                            TextInput::new(
                                description,
                                "Enter a description",
                                match &self.input {
                                    FormValues::Float{description,..} => {
                                        description
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedDescription,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_tolerance_hole = 
                            TextInput::new(
                                tolerance_hole,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{tolerance_hole,..} => {
                                        tolerance_hole
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatTolHole,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_tolerance_pin = 
                            TextInput::new(
                                tolerance_pin,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{tolerance_pin,..} => {
                                        tolerance_pin
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatTolPin,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);

                        let row_header = Row::new()
                            .push(Text::new("Editing Float Tolerance")
                                .size(24)
                                .width(Length::Fill)
                                .horizontal_alignment(HorizontalAlignment::Left)
                            )
                            .spacing(20)
                            .align_items(Align::Center);
                            
                        let row_description = Row::new()
                            .push(Text::new("Description:"))
                            .push(view_description)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_dimension = Row::new()
                            .push(Text::new("Hole Tolerance:"))
                            .push(view_tolerance_hole)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance = Row::new()
                            .push(Text::new("Pin Tolerance:"))
                            .push(view_tolerance_pin)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_buttons = Row::new()
                            .push(view_button_delete)
                            .push(view_button_save)
                            .spacing(20)
                            .align_items(Align::Center);
        
                        let entry_contents = Column::new()
                            .push(row_header)
                            .push(Row::new().height(Length::Units(5)))
                            .push(row_description)
                            .push(row_dimension)
                            .push(row_tolerance)
                            .push(Row::new().height(Length::Units(5)))
                            .push(row_buttons)
                            .spacing(10)
                            .padding(20);
                        
                        Container::new(entry_contents)
                            .style(style::Container::Entry)
                            .into()
                    },
                    FormState::Compound {} => {
                        Container::new(Row::new()).into()
                    },
                }
            }
        }
    }
}

enum NumericString {
    Number,
    Positive,
    Negative,
}
impl NumericString {
    pub fn eval(old: &str, input: &str, criteria: Self) -> String {
        match input.parse::<f64>().is_ok() {
            true => {
                let numeric_input = input.parse::<f64>().unwrap();
                if match criteria {
                    NumericString::Number => true,
                    NumericString::Positive => numeric_input >= 0.0,
                    NumericString::Negative => numeric_input < 0.0,
                } {
                    input.to_string()
                } else {
                    old.to_string()
                }
            }
            false => {
                if match criteria {
                    NumericString::Number => input == "" || input == "-",
                    NumericString::Negative => input == "" || input == "-",
                    NumericString::Positive => false,
                } {
                    input.to_string()
                } else {
                    old.to_string()
                }
            }
        }       
    }
}