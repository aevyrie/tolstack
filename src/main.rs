//#![windows_subsystem = "windows"] // Tells windows compiler not to show console window

mod model;
mod tolerances;

use model::*;
use tolerances::*;
use std::time::Instant;

use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput,
};
use serde_derive::*;


fn main() {
    TolStack::run(Settings::default())
}

// The state of the application
#[derive(Debug, Default, Clone)]
struct StateApplication {
    project_name: EditableLabel,
    scroll_state: scrollable::State,
    button_state: button::State,
    tolerance_controls: ToleranceControls,
    filter_value: Filter,
    tol_entries: Vec<ToleranceEntry>,
    simulation_state: SimulationState,
    simulation_result: ModelResults,
    result_output: String,
    filter_controls: FilterControls,
    dirty: bool,
    saving: bool,
    valid_stack: bool,
}
// Messages - events for users to change the application state
#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    TolNameChanged(String),
    TolTypeChanged(ToleranceTypes),
    CreateTol,
    Calculate,
    CalculateComplete(Option<ModelResults>),
    FilterChanged(Filter),
    TolMessage(usize, MessageEntryTol),
    LabelMessage(LabelMessage),
}

// Loading state wrapper
#[derive(Debug)]
enum TolStack {
    Loading,
    Loaded(StateApplication),
}
impl Application for TolStack {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (TolStack, Command<Message>) {
        (
            TolStack::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            TolStack::Loading => false,
            TolStack::Loaded(state) => state.dirty,
        };
        let project_name = match self {
            TolStack::Loading => String::from("Loading..."),
            TolStack::Loaded(state) => if state.project_name.text.len() == 0 {
                String::from("New Project")
            } else {
                state.project_name.text.clone()
            }};

