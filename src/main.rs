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
use ui::style;

use iced::{
    Application, Column, Command, Container, Element, HorizontalAlignment, Length, Row, Settings, 
    Text, Subscription, window,
};
use colored::*;

use std::path::{Path, PathBuf};

fn main() {
    let mut settings = Settings::default();
    settings.window = window::Settings{
        size: (1024, 768),
        resizable: true,
        decorations: true,
    };
    settings.antialiasing = true;
    TolStack::run(settings);
}

// The state of the application
#[derive(Debug, Default, Clone)]
struct State {
    stylesheet: style::StyleSheet,
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
    //
    StyleUpdateAvailable(bool),
    LoadedStyle(Result<style::StyleSheet, style::LoadError>),
    StyleSaved(Result<(), style::SaveError>),
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
        println!("\n\n{}{}\n{:#?}", chrono::offset::Local::now(), " MESSAGE RECEIVED:".yellow(), message);
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
                        return Command::perform(style::StyleSheet::load(), Message::LoadedStyle)
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
                        return Command::perform(SavedState::open(), Message::Loaded)
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

                    Message::StyleUpdateAvailable(_) => {
                        return Command::perform(style::StyleSheet::load(), Message::LoadedStyle)
                    }

                    Message::LoadedStyle(Ok(stylesheet)) => {
                        state.stylesheet = stylesheet;
                    }

                    Message::LoadedStyle(Err(style::LoadError::FormatError)) => {
                        println!("\n\n{}{}", chrono::offset::Local::now(), " Error loading style file".red())
                    }

                    Message::LoadedStyle(Err(style::LoadError::FileError)) => {
                        return Command::perform(style::StyleSheet::save(state.stylesheet.clone()), Message::StyleSaved)
                    }

                    Message::StyleSaved(_) => {

                    }

                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                    }

                    Message::Loaded(Ok(save_state)) => {
                        *state = State {
                            stack_editor: StackEditor::new().tolerances(save_state.tolerances),
                            header: Header::new().title(save_state.name),
                            monte_carlo_analysis: MonteCarloAnalysis::new()
                                .set_inputs(save_state.n_iteration, save_state.assy_sigma),
                            ..State::default()
                        };
                    }

                    Message::Loaded(Err(_)) => {
                        println!("\n\n{}{}", chrono::offset::Local::now(), " Error loading save file".red())
                    }
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

    fn subscription(&self) -> Subscription<Self::Message> {
        match self {
            TolStack::Loading => Subscription::none(),
            TolStack::Loaded(State {
                stylesheet,
                header: _,
                stack_editor: _,
                monte_carlo_analysis: _,
                dirty: _,
                saving: _,
            }) => {
                stylesheet.check_style_file().map(Message::StyleUpdateAvailable)
            }
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view<'a>(&'a mut self) -> Element<Message> {
        match self {
            TolStack::Loading => loading_message(),
            TolStack::Loaded(State {
                stylesheet,
                header,
                stack_editor,
                monte_carlo_analysis,
                dirty: _,
                saving: _,
            }) => {
                let header = header.view(&stylesheet)
                    .map( move |message| { Message::HeaderMessage(message) });
                
                let stack_editor = stack_editor.view(&stylesheet)
                    .map( move |message| { Message::StackEditorMessage(message) });
                
                let monte_carlo_analysis = monte_carlo_analysis.view(&stylesheet)
                    .map( move |message| { Message::MonteCarloAnalysisMessage(message) });
                
                let gui = Container::new(
                    Column::new()
                        .push(header)
                        .push(Row::new()
                            .push(stack_editor)
                            .push(monte_carlo_analysis)
                        )
                        .padding(20)
                    )
                    .style(style::ContainerStyle::new(&stylesheet.background_container, &stylesheet));
                    
                
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