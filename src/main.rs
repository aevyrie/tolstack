mod model;
mod tolerances;

use model::*;
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput,
};
use serde::{Deserialize, Serialize};
use serde_derive::*;

fn main() {
    TolStack::run(Settings::default())
}

// Loading state wrapper
#[derive(Debug)]
enum TolStack {
    Loading,
    Loaded(State),
}

// The state of the application
#[derive(Debug, Default)]
struct State {
    filename_state: text_input::State,
    filename_value: String,
    scroll_state: scrollable::State,
    tolerance_controls: ToleranceControls,
    filter_value: Filter,
    tolerance_entries: Vec<ToleranceEntry>,
    simulation_state: SimulationState,
    filter_controls: FilterControls,
    dirty: bool,
    saving: bool,
}

// Messages - events for users to change the application state
#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    DescriptionChanged(String),
    TolNameChanged(String),
    TolTypeChanged(ToleranceTypes),
    CreateTol,
    FilterChanged(Filter),
    TolMessage(usize, TolMessage),
    Controls,
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

        format!("TolStack Tolerance Analysis - New{}", if dirty { "*" } else { "" })
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            TolStack::Loading => {
                match message {
                    // Take the loaded state and assign to the working state
                    Message::Loaded(Ok(state)) => {
                        *self = TolStack::Loaded(State {
                            filename_value: state.filename,
                            filter_value: state.filter,
                            simulation_state: state.simulation,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = TolStack::Loaded(State::default());
                    }
                    _ => {}
                }

                Command::none()
            }
            TolStack::Loaded(state) => {
                let mut saved = false;

                match message {
                    Message::DescriptionChanged(value) => {
                        state.filename_value = value;
                    }
                    Message::CreateTol => {
                        let input_text = state.tolerance_controls.tolerance_text_value.clone();
                        let input_type = state.tolerance_controls.tolerance_type;
                        if !input_text.is_empty() {
                            state
                                .tolerance_entries
                                .push(ToleranceEntry::new(
                                    input_text.clone(),
                                    input_type.clone(),
                                ));
                            state.tolerance_controls.tolerance_text_value.clear();
                        }
                    }
                    Message::FilterChanged(filter) => {
                        state.filter_value = filter;
                    }

                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }
                    _ => {}
                }

                if !saved {
                    state.dirty = true;
                }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            filename: state.filename_value.clone(),
                            filter: state.filter_value,
                            simulation: state.simulation_state.clone(),
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
            TolStack::Loaded(State {
                filename_state,
                filename_value,
                scroll_state,
                tolerance_controls,
                filter_value,
                tolerance_entries,
                simulation_state,
                filter_controls,
                dirty,
                saving,
            }) => {
                let title = Text::new("TolStack")
                    .width(Length::Fill)
                    .size(32)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);
                let filename = TextInput::new(
                    filename_state,
                    "What do you want to call this file?",
                    filename_value,
                    Message::TolNameChanged,
                    )
                    .padding(15)
                    .size(30);
                let tolerance_controls = tolerance_controls.view();
                let filter_controls = filter_controls.view(&tolerance_entries, *filter_value);
                let filtered_tols =
                    tolerance_entries.iter().filter(|tol| filter_value.matches(tol.tolerance_type));
                let tolerance_entries: Element<_> = if filtered_tols.count() > 0 {
                    tolerance_entries
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
                        Filter::All => "You haven't added a tolerance to the chain yet.",
                        Filter::Some(tol) => match tol {
                            ToleranceTypes::Linear => "No linear tolerances in the chain.",
                            ToleranceTypes::Float => "No float tolerances in the chain.",
                            ToleranceTypes::Compound => "No compoind tolerances in the chain.",
                        }
                    })
                };

                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title)
                    .push(filename)
                    .push(tolerance_controls)
                    .push(filter_controls)
                    .push(tolerance_entries);

                Scrollable::new(scroll_state)
                    .padding(40)
                    .push(
                        Container::new(content).width(Length::Fill).center_x(),
                    )
                    .into()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToleranceEntry {
    description: String,
    model_data: Option<ToleranceType>,
    tolerance_type: ToleranceTypes,
    active: bool,

    #[serde(skip)]
    state: EntryState,
}
impl ToleranceEntry {
    fn new(description: String, tolerance_type: ToleranceTypes) -> Self {
        ToleranceEntry {
            description,
            model_data: Option::None,
            tolerance_type,
            active: true,
            state: EntryState::Idle {
                edit_button: button::State::new(),
            },
        }
    }

    fn update(&mut self, message: TolMessage) {
        match message {
            TolMessage::Active(is_active) => {
                self.active = is_active;
            }
            TolMessage::Edit => {
                self.state = EntryState::Editing {
                    text_input: text_input::State::focused(),
                    delete_button: button::State::new(),
                };
            }
            TolMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TolMessage::TolEdited(tolerance) => {
                self.model_data = Some(tolerance);
            }
            TolMessage::FinishEditing => {
                if !self.description.is_empty() {
                    self.state = EntryState::Idle {
                        edit_button: button::State::new(),
                    }
                }
            }
            TolMessage::Delete => {}
        }
    }

