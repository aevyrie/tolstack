//#![windows_subsystem = "windows"] // Tells windows compiler not to show console window
mod ui;
mod analysis;

use ui::{
    style,
    components::{*},
};
use analysis::{
    monte_carlo,
};

use std::time::Instant;

use iced::{
    button, text_input, Align, Application, Button,
    Column, Command, Container, Element, HorizontalAlignment, Length,
    Row, Settings, Text, TextInput, window,
};
use serde_derive::*;


fn main() {
    let mut settings = Settings::default();
    settings.window = window::Settings{
        size: (1024, 768),
        resizable: true,
        decorations: true,
    };
    settings.antialiasing = false;
    TolStack::run(settings);
}

// The state of the application
#[derive(Debug, Default, Clone)]
struct StateApplication {
    header_area: Header,
    tolerance_input_area: ToleranceEntryList,

    button_state: button::State,
    iteration_state: text_input::State,
    sigma_state: text_input::State,
    simulation_state: monte_carlo::State,
    //simulation_result: monte_carlo::Results,
    result_output: String,
    dirty: bool,
    saving: bool,
    valid_stack: bool,
}
// Messages - events for users to change the application state
#[derive(Debug, Clone)]
enum Message {
    HeaderMessage(header::Message),
    ToleranceInputMessage(list_tolerance_entries::Message),
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    IterEdited(String),
    SigmaEdited(String),
    Calculate,
    CalculateComplete(Option<monte_carlo::Results>),
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
            TolStack::Loaded(state) => if state.header_area.title.text.len() == 0 {
                String::from("New Project")
            } else {
                state.header_area.title.text.clone()
            }};

        format!("{}{} - TolStack Tolerance Analysis", project_name, if dirty { "*" } else { "" })
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            TolStack::Loading => {
                match message {
                    // Take the loaded state and assign to the working state
                    Message::Loaded(Ok(state)) => {
                        *self = TolStack::Loaded(StateApplication {
                            simulation_state: state.simulation,
                            ..StateApplication::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = TolStack::Loaded(StateApplication {
                            ..StateApplication::default()
                        });
                    }
                    _ => {}
                }

                Command::none()
            }

            TolStack::Loaded(state) => {
                let mut saved = false;

                match message {
                    Message::HeaderMessage(message) => {
                        state.header_area.update(message);
                    }
                    Message::ToleranceInputMessage(message) => {
                        state.tolerance_input_area.update(message);
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }
                    Message::Calculate => {
                        return Command::perform(compute(state.clone()), Message::CalculateComplete)
                    }
                    Message::CalculateComplete(result) => {
                        match result {
                            Some(result) => state.simulation_state.results = result,
                            None => {}
                        }
                    }
                    Message::Loaded(_) => {
                    }
                    Message::IterEdited(input) => {
                        if input.parse::<usize>().is_ok() {
                            let mut number = input.parse::<usize>().unwrap();
                            if number < 100000 { number = 100000 };
                            state.simulation_state.parameters.n_iterations = number;
                        }
                    }
                    Message::SigmaEdited(input) => {
                        if input.parse::<f64>().is_ok() {
                            let mut number = input.parse::<f64>().unwrap();
                            if number <= 1.0 { number = 1.0 };
                            state.simulation_state.parameters.assy_sigma = number;
                        }
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
                            name: state.header_area.title.text.clone(),
                            tolerances: state.tolerance_input_area.tolerances.clone(),
                            simulation: state.simulation_state.clone(),
                            results: state.simulation_state.results.clone(),
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
                header_area,
                tolerance_input_area,
                button_state,
                iteration_state,
                sigma_state,
                simulation_state,
                result_output,
                dirty,
                saving,
                valid_stack,
            }) => {

                let results_header = Column::new()
                    .push(Row::new()
                        .push(Text::new("Simulation Parameters")
                            .size(24)
                            .width(Length::Fill))
                        .align_items(Align::Center)
                        .width(Length::Fill)
                    )
                    .push(Row::new()
                        .push(Text::new("Iterations"))
                        .push(
                            TextInput::new(
                                iteration_state,
                                "Enter a value...",
                                &simulation_state.parameters.n_iterations.to_string(),
                                Message::IterEdited,
                            )
                            .padding(10)
                        )
                        .align_items(Align::Center)
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Assembly Sigma"))
                        .push(
                            TextInput::new(
                                sigma_state,
                                "Enter a value...",
                                &simulation_state.parameters.assy_sigma.to_string(),
                                Message::SigmaEdited,
                            )
                            .padding(10)
                        )
                        .align_items(Align::Center)
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Column::new().width(Length::Fill))
                        .push(
                            Button::new( 
                                button_state, 
                                Row::new()
                                    .spacing(10)
                                    //.push(icons::check())
                                    .push(Text::new("Run Simulation")),
                            )
                            .style(style::Button::Constructive)
                            .padding(10)
                            .on_press(Message::Calculate)
                        )
                    )
                    .spacing(20);

                let results_body = Column::new()
                    .push(Row::new()
                        .push(Text::new("Mean:"))
                        .push(Text::new(format!("{:.3}",simulation_state.results.mean)))
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Tolerance:"))
                        .push(Text::new(format!("{:.3}",simulation_state.results.tolerance)))
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Standard Deviation:"))
                        .push(Text::new(format!("{:.3}",simulation_state.results.stddev)))
                        .spacing(20)
                    )
                    .push(Row::new()
                        .push(Text::new("Iterations:"))
                        .push(Text::new(format!("{}",simulation_state.results.iterations)))
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
                

                let header = header_area.view()
                    .map( move |message| { Message::HeaderMessage(message) });
                let tolerance_input_area = tolerance_input_area.view()
                    .map( move |message| { Message::ToleranceInputMessage(message) });
                
                let gui = Column::new()
                    .padding(20)
                    .push(header)
                    .push(Row::new()
                        .push(tolerance_input_area)
                        .push(tol_chain_output)
                    );
                
                //let gui = gui.explain(Color::BLACK);

                gui.into()
            }
        }
    }
}

/// Takes the application state, constructs a new tolerance model, and runs the simulation
async fn compute(mut state: StateApplication) -> Option<monte_carlo::Results> {
    state.simulation_state.clear();
    // Make sure all active entries are valid
    let mut valid = true;
    for entry in &state.tolerance_input_area.tolerances {
        if entry.active && !entry.valid { 
            valid = false;
        }
    }
    // Build the model
    if valid {
        for entry in &state.tolerance_input_area.tolerances {
            if entry.active {
                state.simulation_state.add(entry.analysis_model.clone());
            }
        }
    }


    //let time_start = Instant::now();
    let result = monte_carlo::run(&state.simulation_state).await.unwrap();
    /*
    let duration = time_start.elapsed();
    println!("Result: {:.3} +/- {:.3}; Stddev: {:.3};\nSamples: {}; Duration: {:.3?}", 
        state.simulation_result.mean, 
        state.simulation_result.tolerance, 
        state.simulation_result.stddev, 
        state.simulation_result.iterations,
        duration,
    );*/

    Some(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    name: String,
    tolerances: Vec<ToleranceEntry>,
    simulation: monte_carlo::State,
    results: monte_carlo::Results,
}

#[derive(Debug, Clone)]
pub enum Controls {
    SolvePressed,
    OpenFilePressed,
    SaveFilePressed,
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
