//#![windows_subsystem = "windows"] // Tells windows compiler not to show console window

mod ui {
    pub mod components;
    pub mod icons;
    pub mod style;
}

mod analysis {
    pub mod monte_carlo;
    pub mod root_sum_square;
    pub mod structures;
}

mod io {
    pub mod dialogs;
    pub mod export_csv;
    pub mod saved_state;
}

use io::{export_csv, saved_state::*};
use ui::{components::*, style, style::*};

use colored::*;
use iced::{
    window, Application, Column, Command, Container, Element, HorizontalAlignment, Length, Row,
    Settings, Subscription, Text,
};

use std::path::PathBuf;

fn main() {
    let mut settings = Settings::default();
    settings.window = window::Settings {
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
    iss: style::IcedStyleSheet,
    header: Header,
    stack_editor: StackEditor,
    analysis_state: AnalysisState,
    dirty: bool,
    saving: bool,
    file_path: Option<PathBuf>,
}
// Messages - events for users to change the application state
#[derive(Debug, Clone)]
enum Message {
    // Subcomponent messages
    HeaderMessage(area_header::Message),
    StackEditorMessage(area_stack_editor::Message),
    MonteCarloAnalysisMessage(area_mc_analysis::Message),
    //
    Loaded(Result<(Option<PathBuf>, SavedState), io::saved_state::LoadError>),
    Saved(Result<Option<PathBuf>, io::saved_state::SaveError>),
    ExportComplete(Result<(), io::export_csv::SaveError>),
    //
    StyleUpdateAvailable(bool),
    LoadedStyle(Result<style::IcedStyleSheet, style::LoadError>),
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
            Command::perform(SavedState::new(), Message::Loaded),
            //Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            TolStack::Loading => false,
            TolStack::Loaded(state) => state.dirty,
        };
        let project_name = match self {
            TolStack::Loading => String::from("Loading..."),
            TolStack::Loaded(state) => {
                if state.header.title.text.len() == 0 {
                    String::from("New Project")
                } else {
                    state.header.title.text.clone()
                }
            }
        };
        let path_str = match self {
            TolStack::Loading => String::from(""),
            TolStack::Loaded(state) => {
                match &state.file_path {
                    Some(path) => {
                        match path.to_str() {
                            Some(str) => format!(" - {}",String::from(str)),
                            None => String::from(""),
                        }
                    }
                    None => String::from("")
                }
            }
        };

        format!(
            "{}{}{} - TolStack Tolerance Analysis",
            project_name,
            if dirty { "*" } else { "" },
            path_str,
        )
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) -> Command<Message> {
        if cfg!(debug_assertions) {
            println!(
                "\n\n{}{}\n{:#?}",
                chrono::offset::Local::now(),
                " MESSAGE RECEIVED:".yellow(),
                message
            );
        }
        match self {
            TolStack::Loading => {
                match message {
                    // Take the loaded state and assign to the working state
                    Message::Loaded(Ok((path, state))) => {
                        *self = TolStack::Loaded(State {
                            stack_editor: StackEditor::new().tolerances(state.tolerances),
                            header: Header::new().title(state.name),
                            analysis_state: AnalysisState::new()
                                .set_inputs(state.n_iteration, state.assy_sigma),
                            file_path: path,
                            ..State::default()
                        });

                        if cfg!(debug_assertions) {
                            return Command::perform(
                                style::IcedStyleSheet::load(),
                                Message::LoadedStyle,
                            );
                        } else {
                            return Command::none();
                        }
                    }

                    Message::Loaded(Err(_)) => {
                        *self = TolStack::Loaded(State { ..State::default() });
                    }
                    _ => {}
                }

                Command::none()
            }

            TolStack::Loaded(state) => {
                let mut saved = false;

                match message {
                    Message::HeaderMessage(area_header::Message::NewFile) => {
                        return Command::perform(SavedState::new(), Message::Loaded)
                    }
                    Message::HeaderMessage(area_header::Message::OpenFile) => {
                        return Command::perform(SavedState::open(), Message::Loaded)
                    }

                    Message::HeaderMessage(area_header::Message::SaveFile) => {
                        let save_data = SavedState {
                            name: state.header.title.text.clone(),
                            tolerances: state.stack_editor.tolerances.clone(),
                            n_iteration: state.analysis_state.entry_form.n_iteration,
                            assy_sigma: state.analysis_state.entry_form.assy_sigma,
                        };

                        match &state.file_path {
                            Some(path) => {
                                return Command::perform(
                                    save_data.save(path.clone()),
                                    Message::Saved,
                                )
                            }
                            None => return Command::perform(save_data.save_as(), Message::Saved),
                        };
                    }

                    Message::HeaderMessage(area_header::Message::SaveAsFile) => {
                        let save_data = SavedState {
                            name: state.header.title.text.clone(),
                            tolerances: state.stack_editor.tolerances.clone(),
                            n_iteration: state.analysis_state.entry_form.n_iteration,
                            assy_sigma: state.analysis_state.entry_form.assy_sigma,
                        };

                        return Command::perform(save_data.save_as(), Message::Saved)
                    }

                    Message::HeaderMessage(area_header::Message::ExportCSV) => {
                        return Command::perform(
                            export_csv::serialize_csv(
                                state.analysis_state.model_state.results.export(),
                            ),
                            Message::ExportComplete,
                        )
                    }

                    Message::ExportComplete(_) => {}

                    Message::HeaderMessage(message) => state.header.update(message),

                    Message::StackEditorMessage(message) => state.stack_editor.update(message),

                    Message::MonteCarloAnalysisMessage(
                        area_mc_analysis::Message::NewMcAnalysisMessage(
                            form_new_mc_analysis::Message::Calculate,
                        ),
                    ) => {
                        // Clone the contents of the stack editor tolerance list into the monte
                        // carlo simulation's input tolerance list.
                        state.analysis_state.input_stack = state.stack_editor.tolerances.clone();
                        // Pass this message into the child so the computation gets kicked off.
                        let calculate_message = area_mc_analysis::Message::NewMcAnalysisMessage(
                            form_new_mc_analysis::Message::Calculate,
                        );
                        return state
                            .analysis_state
                            .update(calculate_message)
                            .map(move |message| Message::MonteCarloAnalysisMessage(message));
                    }

                    Message::MonteCarloAnalysisMessage(message) => {
                        // TODO collect commands and run at end instead of breaking at match arm.
                        return state
                            .analysis_state
                            .update(message)
                            .map(move |message| Message::MonteCarloAnalysisMessage(message));
                    }

                    Message::StyleUpdateAvailable(_) => {
                        return Command::perform(
                            style::IcedStyleSheet::load(),
                            Message::LoadedStyle,
                        )
                    }

                    Message::LoadedStyle(Ok(iss)) => {
                        state.iss = iss;
                    }

                    Message::LoadedStyle(Err(style::LoadError::FormatError)) => println!(
                        "\n\n{}{}",
                        chrono::offset::Local::now(),
                        " Error loading style file".red()
                    ),

                    Message::LoadedStyle(Err(style::LoadError::FileError)) => {
                        return Command::perform(
                            style::IcedStyleSheet::save(state.iss.clone()),
                            Message::StyleSaved,
                        )
                    }

                    Message::StyleSaved(_) => {}

                    Message::Saved(save_result) => {
                        if let Ok(path_result) = save_result {
                            state.saving = false;
                            saved = true;
                            if let Some(path) = path_result {
                                state.file_path = Some(path);
                            }
                        }
                    }

                    Message::Loaded(Ok((path, save_state))) => {
                        *state = State {
                            stack_editor: StackEditor::new().tolerances(save_state.tolerances),
                            header: Header::new().title(save_state.name),
                            analysis_state: AnalysisState::new()
                                .set_inputs(save_state.n_iteration, save_state.assy_sigma),
                            file_path: path,
                            ..State::default()
                        };
                    }

                    Message::Loaded(Err(_)) => println!(
                        "\n\n{}{}",
                        chrono::offset::Local::now(),
                        " Error loading save file".red()
                    ),
                }

                if !saved {
                    state.dirty = true
                }

                if state.dirty && !state.saving {
                    if let Some(path) = &state.file_path {
                        state.dirty = false;
                        state.saving = true;
                        Command::perform(
                            SavedState {
                                name: state.header.title.text.clone(),
                                tolerances: state.stack_editor.tolerances.clone(),
                                n_iteration: state.analysis_state.entry_form.n_iteration,
                                assy_sigma: state.analysis_state.entry_form.assy_sigma,
                            }
                            .save(path.clone()),
                            Message::Saved,
                        )
                    } else {
                        Command::none()
                    }
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
                iss,
                header: _,
                stack_editor: _,
                analysis_state: _,
                dirty: _,
                saving: _,
                file_path: _,
            }) => {
                if cfg!(debug_assertions) {
                    iss.check_style_file().map(Message::StyleUpdateAvailable)
                } else {
                    Subscription::none()
                }
            }
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view<'a>(&'a mut self) -> Element<Message> {
        match self {
            TolStack::Loading => loading_message(),
            TolStack::Loaded(State {
                iss,
                header,
                stack_editor,
                analysis_state,
                dirty: _,
                saving: _,
                file_path: _,
            }) => {
                let header = header
                    .view(&iss)
                    .map(move |message| Message::HeaderMessage(message));

                let stack_editor = stack_editor
                    .view(&iss)
                    .map(move |message| Message::StackEditorMessage(message));

                let analysis_state = analysis_state
                    .view(&iss)
                    .map(move |message| Message::MonteCarloAnalysisMessage(message));

                let content = Column::new()
                    .push(Row::new().push(stack_editor).push(analysis_state))
                    .padding(iss.padding(&iss.home_padding));

                let gui = Container::new(Column::new().push(header).push(content))
                    .style(iss.container(&iss.home_container));

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