    fn view(&mut self) -> Element<TolMessage> {
        match &mut self.state {
            EntryState::Idle { edit_button } => {
                let checkbox = Checkbox::new(
                    self.active,
                    &self.description,
                    TolMessage::Active,
                )
                .width(Length::Fill);

                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(checkbox)
                    .push(
                        Button::new(edit_button, edit_icon())
                            .on_press(TolMessage::Edit)
                            .padding(10)
                            .style(style::Button::Icon),
                    )
                    .into()
            }
            EntryState::Editing {
                text_input,
                delete_button,
            } => {
                let text_input = TextInput::new(
                    text_input,
                    "Describe this tolerance...",
                    &self.description,
                    TolMessage::DescriptionEdited,
                )
                .on_submit(TolMessage::FinishEditing)
                .padding(10);

                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(text_input)
                    .push(
                        Button::new(
                            delete_button,
                            Row::new()
                                .spacing(10)
                                .push(delete_icon())
                                .push(Text::new("Delete")),
                        )
                        .on_press(TolMessage::Delete)
                        .padding(10)
                        .style(style::Button::Destructive),
                    )
                    .into()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum EntryState {
    Idle {
        edit_button: button::State,
    },
    Editing {
        text_input: text_input::State,
        delete_button: button::State,
    },
}
impl Default for EntryState {
    fn default() -> Self {
        EntryState::Idle {
            edit_button: button::State::new(),
        }
    }
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

        let tolerance_text = TextInput::new(
            tolerance_text_state,
            "Tolerance name, press enter to add.",
            tolerance_text_value,
            Message::TolNameChanged,
            )
            .padding(15);

        let button = |state, label, tolerance: ToleranceTypes| {
            let label = Text::new(label).size(16);
            let button =
                Button::new(state, label).style(style::Button::Filter {
                    selected: tolerance == ToleranceTypes::Linear,
                });

            button.on_press(Message::TolTypeChanged(tolerance)).padding(8)
        };

        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .width(Length::Shrink)
                    .spacing(10)
                    .push(tolerance_text)
                    .push(button(
                        linear_button,
                        "Linear",
                        ToleranceTypes::Linear,
                    ))
                    .push(button(
                        float_button,
                        "Float",
                        ToleranceTypes::Float,
                    ))
                    .push(button(
                        compound_button,
                        "Compound",
                        ToleranceTypes::Compound,
                    )),
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
                    .push(filter_button(
                        compound_button,
                        "Compound",
                        Filter::Some(ToleranceTypes::Compound),
                        current_filter,
                    )),
            )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    filename: String,
    filter: Filter,
    simulation: SimulationState,
}

#[derive(Debug, Clone)]
pub enum TolMessage {
    Active(bool),
    Edit,
    DescriptionEdited(String),
    TolEdited(ToleranceType),
    FinishEditing,
    Delete,
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
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Filter { selected: bool },
        Icon,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            match self {
                Button::Filter { selected } => {
                    if *selected {
                        button::Style {
                            background: Some(Background::Color(
                                Color::from_rgb(0.2, 0.2, 0.7),
                            )),
                            border_radius: 10,
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
            }
        }

        fn hovered(&self) -> button::Style {
            let active = self.active();

            button::Style {
                text_color: match self {
                    Button::Icon => Color::from_rgb(0.2, 0.2, 0.7),
                    Button::Filter { selected } if !selected => {
                        Color::from_rgb(0.2, 0.2, 0.7)
                    }
                    _ => active.text_color,
                },
                shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
                ..active
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

/*
use num_format::{Locale, ToFormattedString};
use model::*;
use std::time::Instant;
use std::error::Error;
    let time_start = Instant::now();

    // Load model data
    let mut model = match deserialize_json("save") {
        Ok(result) => {
            println!("Loaded data from file");
            result
        },
        Err(error) => {
            println!("Error loading file:\n{}", error);
            println!("Loading placeholder data.");
            data()
        },
    };

    model.serialize_yaml("save")?;
    model.serialize_json("save")?;
    println!("{:.3?} to load data.", time_start.elapsed());

    let results = model::run(&model)?;

    let duration = time_start.elapsed();

    println!("Result: {:.3} +/- {:.3}; Stddev: {:.3};\nSamples: {}; Duration: {:.3?}", 
        results.mean, 
        results.tolerance, 
        results.stddev, 
        results.iterations.to_formatted_string(&Locale::en), 
        duration,
    );

    println!("Rate: {:.2} iter/μs; Timing: {:.2} ns/iter", 
        (results.iterations as f64)/(duration.as_micros() as f64),
        (duration.as_nanos() as f64)/(results.iterations as f64),
    );

    Ok(())
}
*/