use iced::{
    scrollable, Align, Container, Element, HorizontalAlignment, Length, Row, Column, Text,
    Scrollable, 
};
use crate::ui::{ style };
use crate::analysis::{
    structures::*,
};
use crate::ui::components::*;

#[derive(Debug, Clone)]
pub enum Message {
    EntryMessage(usize, tolerance_entry::Message),
    FilterMessage(tolerance_filter::Message),
    NewEntryMessage(new_tolerance_entry::Message),
}

#[derive(Debug, Default, Clone)]
pub struct StackEditor {
    entry_form: NewToleranceEntry,
    filter: ToleranceFilter,
    pub tolerances: Vec<ToleranceEntry>,
    scroll_state: scrollable::State,
}
impl StackEditor {
    pub fn new() -> Self{
        StackEditor::default()
    }
    pub fn update(&mut self, message: Message) {
        let StackEditor {
            entry_form,
            filter,
            tolerances,
            scroll_state,
        } = self;
        match message {
            Message::NewEntryMessage(message) => {
                match &message {
                    new_tolerance_entry::Message::CreateTol(input_text, input_type) => {
                        if !input_text.is_empty() {
                            tolerances.push(ToleranceEntry::new(
                                input_text.clone(),
                                input_type.clone(),
                            ));
                        }
                    }
                    new_tolerance_entry::Message::TolNameChanged(_) => {}
                    new_tolerance_entry::Message::TolTypeChanged(_) => {}
                }
                entry_form.update(message);
            }

            Message::FilterMessage(message) => {
                match &message {
                    tolerance_filter::Message::FilterChanged(_) => {}
                };
                // Once we've processed the filter message in the parent component, pass the
                //  message into the filter to be processed.
                filter.update(message);
            }

            Message::EntryMessage(i, message) => {
                // Some message `tol_message`  from a tolerance entry at index `i`
                match &message {
                    tolerance_entry::Message::EntryDelete => {
                        tolerances.remove(i);
                    }
                    tolerance_entry::Message::EntryFinishEditing => match tolerances.get_mut(i) {
                        Some(entry) => match  &entry.input {
                            FormValues::Linear {
                                description,
                                dimension,
                                tolerance,
                            } => {
                                let mut sanitized_dimension = 0.0;
                                let mut sanitized_tolerance = 0.0;

                                entry.valid = true;

                                match dimension.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_dimension = value;
                                    }
                                    Err(e) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance = value;
                                    }
                                    Err(e) => {
                                        entry.valid = false;
                                    }
                                }
                                if entry.valid {
                                    entry.active = true;
                                    let linear = DimTol::new(
                                        sanitized_dimension, 
                                        sanitized_tolerance, 
                                        sanitized_tolerance, 
                                        3.0,
                                    );
                                    let linear = Tolerance::Linear(LinearTL::new(linear));
                                    entry.analysis_model = linear;
                                } else { entry.active = false; }
                            },
                            FormValues::Float {
                                description,
                                tolerance_hole,
                                tolerance_pin,
                            } => {
                                let mut sanitized_tolerance_hole = 0.0;
                                let mut sanitized_tolerance_pin = 0.0;

                                entry.valid = true;

                                match tolerance_hole.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_hole = value;
                                    }
                                    Err(e) => {
                                        entry.valid = false;
                                    }
                                }
                                match tolerance_pin.parse::<f64>() {
                                    Ok(value) => {
                                        sanitized_tolerance_pin = value;
                                    }
                                    Err(e) => {
                                        entry.valid = false;
                                    }
                                }
                                if entry.valid {
                                    entry.active = true;
                                    let hole = DimTol::new(
                                        0.0, 
                                        sanitized_tolerance_hole, 
                                        sanitized_tolerance_hole, 
                                        3.0,
                                    );
                                    let pin = DimTol::new(
                                        0.0, 
                                        sanitized_tolerance_pin, 
                                        sanitized_tolerance_pin, 
                                        3.0,
                                    );
                                    let data = Tolerance::Float(
                                        FloatTL::new(hole, pin,3.0)
                                    );
                                    //println!("{:#?}",data);
                                    entry.analysis_model = data;
                                }
                            },
                            FormValues::Compound {
                                description,
                                tolerance_hole_1,
                                tolerance_pin_1,
                                tolerance_hole_2,
                                tolerance_pin_2,
                            } => {

                            },
                        }
                        ,
                        None => {}
                    }
                    _ => {}
                };
                if let Some(tol) = tolerances.get_mut(i) {
                    // Once we've processed the entry message in the parent component, pass the
                    //  message into the entry it came from to be processed, after checking that
                    //  the entry exists.
                    tol.update(message);
                }
            }
        }
    }
    pub fn view(&mut self) -> Element<Message> {
        let StackEditor {
            entry_form,
            filter,
            tolerances,
            scroll_state,
        } = self;
        
        let filtered_tols =
            tolerances.iter().filter(|tol| filter.filter_value.matches(tol.analysis_model));
        let tolerances: Element<_> = if filtered_tols.count() > 0 {
            self.tolerances
                .iter_mut()
                .enumerate()
                .filter(|(_, tol)| filter.filter_value.matches(tol.analysis_model))
                .fold(Column::new().spacing(20), |column, (i, tol)| {
                    column.push(tol.view().map( move |message| {
                        Message::EntryMessage(i, message)
                    }))
                })
                .into()
            } else {
            empty_message(match filter.filter_value {
                Filter::All => "There are no tolerances in the stack yet.",
                Filter::Some(tol) => match tol {
                    Tolerance::Linear(_) => "No linear tolerances in the stack.",
                    Tolerance::Float(_) => "No float tolerances in the stack.",
                    Tolerance::Compound(_) => "No compound tolerances in the stack.",
                }
            })
        };
        let content = Column::new()
            .spacing(20)
            .push(tolerances);
        let stack_title = Text::new("Tolerance Stack")
            .width(Length::Fill)
            .size(24)
            .horizontal_alignment(HorizontalAlignment::Left);
        let scrollable_content = Scrollable::new(&mut self.scroll_state)
            .padding(10)
            .height(Length::Fill)
            .width(Length::Shrink)
            .push(
                Container::new(content).width(Length::Shrink).center_x(),
            );
        let filter_controls = filter.view()
            .map( move |message| { Message::FilterMessage(message) });
        let tol_stack_area = Container::new(
            Container::new(Column::new()
                    .push( Row::new()
                            .push(stack_title)
                            .push(filter_controls)
                            .padding(10)
                            .align_items(Align::Center)
                        )
                    .push(scrollable_content)
                )
                .style(style::Container::Background)
                .padding(10)
                .width(Length::Shrink)
            )
            .padding(20)
            .width(Length::Fill)
            .center_x();

        let new_tol_area = Container::new(
                Container::new(self.entry_form.view()
                    .map( move |message| { Message::NewEntryMessage(message) })
                )
                .padding(20)
                .style(style::Container::Background)
            )
            .padding(20)
            .width(Length::Fill)
            .center_x();
        
        let tol_chain_input = Column::new()
            .push(new_tol_area)
            .push(tol_stack_area)
            .width(Length::FillPortion(3));
            
        tol_chain_input.into()
    }
    pub fn tolerances(&mut self, tolerances: Vec<ToleranceEntry>) -> Self {
        self.tolerances = tolerances;
        self.clone()
    }
}

fn empty_message(message: &str) -> Element<'static, Message> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .into()
}