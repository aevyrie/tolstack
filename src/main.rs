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
    Description: String,
    scroll: scrollable::State,
    input: text_input::State,
    filter: Filter,
    simulation: SimulationState,
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
            //Command::perform(SavedState::load(), Message::Loaded),
            Command::none(),
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
                Command::none()
            }
            TolStack::Loaded(state) => {
                Command::none()
            }
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view(&mut self) -> Element<Message> {
        match self {
            TolStack::Loading => loading_message(),
            TolStack::Loaded(State {
                Description,
                scroll,
                input,
                filter,
                simulation,
                filter_controls,
                dirty,
                saving,
            }) => {
                let title = Text::new("TolStack")
                    .width(Length::Fill)
                    .size(100)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);
                
                let content = Column::new()
                    .max_width(800)
                    .spacing(20)
                    .push(title);

                Scrollable::new(scroll)
                    .padding(40)
                    .push(
                        Container::new(content).width(Length::Fill).center_x(),
                    )
                    .into()
            }
        }
    }
}

#[derive(Debug, Default)]
struct FilterControls {
    all_button: button::State,
    linear_button: button::State,
    float_button: button::State,
    compound_button: button::State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    filter: Filter,
    simulation: SimulationState,
}

#[derive(Debug, Clone)]
pub enum TolMessage {
    Active(bool),
    Edit,
    InputEdited(TolInput, String),
    Delete,
}

#[derive(Debug, Clone)]
pub enum TolInput {
    Name,
    Description,
    Dimension,
    Tolerance,
}

#[derive(Debug, Clone)]
pub enum Controls {
    SolvePressed,
    OpenFilePressed,
    SaveFilePressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Linear,
    Float,
    Compound,
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

fn loading_message() -> Element<'static, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
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