        format!("{}{} - TolStack Tolerance Analysis", project_name, if dirty { "*" } else { "" })
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            // While the application is loading:
            TolStack::Loading => {
                match message {
                    // Take the loaded state and assign to the working state
                    Message::Loaded(Ok(state)) => {
                        *self = TolStack::Loaded(StateApplication {
                            project_name: state.project_name,
                            filter_value: state.filter,
                            simulation_state: state.simulation,
                            simulation_result: state.results,
                            tol_entries: state.tolerance_entries,
                            ..StateApplication::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = TolStack::Loaded(StateApplication::default());
                    }
                    _ => {}
                }

                Command::none()
            }
            // Once the application has loaded:
            TolStack::Loaded(state) => {
                let mut saved = false;

                match message {
                    Message::TolTypeChanged(value) => {
                        state.tolerance_controls.tolerance_type = value;
                        Command::none()
                    }
                    Message::TolNameChanged(value) => {
                        state.tolerance_controls.tolerance_text_value = value;
                        Command::none()
                    }
                    Message::CreateTol => {
                        let input_text = state.tolerance_controls.tolerance_text_value.clone();
                        let input_type = state.tolerance_controls.tolerance_type;
                        if !input_text.is_empty() {
                            state
                                .tol_entries
                                .push(ToleranceEntry::new(
                                    input_text.clone(),
                                    input_type.clone(),
                                ));
                            state.tolerance_controls.tolerance_text_value.clear();
                        }
                        Command::none()
                    }
                    Message::TolMessage(i, MessageEntryTol::EntryDelete) => {
                        state.tol_entries.remove(i);
                        Command::none()
                    }
                    Message::TolMessage(i, tol_message) => {
                        // Some message `tol_message`  from a tolerance entry at index `i`
                        match &tol_message {
                            MessageEntryTol::EntryFinishEditing => match state.tol_entries.get_mut(i) {
                                Some(entry) => match  &entry.value_input {
                                    ValueInputFormTolerance::Linear {
                                        value_input_description,
                                        value_input_dimension,
                                        value_input_tolerance,
                                    } => {
                                        let mut sanitized_dimension = 0.0;
                                        let mut sanitized_tolerance = 0.0;

                                        entry.valid = true;

                                        match value_input_dimension.parse::<f64>() {
                                            Ok(value) => {
                                                sanitized_dimension = value;
                                            }
                                            Err(e) => {
                                                entry.valid = false;
                                            }
                                        }
                                        match value_input_tolerance.parse::<f64>() {
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
                                            println!("{:#?}", linear);
                                            entry.backend_model_data = Some(linear);
                                        } else { entry.active = false; }
                                    },
                                    ValueInputFormTolerance::Float {
                                        value_input_description,
                                        value_input_tolerance_hole,
                                        value_input_tolerance_pin,
                                    } => {
                                        let mut sanitized_tolerance_hole = 0.0;
                                        let mut sanitized_tolerance_pin = 0.0;

                                        entry.valid = true;

                                        match value_input_tolerance_hole.parse::<f64>() {
                                            Ok(value) => {
                                                sanitized_tolerance_hole = value;
                                            }
                                            Err(e) => {
                                                entry.valid = false;
                                            }
                                        }
                                        match value_input_tolerance_pin.parse::<f64>() {
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
                                            println!("{:#?}",data);
                                            entry.backend_model_data = Some(data);
                                        }
                                    },
                                    ValueInputFormTolerance::Compound {
                                        value_input_description,
                                        value_input_tolerance_hole_1,
                                        value_input_tolerance_pin_1,
                                        value_input_tolerance_hole_2,
                                        value_input_tolerance_pin_2,
                                    } => {

                                    },
                                }
                                ,
                                None => {}
                            }
                            _ => {}
                        };
                        if let Some(tol) = state.tol_entries.get_mut(i) {
                            tol.update(tol_message);
                        }
                        Command::none()
                    }
                    Message::LabelMessage(label_message) => {
                        state.project_name.update(label_message);
                        Command::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter_value = filter;
                        Command::none()
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                        Command::none()
                    }
                    Message::Calculate => {
                        println!("Calculation started");
                        Command::perform(compute(state.clone()), Message::CalculateComplete)
                        //state.compute();
                    }
                    Message::CalculateComplete(result) => {
                        println!("Calculation complete");
                        match result {
                            Some(result) => state.simulation_result = result,
                            None => {}
                        }
                        Command::none()
                    }
                    Message::Loaded(_) => {
                        Command::none()
                    }
                }

                if !saved {
                    state.dirty = true;
                }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            project_name: state.project_name.clone(),
                            filter: state.filter_value,
                            simulation: state.simulation_state.clone(),
                            results: state.simulation_result.clone(),
                            tolerance_entries: state.tol_entries.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                }
            }
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view(&mut self) -> Element<Message> {
        match self {
            TolStack::Loading => loading_message(),
            TolStack::Loaded(StateApplication {
                project_name,
                scroll_state,
                button_state,
                tolerance_controls,
                filter_value,
                tol_entries,
                simulation_state,
                simulation_result,
                result_output,
                filter_controls,
                dirty,
                saving,
                valid_stack,
            }) => {
                let project_label = Text::new("Project: ")
                    .width(Length::Shrink)
                    .size(32)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Left);
                let project_name: Row<_> = Row::new()
                    .push(project_label)
                    .push(project_name.view().map( move |message| {
                        Message::LabelMessage(message)
                    }))
                    .align_items(Align::Center)
                    .spacing(10)
                    .into();
                                    
                let project_title = 
                    Container::new(
                        Row::new()
                            .push(project_name)
                            .width(Length::Shrink)
                    )
                    .width(Length::Fill)
                    .center_x()
                    .center_y();

                let tolerance_controls = tolerance_controls.view().padding(20);
                let filter_controls = filter_controls.view(&tol_entries, *filter_value);
                let filtered_tols =
                    tol_entries.iter().filter(|tol| filter_value.matches(tol.tolerance_type));
                let tolerance_entries: Element<_> = if filtered_tols.count() > 0 {
                    tol_entries
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, tol)| filter_value.matches(tol.tolerance_type))
                        .fold(Column::new().spacing(20), |column, (i, tol)| {
                            column.push(tol.view().map( move |message| {
                                Message::TolMessage(i, message)
                            }))
                        })
                        .into()
                    } else {
                    empty_message(match filter_value {
                        Filter::All => "There are no tolerances in the stack yet.",
                        Filter::Some(tol) => match tol {
                            ToleranceTypes::Linear => "No linear tolerances in the stack.",
                            ToleranceTypes::Float => "No float tolerances in the stack.",
                            ToleranceTypes::Compound => "No compound tolerances in the stack.",
                        }
                    })
                };

                let header = Container::new(
                Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(project_title)
                )
                .width(Length::Fill)
                .padding(10)
                .center_x();
                
                let content = Column::new()
                    .spacing(20)
                    .push(tolerance_entries);
                let stack_title = Text::new("Tolerance Stack")
                    .width(Length::Fill)
                    .size(24)
                    .horizontal_alignment(HorizontalAlignment::Left);
                let scrollable_content = Scrollable::new(scroll_state)
                    .padding(10)
                    .height(Length::Fill)
                    .width(Length::Shrink)
                    .push(
                        Container::new(content).width(Length::Shrink).center_x(),
                    );
                let filtereable_scroll_region = Container::new(
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

                let results_header = Column::new()
                    .push(Row::new()
                        .push(Text::new("Results")
                            .size(24)
                            .width(Length::Fill))
                        .push(
                            Button::new( 
                                button_state, 
                                Row::new()
                                    .spacing(10)
                                    //.push(check_icon())
                                    .push(Text::new("Calculate")),
                            )
                            .style(style::Button::Constructive)
                            .padding(10)
                            .on_press(Message::Calculate)
                        )
                        .align_items(Align::Center)
                        .width(Length::Fill)
                    );

                let results_body = Column::new()
                    .push(Row::new()
                        .push(Text::new("Mean:"))
                        .push(Text::new(format!("{:.3}",simulation_result.mean)))
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Tolerance:"))
                        .push(Text::new(format!("{:.3}",simulation_result.tolerance)))
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Standard Deviation:"))
                        .push(Text::new(format!("{:.3}",simulation_result.stddev)))
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Iterations:"))
                        .push(Text::new(format!("{}",simulation_result.iterations)))
                        .spacing(20)
                    )
                    .spacing(20);

                let results_summary = Container::new(Column::new()
                        .push(results_header)
                        .push(results_body)
                        .height(Length::Fill)
                        .spacing(40)
                    )
                    .padding(10);


                let tol_chain_input = Column::new()
                    .push(Container::new(Container::new(tolerance_controls)
                            .style(style::Container::Background)
                        )
                        .padding(20)
                        .width(Length::Fill)
                        .center_x()
                    )
                    .push(filtereable_scroll_region)
                    .width(Length::FillPortion(3));

                let tol_chain_output = Column::new()
                    .push(Container::new(Container::new(results_summary)
                        .style(style::Container::Background)
                        .padding(5)
                        .height(Length::Fill)
                    )
                    .padding(20)
                    .height(Length::Fill)
                    .center_x()
                    )
                    .height(Length::Fill)
                    .width(Length::FillPortion(2));
                
                let gui: Element<_> = Column::new()
                    .padding(20)
                    .push(header)
                    .push(Row::new()
                        .push(tol_chain_input)
                        .push(tol_chain_output)
                    )
                    .into();
                
                //let gui = gui.explain(Color::BLACK);

                gui.into()
            }
        }
    }
}

