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
        tolerance_pos: text_input::State,
        tolerance_neg: text_input::State,
        sigma: text_input::State,
    },
    Float {
        button_save: button::State,
        button_delete: button::State,
        description: text_input::State,
        diameter_hole: text_input::State,
        diameter_pin: text_input::State,
        tolerance_hole_pos: text_input::State,
        tolerance_hole_neg: text_input::State,
        tolerance_pin_pos: text_input::State,
        tolerance_pin_neg: text_input::State,
        sigma: text_input::State,
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
    EditedLinearTolerancePos(String),
    EditedLinearToleranceNeg(String),
    EditedLinearSigma(String),
    // Float entry messages
    EditedFloatDiameterHole(String),
    EditedFloatDiameterPin(String),
    EditedFloatTolHolePos(String),
    EditedFloatTolHoleNeg(String),
    EditedFloatTolPinPos(String),
    EditedFloatTolPinNeg(String),
    EditedFloatSigma(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormValues {
    Linear { 
        description: String,
        dimension: String,
        tolerance_pos: String,
        tolerance_neg: String,
        sigma: String,
    },
    Float {
        description: String,
        diameter_hole: String,
        diameter_pin: String,
        tolerance_hole_pos: String,
        tolerance_hole_neg: String,
        tolerance_pin_pos: String,
        tolerance_pin_neg: String,
        sigma: String,
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
                        tolerance_pos: String::from(""),
                        tolerance_neg: String::from(""),
                        sigma: String::from(""),
                    }
                }
                Tolerance::Float(_) => {
                    FormValues::Float{
                        description: description,
                        diameter_hole: String::from(""),
                        diameter_pin: String::from(""),
                        tolerance_hole_pos: String::from(""),
                        tolerance_hole_neg: String::from(""),
                        tolerance_pin_pos: String::from(""),
                        tolerance_pin_neg: String::from(""),
                        sigma: String::from(""),
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
                                tolerance_pos: text_input::State::new(),
                                tolerance_neg: text_input::State::new(),
                                sigma: text_input::State::new(),
                            }
                        }
                    }
                    Tolerance::Float(_) => {
                        State::Editing {
                            form_tolentry: FormState::Float {
                                button_save: button::State::new(),
                                button_delete: button::State::new(),
                                description: text_input::State::focused(),
                                diameter_hole: text_input::State::new(),
                                diameter_pin: text_input::State::new(),
                                tolerance_hole_pos: text_input::State::new(),
                                tolerance_hole_neg: text_input::State::new(),
                                tolerance_pin_pos: text_input::State::new(),
                                tolerance_pin_neg: text_input::State::new(),
                                sigma: text_input::State::new(),
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
            Message::EditedLinearTolerancePos(input) => {
                match &mut self.input {
                    FormValues::Linear{tolerance_pos,..} => {
                        *tolerance_pos = NumericString::eval(
                            tolerance_pos,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedLinearToleranceNeg(input) => {
                match &mut self.input {
                    FormValues::Linear{tolerance_neg,..} => {
                        *tolerance_neg = NumericString::eval(
                            tolerance_neg,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedLinearSigma(input) => {
                match &mut self.input {
                    FormValues::Linear{sigma,..} => {
                        *sigma = NumericString::eval(
                            sigma,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatDiameterHole(input) => {
                match &mut self.input {
                    FormValues::Float{diameter_hole,..} => {
                        *diameter_hole = NumericString::eval(
                            diameter_hole,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatDiameterPin(input) => {
                match &mut self.input {
                    FormValues::Float{diameter_pin,..} => {
                        *diameter_pin = NumericString::eval(
                            diameter_pin,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatTolHolePos(input) => {
                match &mut self.input {
                    FormValues::Float{tolerance_hole_pos,..} => {
                        *tolerance_hole_pos = NumericString::eval(
                            tolerance_hole_pos,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatTolHoleNeg(input) => {
                match &mut self.input {
                    FormValues::Float{tolerance_hole_neg,..} => {
                        *tolerance_hole_neg = NumericString::eval(
                            tolerance_hole_neg,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatTolPinPos(input) => {
                match &mut self.input {
                    FormValues::Float{tolerance_pin_pos,..} => {
                        *tolerance_pin_pos = NumericString::eval(
                            tolerance_pin_pos,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatTolPinNeg(input) => {
                match &mut self.input {
                    FormValues::Float{tolerance_pin_neg,..} => {
                        *tolerance_pin_neg = NumericString::eval(
                            tolerance_pin_neg,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            Message::EditedFloatSigma(input) => {
                match &mut self.input {
                    FormValues::Float{sigma,..} => {
                        *sigma = NumericString::eval(
                            sigma,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
        }
    }

    pub fn  view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
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
                    },
                    Message::EntryActive,
                )
                .width(Length::Fill);

                let summary = Text::new( match self.valid {
                    true => {
                        match self.analysis_model {
                            Tolerance::Linear(dim) => {
                                if dim.distance.tol_neg == dim.distance.tol_pos {
                                    format!("{} +/- {}", dim.distance.dim, dim.distance.tol_pos)
                                } else {
                                    format!("{} +{}/-{}", dim.distance.dim, dim.distance.tol_pos, dim.distance.tol_neg)
                                }
                            }
                            Tolerance::Float(dim) => {
                                let hole = if dim.hole.tol_neg == dim.hole.tol_pos {
                                        format!("{} +/- {}", dim.hole.dim, dim.hole.tol_pos)
                                    } else {
                                        format!("{} +{}/-{}", dim.hole.dim, dim.hole.tol_pos, dim.hole.tol_neg)
                                    };
                                let pin = if dim.pin.tol_neg == dim.pin.tol_pos {
                                        format!("{} +/- {}", dim.pin.dim, dim.pin.tol_pos)
                                    } else {
                                        format!("{} +{}/-{}", dim.pin.dim, dim.pin.tol_pos, dim.pin.tol_neg)
                                    };
                                format!("Hole: {}\nPin: {}", hole, pin)
                            }
                        }
                    }
                    false => {
                        format!("Incomplete entry")
                    }
                });

                let row_contents = Row::new()
                    .padding(iss.padding(&iss.tol_entry_padding))    
                    .spacing(iss.spacing(&iss.tol_entry_spacing))
                    .align_items(Align::Center)
                    .push(checkbox)
                    .push(summary)
                    .push(
                        Button::new(
                            button_edit, 
                            Row::new()
                                .push(Text::new("Edit"))
                                .push(icons::edit())
                                .spacing(iss.spacing(&iss.tol_entry_button_spacing))
                        )
                        .on_press(Message::EntryEdit)
                        .padding(iss.padding(&iss.tol_entry_button_padding))
                        .style(iss.button(&iss.tol_entry_button))
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
                        tolerance_pos,
                        tolerance_neg,
                        sigma,
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
                        
                        let view_tolerance_pos = 
                            TextInput::new(
                                tolerance_pos,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Linear{tolerance_pos,..} => {
                                        tolerance_pos
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedLinearTolerancePos,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);

                        let view_tolerance_neg = 
                            TextInput::new(
                                tolerance_neg,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Linear{tolerance_neg,..} => {
                                        tolerance_neg
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedLinearToleranceNeg,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);

                        let view_sigma = 
                            TextInput::new(
                                sigma,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Linear{sigma,..} => {
                                        sigma
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedLinearSigma,
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

                        let row_tolerance_pos = Row::new()
                            .push(Text::new("+ Tolerance:"))
                            .push(view_tolerance_pos)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance_neg = Row::new()
                            .push(Text::new("- Tolerance:"))
                            .push(view_tolerance_neg)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_sigma = Row::new()
                            .push(Text::new("Sigma:"))
                            .push(view_sigma)
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
                            .push(row_tolerance_pos)
                            .push(row_tolerance_neg)
                            .push(row_sigma)
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
                        diameter_hole,
                        diameter_pin,
                        tolerance_hole_pos,
                        tolerance_hole_neg,
                        tolerance_pin_pos,
                        tolerance_pin_neg,
                        sigma,
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

                        let view_diameter_hole = 
                            TextInput::new(
                                diameter_hole,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{diameter_hole,..} => {
                                        diameter_hole
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatDiameterHole,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);

                        let view_diameter_pin = 
                            TextInput::new(
                                diameter_pin,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{diameter_pin,..} => {
                                        diameter_pin
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatDiameterPin,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_tolerance_hole_pos = 
                            TextInput::new(
                                tolerance_hole_pos,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{tolerance_hole_pos,..} => {
                                        tolerance_hole_pos
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatTolHolePos,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_tolerance_hole_neg = 
                            TextInput::new(
                                tolerance_hole_neg,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{tolerance_hole_neg,..} => {
                                        tolerance_hole_neg
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatTolHoleNeg,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);

                        let view_tolerance_pin_pos = 
                            TextInput::new(
                                tolerance_pin_pos,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{tolerance_pin_pos,..} => {
                                        tolerance_pin_pos
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatTolPinPos,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_tolerance_pin_neg = 
                            TextInput::new(
                                tolerance_pin_neg,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{tolerance_pin_neg,..} => {
                                        tolerance_pin_neg
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatTolPinNeg,
                            )
                            .on_submit(Message::EntryFinishEditing)
                            .padding(10);
                        
                        let view_sigma = 
                            TextInput::new(
                                sigma,
                                "Enter a value",
                                match &self.input {
                                    FormValues::Float{sigma,..} => {
                                        sigma
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                Message::EditedFloatSigma,
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

                        let row_diameter_hole = Row::new()
                            .push(Text::new("Hole Diameter:"))
                            .push(view_diameter_hole)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_diameter_pin = Row::new()
                            .push(Text::new("Pin Diameter:"))
                            .push(view_diameter_pin)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance_hole_pos = Row::new()
                            .push(Text::new("+ Hole Tolerance:"))
                            .push(view_tolerance_hole_pos)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance_hole_neg = Row::new()
                            .push(Text::new("- Hole Tolerance:"))
                            .push(view_tolerance_hole_neg)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance_pin_pos = Row::new()
                            .push(Text::new("+ Pin Tolerance:"))
                            .push(view_tolerance_pin_pos)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance_pin_neg = Row::new()
                            .push(Text::new("- Pin Tolerance:"))
                            .push(view_tolerance_pin_neg)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_sigma = Row::new()
                            .push(Text::new("Sigma:"))
                            .push(view_sigma)
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
                            .push(Text::new("Hole"))
                            .push(row_diameter_hole)
                            .push(row_tolerance_hole_pos)
                            .push(row_tolerance_hole_neg)
                            .push(Text::new("Pin"))
                            .push(row_diameter_pin)
                            .push(row_tolerance_pin_pos)
                            .push(row_tolerance_pin_neg)
                            .push(row_sigma)
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
                    if input == "" || input == "." {
                        input.to_string()
                    } else {
                        old.to_string()
                    }
                }
            }
            false => {
                if match criteria {
                    NumericString::Number => input == "" || input == "-" || input == ".",
                    NumericString::Negative => input == "" || input == "-" || input == ".",
                    NumericString::Positive => input == "" || input == ".",
                } {
                    input.to_string()
                } else {
                    old.to_string()
                }
            }
        }       
    }
}