mod model;
mod tolerances;

use model::*;
use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox,
    Column, Command, Container, Element, Font, HorizontalAlignment, Length,
    Row, Scrollable, Settings, Text, TextInput,
};

fn main() {
    TolStack::run(Settings::default())
}

// Loading state wrapper
#[derive(Default)]
struct TolStack {
    Loading,
    Loaded(State)
}

// The state of the application
#[derive(Debug, Default)]
struct State {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    filter: Filter,
    simulation: SimulationState,
    controls: Controls,
    dirty: bool,
    saving: bool,
}

// Messages - events for users to change the application state
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTol,
    FilterChanged(Filter),
    TolMessage(usize, TolMessage),
    ProgramButtons,
}

#[derive(Debug, Clone)]
pub enum TolMessage {
    Active(bool),
    Edit,
    InputEdited(TolInput, String),
    Delete,
}

pub enum TolInput {
    Name,
    Description,
    Dimension,
    Tolerance,
}

pub enum ProgramButtons {
    SolvePressed,
    EditNamePressed(String),
    OpenFilePressed,
    SaveFilePressed,

}


impl Application for TolStack {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Stackup - New")
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) {
        match message {
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view(&mut self) -> Element<Message> {
    }
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