async fn compute(mut state: StateApplication) -> Option<ModelResults> {
    //self.simulation_state = SimulationState::default();
    state.simulation_state.clear();
    // Make sure all active entries are valid
    let mut valid = true;
    for entry in &state.tol_entries {
        if entry.active && !entry.valid { 
            valid = false;
        }
    }
    // Build the model
    if valid {
        for entry in &state.tol_entries {
            if entry.active {
                match &entry.backend_model_data {
                    Some(data) => {
                        println!("ADDING TO MODEL:\n{:#?}", entry);
                        state.simulation_state.add(data.clone());
                    },
                    None => {}, // TODO handle this case, could result in bad output
                }
            }
        }
    }

    let time_start = Instant::now();
    let result = model::run(&state.simulation_state).await.unwrap();
    let duration = time_start.elapsed();

    println!("Result: {:.3} +/- {:.3}; Stddev: {:.3};\nSamples: {}; Duration: {:.3?}", 
        state.simulation_result.mean, 
        state.simulation_result.tolerance, 
        state.simulation_result.stddev, 
        state.simulation_result.iterations,
        duration,
    );

    Some(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EditableLabel {
    text: String,
    #[serde(skip)]
    state: TextEditState,
}
impl EditableLabel {
    fn new(text: String) -> Self {
        EditableLabel {
            text,
            state: TextEditState::Idle {
                edit_button: button::State::new(),
            },
        }
    }

    fn update(&mut self, message: LabelMessage) {
        match message {
            LabelMessage::Edit => {
                self.state = TextEditState::Editing {
                    text_input: text_input::State::focused(),
                };
            }
            LabelMessage::TextEdited(new_text) => {
                self.text = new_text;
            }
            LabelMessage::FinishEditing => {
                if !self.text.is_empty() {
                    self.state = TextEditState::Idle {
                        edit_button: button::State::new(),
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<LabelMessage> {
        match &mut self.state {
            TextEditState::Idle { edit_button } => {
                let label = Text::new(self.text.clone())
                    .width(Length::Shrink)
                    .size(32)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Left);

                let row_contents = Row::new()
                    .spacing(10)
                    .align_items(Align::Center)
                    .push(label)
                    .push(
                        Button::new(edit_button, edit_icon())
                            .on_press(LabelMessage::Edit)
                            .padding(10)
                            .style(style::Button::Icon),
                    );

                Container::new(row_contents)
                    .into()
            }
            TextEditState::Editing {
                text_input,
            } => {
                let text_input = TextInput::new(
                    text_input,
                    "New Project",
                    &self.text,
                    LabelMessage::TextEdited,
                )
                .on_submit(LabelMessage::FinishEditing)
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
            text: String::from("New Project"),
            state: TextEditState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TextEditState {
    Idle {
        edit_button: button::State,
    },
    Editing {
        text_input: text_input::State,
    },
}
impl Default for TextEditState {
    fn default() -> Self {
        TextEditState::Idle {
            edit_button: button::State::new(),
        }
    }
}

// TODO combine ToleranceTypes with ValueInputFormTolerance
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ValueInputFormTolerance {
    Linear { 
        value_input_description: String,
        value_input_dimension: String,
        value_input_tolerance: String,
    },
    Float {
        value_input_description: String,
        value_input_tolerance_hole: String,
        value_input_tolerance_pin: String,
    },
    Compound {
        value_input_description: String,
        value_input_tolerance_hole_1: String,
        value_input_tolerance_pin_1: String,
        value_input_tolerance_hole_2: String,
        value_input_tolerance_pin_2: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToleranceEntry {
    value_input: ValueInputFormTolerance,
    backend_model_data: Option<Tolerance>,
    tolerance_type: ToleranceTypes,
    active: bool,
    valid: bool,

    #[serde(skip)]
    state: StateEntryTolerance,
}
impl ToleranceEntry {
    fn new(description: String, tolerance_type: ToleranceTypes) -> Self {
        ToleranceEntry {
            value_input: match tolerance_type {
                ToleranceTypes::Linear => {
                    ValueInputFormTolerance::Linear{
                        value_input_description: description,
                        value_input_dimension: String::from(""),
                        value_input_tolerance: String::from(""),
                    }
                }
                ToleranceTypes::Float => {
                    ValueInputFormTolerance::Float{
                        value_input_description: description,
                        value_input_tolerance_hole: String::from(""),
                        value_input_tolerance_pin: String::from(""),
                    }
                }
                ToleranceTypes::Compound => {
                    ValueInputFormTolerance::Compound{
                        value_input_description: description,
                        value_input_tolerance_hole_1: String::from(""),
                        value_input_tolerance_pin_1: String::from(""),
                        value_input_tolerance_hole_2: String::from(""),
                        value_input_tolerance_pin_2: String::from(""),
                    }
                }
            },
            backend_model_data: None,
            tolerance_type: tolerance_type,
            active: false,
            valid: false,
            state: StateEntryTolerance::Idle {
                state_button_edit: button::State::new(),
            },
        }
    }

    fn update(&mut self, message: MessageEntryTol) {
        match message {
            MessageEntryTol::EntryActive(is_active) => {
                if self.valid { 
                    self.active = is_active 
                } else {
                    self.active = false;
                }
            }
            MessageEntryTol::EntryEdit => {
                self.state = match self.tolerance_type {
                    ToleranceTypes::Linear => {
                        StateEntryTolerance::Editing {
                            state_form_tolentry: StateFormTolerance::Linear {
                                state_button_save: button::State::new(),
                                state_button_delete: button::State::new(),
                                state_input_description: text_input::State::focused(),
                                state_input_dimension: text_input::State::new(),
                                state_input_tolerance: text_input::State::new(),
                            }
                        }
                    }
                    ToleranceTypes::Float => {
                        StateEntryTolerance::Editing {
                            state_form_tolentry: StateFormTolerance::Float {
                                state_button_save: button::State::new(),
                                state_button_delete: button::State::new(),
                                state_input_description: text_input::State::focused(),
                                state_input_tolerance_hole: text_input::State::new(),
                                state_input_tolerance_pin: text_input::State::new(),
                            }
                        }
                    }
                    ToleranceTypes::Compound => {
                        StateEntryTolerance::Editing {
                            state_form_tolentry: StateFormTolerance::Compound {
                            }
                        }
                    }
                };
            }
            MessageEntryTol::EntryFinishEditing => {
                if match &self.value_input {
                    ValueInputFormTolerance::Linear{value_input_description,..} => {
                        !value_input_description.is_empty()
                    },
                    ValueInputFormTolerance::Float{value_input_description,..} => {
                        !value_input_description.is_empty()
                    },
                    ValueInputFormTolerance::Compound{value_input_description,..} => {
                        !value_input_description.is_empty()
                    },
                } {
                    self.state = StateEntryTolerance::Idle {
                        state_button_edit: button::State::new(),
                    }
                }
            }
            MessageEntryTol::EntryDelete => {}
            MessageEntryTol::EditedDescription(input) => {
                match &mut self.value_input {
                    ValueInputFormTolerance::Linear{value_input_description,..} => {
                        *value_input_description = input
                    },
                    ValueInputFormTolerance::Float{value_input_description,..} => {
                        *value_input_description = input
                    },
                    ValueInputFormTolerance::Compound{value_input_description,..} => {
                        *value_input_description = input
                    },
                };
            }
            MessageEntryTol::EditedLinearDimension(input) => {
                match &mut self.value_input {
                    ValueInputFormTolerance::Linear{value_input_dimension,..} => {
                        *value_input_dimension = NumericString::eval(
                            value_input_dimension,
                            &input,
                            NumericString::Number
                        )
                    },
                    _ => {}
                };
            }
            MessageEntryTol::EditedLinearTolerance(input) => {
                match &mut self.value_input {
                    ValueInputFormTolerance::Linear{value_input_tolerance,..} => {
                        *value_input_tolerance = NumericString::eval(
                            value_input_tolerance,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            MessageEntryTol::EditedFloatTolHole(input) => {
                match &mut self.value_input {
                    ValueInputFormTolerance::Float{value_input_tolerance_hole,..} => {
                        *value_input_tolerance_hole = NumericString::eval(
                            value_input_tolerance_hole,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
            MessageEntryTol::EditedFloatTolPin(input) => {
                match &mut self.value_input {
                    ValueInputFormTolerance::Float{value_input_tolerance_pin,..} => {
                        *value_input_tolerance_pin = NumericString::eval(
                            value_input_tolerance_pin,
                            &input,
                            NumericString::Positive
                        )
                    },
                    _ => {}
                };
            }
        }
    }

    fn view(&mut self) -> Element<MessageEntryTol> {
        match &mut self.state {
            StateEntryTolerance::Idle { state_button_edit } => {
                let checkbox = Checkbox::new(
                    self.active,
                    match &self.value_input {
                        ValueInputFormTolerance::Linear{value_input_description,..} => {
                            value_input_description
                        },
                        ValueInputFormTolerance::Float{value_input_description,..} => {
                            value_input_description
                        },
                        ValueInputFormTolerance::Compound{value_input_description,..} => {
                            value_input_description
                        },
                    },
                    MessageEntryTol::EntryActive,
                )
                .width(Length::Fill);

                let row_contents = Row::new()
                    .padding(10)    
                    .spacing(20)
                    .align_items(Align::Center)
                    .push( checkbox )
                    .push(
                        Button::new(state_button_edit, edit_icon())
                            .on_press(MessageEntryTol::EntryEdit)
                            .padding(10)
                            .style(style::Button::Icon),
                    );

                Container::new(row_contents)
                    .style(style::Container::Entry)
                    .into()
            }
            StateEntryTolerance::Editing { state_form_tolentry } => {
                match state_form_tolentry {
                    StateFormTolerance::Linear {
                        state_button_save,
                        state_button_delete,
                        state_input_description,
                        state_input_dimension,
                        state_input_tolerance,
                    } => {
                        
                        let view_button_save =
                            Button::new(
                                state_button_save,
                                Row::new()
                                    .spacing(10)
                                    .push(check_icon())
                                    .push(Text::new("Save")),
                            )
                            .on_press(MessageEntryTol::EntryFinishEditing)
                            .padding(10)
                            .style(style::Button::Constructive);
                        
                        let view_button_delete =
                            Button::new(
                                state_button_delete,
                                Row::new()
                                    .spacing(10)
                                    .push(delete_icon())
                                    .push(Text::new("Delete")),
                            )
                            .on_press(MessageEntryTol::EntryDelete)
                            .padding(10)
                            .style(style::Button::Destructive);

                        let view_input_description = 
                            TextInput::new(
                                state_input_description,
                                "Enter a description",
                                match &self.value_input {
                                    ValueInputFormTolerance::Linear{value_input_description,..} => {
                                        value_input_description
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                MessageEntryTol::EditedDescription,
                            )
                            .on_submit(MessageEntryTol::EntryFinishEditing)
                            .padding(10);
                        
                        let view_input_dimension = 
                            TextInput::new(
                                state_input_dimension,
                                "Enter a value",
                                match &self.value_input {
                                    ValueInputFormTolerance::Linear{value_input_dimension,..} => {
                                        value_input_dimension
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                MessageEntryTol::EditedLinearDimension,
                            )
                            .on_submit(MessageEntryTol::EntryFinishEditing)
                            .padding(10);
                        
                        let view_input_tolerance = 
                            TextInput::new(
                                state_input_tolerance,
                                "Enter a value",
                                match &self.value_input {
                                    ValueInputFormTolerance::Linear{value_input_tolerance,..} => {
                                        value_input_tolerance
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                MessageEntryTol::EditedLinearTolerance,
                            )
                            .on_submit(MessageEntryTol::EntryFinishEditing)
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
                            .push(view_input_description)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_dimension = Row::new()
                            .push(Text::new("Dimension:"))
                            .push(view_input_dimension)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance = Row::new()
                            .push(Text::new("Tolerance:"))
                            .push(view_input_tolerance)
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
                    StateFormTolerance::Float {
                        state_button_save,
                        state_button_delete,
                        state_input_description,
                        state_input_tolerance_hole,
                        state_input_tolerance_pin,
                    } => {
                        
                        let view_button_save =
                            Button::new(
                                state_button_save,
                                Row::new()
                                    .spacing(10)
                                    .push(check_icon())
                                    .push(Text::new("Save")),
                            )
                            .on_press(MessageEntryTol::EntryFinishEditing)
                            .padding(10)
                            .style(style::Button::Constructive);
                        
                        let view_button_delete =
                            Button::new(
                                state_button_delete,
                                Row::new()
                                    .spacing(10)
                                    .push(delete_icon())
                                    .push(Text::new("Delete")),
                            )
                            .on_press(MessageEntryTol::EntryDelete)
                            .padding(10)
                            .style(style::Button::Destructive);

                        let view_input_description = 
                            TextInput::new(
                                state_input_description,
                                "Enter a description",
                                match &self.value_input {
                                    ValueInputFormTolerance::Float{value_input_description,..} => {
                                        value_input_description
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                MessageEntryTol::EditedDescription,
                            )
                            .on_submit(MessageEntryTol::EntryFinishEditing)
                            .padding(10);
                        
                        let view_input_tolerance_hole = 
                            TextInput::new(
                                state_input_tolerance_hole,
                                "Enter a value",
                                match &self.value_input {
                                    ValueInputFormTolerance::Float{value_input_tolerance_hole,..} => {
                                        value_input_tolerance_hole
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                MessageEntryTol::EditedFloatTolHole,
                            )
                            .on_submit(MessageEntryTol::EntryFinishEditing)
                            .padding(10);
                        
                        let view_input_tolerance_pin = 
                            TextInput::new(
                                state_input_tolerance_pin,
                                "Enter a value",
                                match &self.value_input {
                                    ValueInputFormTolerance::Float{value_input_tolerance_pin,..} => {
                                        value_input_tolerance_pin
                                    },
                                    _ => {"Error: tolerance type mismatch"}
                                },
                                MessageEntryTol::EditedFloatTolPin,
                            )
                            .on_submit(MessageEntryTol::EntryFinishEditing)
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
                            .push(view_input_description)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_dimension = Row::new()
                            .push(Text::new("Hole Tolerance:"))
                            .push(view_input_tolerance_hole)
                            .spacing(20)
                            .align_items(Align::Center);

                        let row_tolerance = Row::new()
                            .push(Text::new("Pin Tolerance:"))
                            .push(view_input_tolerance_pin)
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
                    StateFormTolerance::Compound {} => {
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

#[derive(Debug, Clone)]
pub enum StateEntryTolerance {
    Idle {
        state_button_edit: button::State,
    },
    Editing {
        state_form_tolentry: StateFormTolerance,
    },
}
impl Default for StateEntryTolerance {
    fn default() -> Self {
        StateEntryTolerance::Idle {
            state_button_edit: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StateFormTolerance {
    Linear {
        state_button_save: button::State,
        state_button_delete: button::State,
        state_input_description: text_input::State,
        state_input_dimension: text_input::State,
        state_input_tolerance: text_input::State,
    },
    Float {
        state_button_save: button::State,
        state_button_delete: button::State,
        state_input_description: text_input::State,
        state_input_tolerance_hole: text_input::State,
        state_input_tolerance_pin: text_input::State,
    },
    Compound {},
}

#[derive(Debug, Default, Clone)]
struct ToleranceControls {
    tolerance_type: ToleranceTypes,
    tolerance_text_value: String,
    tolerance_text_state: text_input::State,
    linear_button: button::State,
    float_button: button::State,
    compound_button: button::State,
}
impl ToleranceControls {
    fn view(&mut self) -> Row<Message> {
        let ToleranceControls {
            tolerance_type,
            tolerance_text_value,
            tolerance_text_state,
            linear_button,
            float_button,
            compound_button,
        } = self;

        let tolerance_label = Text::new("Add Tolerance")
                    .width(Length::Fill)
                    .size(24)
                    .horizontal_alignment(HorizontalAlignment::Left);
        let tolerance_text = TextInput::new(
            tolerance_text_state,
            "Tolerance name, press enter to add.",
            tolerance_text_value,
            Message::TolNameChanged,
            )
            .padding(15)
            .on_submit(Message::CreateTol);

        let button = |state, label, tolerance: ToleranceTypes, current_tol: ToleranceTypes| {
            let label = Text::new(label).size(18);
            let button =
                Button::new(state, label).style(style::Button::Choice {
                    selected: tolerance == current_tol,
                });

            button.on_press(Message::TolTypeChanged(tolerance)).padding(8)
        };

        Row::new().push(Column::new()
            .push(Row::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(tolerance_label)
                .push(
                    Row::new()
                        .width(Length::Shrink)
                        .spacing(10)
                        .push(button(
                            linear_button,
                            "Linear",
                            ToleranceTypes::Linear,
                            self.tolerance_type,
                        ))
                        .push(button(
                            float_button,
                            "Float",
                            ToleranceTypes::Float,
                            self.tolerance_type,
                        ))
                        /*.push(button(
                            compound_button,
                            "Compound",
                            ToleranceTypes::Compound,
                            self.tolerance_type,
                        ))*/
                )
            )
            .push(tolerance_text)
            .spacing(10)
        )
    }
}

#[derive(Debug, Default, Clone)]
struct FilterControls {
    all_button: button::State,
    linear_button: button::State,
    float_button: button::State,
    compound_button: button::State,
}
impl FilterControls {
    fn view(&mut self, tols: &Vec<ToleranceEntry>, current_filter: Filter) -> Row<Message> {
        let FilterControls {
            all_button,
            linear_button,
            float_button,
            compound_button,
        } = self;

        let filter_button = |state, label, filter, current_filter| {
            let label = Text::new(label).size(16);
            let button =
                Button::new(state, label).style(style::Button::Filter {
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
                    .push(filter_button(
                        all_button,
                        "All",
                        Filter::All,
                        current_filter,
                    ))
                    .push(filter_button(
                        linear_button,
                        "Linear",
                        Filter::Some(ToleranceTypes::Linear),
                        current_filter,
                    ))
                    .push(filter_button(
                        float_button,
                        "Float",
                        Filter::Some(ToleranceTypes::Float),
                        current_filter,
                    ))
                    /*.push(filter_button(
                        compound_button,
                        "Compound",
                        Filter::Some(ToleranceTypes::Compound),
                        current_filter,
                    )),*/
            )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    project_name: EditableLabel,
    filter: Filter,
    simulation: SimulationState,
    results: ModelResults,
    tolerance_entries: Vec<ToleranceEntry>,
}

#[derive(Debug, Clone)]
pub enum MessageEntryTol {
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

#[derive(Debug, Clone)]
pub enum LabelMessage {
    Edit,
    TextEdited(String),
    FinishEditing,
}

#[derive(Debug, Clone)]
pub enum Controls {
    SolvePressed,
    OpenFilePressed,
    SaveFilePressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToleranceTypes {
    Linear,
    Float,
    Compound,
}
impl Default for ToleranceTypes {
    fn default() -> Self {
        ToleranceTypes::Linear
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Some(ToleranceTypes),
}
impl Filter {
    fn matches(&self, tol: ToleranceTypes) -> bool {
        match self {
            Filter::All => true,
            Filter::Some(tol_self) => *tol_self == tol,
        }
    }
}
impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
enum SaveError {
    DirectoryError,
    FileError,
    WriteError,
    FormatError,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path = if let Some(project_dirs) =
            directories::ProjectDirs::from("rs", "aevyrie", "TolStack")
        {
            project_dirs.data_dir().into()
        } else {
            std::env::current_dir().unwrap_or(std::path::PathBuf::new())
        };

        path.push("tolstack.json");

        path
    }

    async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;

        let json = serde_json::to_string_pretty(&self)
            .map_err(|_| SaveError::FormatError)?;

        let path = Self::path();

        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }

        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;

            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }
}

fn loading_message() -> Element<'static, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .into()
}

mod style {
    use iced::{button, container, Background, Color, Vector};

    pub enum Button {
        Filter { selected: bool },
        Choice { selected: bool },
        Icon,
        Destructive,
        Constructive,
    }
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Filter { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(
                                Color::from_rgb(0.95, 0.95, 0.95),
                            )),
                            border_radius: 5,
                            text_color: Color::BLACK,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style::default()
                    }
                }
                Button::Choice { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(
                                Color::from_rgb(0.2, 0.4, 0.7),
                            )),
                            border_radius: 5,
                            text_color: Color::WHITE,
                            ..button::Style::default()
                        }
                    } else {
                        button::Style::default()
                    }
                }
                Button::Icon => button::Style {
                    text_color: Color::from_rgb(0.5, 0.5, 0.5),
                    ..button::Style::default()
                },
                Button::Destructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(
                        0.8, 0.2, 0.2,
                    ))),
                    border_radius: 5,
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..button::Style::default()
                },
                Button::Constructive => button::Style {
                    background: Some(Background::Color(Color::from_rgb(
                        0.2, 0.8, 0.2,
                    ))),
                    border_radius: 5,
                    text_color: Color::WHITE,
                    shadow_offset: Vector::new(1.0, 1.0),
                    ..button::Style::default()
                },
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                    Button::Filter { selected } if !selected => {
                        Color::from_rgb(0.5, 0.5, 0.5)
                    }
                    Button::Filter { selected } if !selected => {
                        Color::from_rgb(0.3, 0.5, 0.8)
                    }
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
            }
        }
    }

    pub enum Container {
        Entry,
        Background,
    }
    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            match self {
                Container::Entry => container::Style {
                    text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
                    border_radius: 5,
                    border_width: 1,
                    border_color: Color::from_rgb(0.9, 0.9, 0.9),
                },
                Container::Background => container::Style {
                    text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                    background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
                    border_radius: 5,
                    border_width: 1,
                    border_color: Color::from_rgb(0.9, 0.9, 0.9),
                },
            }
        }
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

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

fn edit_icon() -> Text {
    icon('\u{F303}')
}

fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

fn check_icon() -> Text {
    icon('\u{2713}')
}