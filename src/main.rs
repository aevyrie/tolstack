//#![windows_subsystem = "windows"] // Tells windows compiler not to show console window

mod ui {
    pub mod components;
    pub mod icons;
    pub mod style;
}

mod analysis {
    pub mod monte_carlo;
    pub mod structures;
}

mod io {
    pub mod dialogs;
    pub mod saved_state;
}

use ui::components::*;
use io::saved_state::*;

use iced::{
    button, text_input, Align, Application, Button,
    Column, Command, Container, Element, HorizontalAlignment, Length,
    Row, Settings, Text, TextInput, window,
};



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
struct State {
    header: Header,
    stack_editor: StackEditor,
    monte_carlo_analysis: MonteCarloAnalysis,
    dirty: bool,
    saving: bool,
}
// Messages - events for users to change the application state
#[derive(Debug, Clone)]
enum Message {
    // Subcomponent messages
    HeaderMessage(area_header::Message),
    StackEditorMessage(area_stack_editor::Message),
    MonteCarloAnalysisMessage(area_mc_analysis::Message),
    // 
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
}

// Loading state wrapper
#[derive(Debug)]
enum TolStack {
    Loading,
    Loaded(State),
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
            TolStack::Loaded(state) => if state.header.title.text.len() == 0 {
                String::from("New Project")
            } else {
                state.header.title.text.clone()
            }};

        format!("{}{} - TolStack Tolerance Analysis", project_name, if dirty { "*" } else { "" })
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) -> Command<Message> {
        println!("\n\nMESSAGE RECEIVED:\n\n{:#?}", message);
        match self {
            TolStack::Loading => {
                match message {
                    // Take the loaded state and assign to the working state
                    Message::Loaded(Ok(state)) => {
                        *self = TolStack::Loaded(State {
                            stack_editor: StackEditor::new().tolerances(state.tolerances),
                            header: Header::new().title(state.name),
                            monte_carlo_analysis: MonteCarloAnalysis::new()
                                .set_inputs(state.n_iteration, state.assy_sigma),
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = TolStack::Loaded(State {
                            ..State::default()
                        });
                    }
                    _ => {}
                }

                Command::none()
            }

            TolStack::Loaded(state) => {
                let mut saved = false;

                match message {

                    Message::HeaderMessage(area_header::Message::OpenFile) => {
                        return Command::perform(SavedState::load(), Message::Loaded)
                    }

                    Message::HeaderMessage(message) => {
                        state.header.update(message)
                    }

                    Message::StackEditorMessage(message) => {
                        state.stack_editor.update(message)
                    }
                    
                    Message::MonteCarloAnalysisMessage(
                        area_mc_analysis::Message::NewMcAnalysisMessage(
                            form_new_mc_analysis::Message::Calculate
                        )
                    ) => {
                        // Clone the contents of the stack editor tolerance list into the monte
                        // carlo simulation's input tolerance list.
                        state.monte_carlo_analysis.input_stack = state.stack_editor.tolerances.clone();
                        // Pass this message into the child so the computation gets kicked off.
                        let calculate_message = area_mc_analysis::Message::NewMcAnalysisMessage(
                            form_new_mc_analysis::Message::Calculate
                        );
                        return state.monte_carlo_analysis.update(calculate_message)
                            .map( move |message| { Message::MonteCarloAnalysisMessage(message) })
                    }

                    Message::MonteCarloAnalysisMessage(message) => {
                        // TODO collect commands and run at end instead of breaking at match arm.
                        return state.monte_carlo_analysis.update(message)
                            .map( move |message| { Message::MonteCarloAnalysisMessage(message) })
                    }

                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }

                    Message::Loaded(_) => {}
                }

                if !saved { state.dirty = true }

                if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            name: state.header.title.text.clone(),
                            tolerances: state.stack_editor.tolerances.clone(),
                            n_iteration: state.monte_carlo_analysis.entry_form.n_iteration,
                            assy_sigma: state.monte_carlo_analysis.entry_form.assy_sigma,
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
                header,
                stack_editor,
                monte_carlo_analysis,
                dirty: _,
                saving: _,
            }) => {
                let header = header.view()
                    .map( move |message| { Message::HeaderMessage(message) });
                
                let stack_editor = stack_editor.view()
                    .map( move |message| { Message::StackEditorMessage(message) });
                
                let monte_carlo_analysis = monte_carlo_analysis.view()
                    .map( move |message| { Message::MonteCarloAnalysisMessage(message) });
                
                let gui = Column::new()
                    .push(header)
                    .push(Row::new()
                        .push(stack_editor)
                        .push(monte_carlo_analysis)
                    )
                    .padding(20);
                
                //debug:
                //let gui = gui.explain(Color::BLACK);

                gui.into()
            }
        }
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