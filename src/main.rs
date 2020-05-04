mod model;
mod tolerances;

use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};

fn main() {
    StackupApp::run(Settings::default())
}

// The state of the application
#[derive(Default)]
struct StackupApp {
    // The counter value
    value: i32,

    // The local state of the two buttons
    new_entry: button::State,
    delete_entry: button::State,
    solve: button::State,
}

// Messages - events for users to change the application state
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    NewEntryPressed,
    EditEntyPressed,
    DeleteEntryPressed,
    SolvePressed,
    EditNamePressed,
    OpenFilePressed,
    SaveFilePressed,
}


impl Application for StackupApp {
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
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
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