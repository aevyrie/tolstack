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

use analysis::structures::*;
use io::{export_csv, saved_state::*};
use ui::{components::*, style};

use colored::*;
use iced::{
    keyboard, text_input, time, window, Application, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};
use image::GenericImageView;

use std::path::PathBuf;

fn main() {
    let bytes = include_bytes!("ui/icon.png");
    let img = image::load_from_memory(bytes).unwrap();
    let img_dims = img.dimensions();
    let img_raw = img.into_rgba8().into_raw();
    let icon = window::Icon::from_rgba(img_raw, img_dims.0, img_dims.1).unwrap();

    let mut settings = Settings::default();
    settings.window = window::Settings {
        size: (1024, 768),
        resizable: true,
        decorations: true,
        min_size: Some((800, 600)),
        max_size: None,
        transparent: false,
        always_on_top: false,
        icon: Some(icon),
    };
    settings.antialiasing = true;
    TolStack::run(settings).unwrap();
}

// The state of the application
#[derive(Debug, Clone)]
struct State {
    last_save: std::time::Instant,
    iss: style::IcedStyleSheet,
    header: Header,
    stack_editor: StackEditor,
    analysis_state: AnalysisState,
    dirty: bool,
    saving: bool,
    file_path: Option<PathBuf>,
}
impl Default for State {
    fn default() -> Self {
        State {
            last_save: std::time::Instant::now(),
            iss: style::IcedStyleSheet::default(),
            header: Header::default(),
            stack_editor: StackEditor::default(),
            analysis_state: AnalysisState::default(),
            dirty: false,
            saving: false,
            file_path: None,
        }
    }
}
impl State {
    /// Marks the state as having unsaved changes
    fn mark_unsaved_changes(&mut self) {
        self.dirty = true;
    }
    fn stack_is_not_empty(&self) -> bool {
        self.stack_editor
            .tolerances
            .iter()
            .filter(|x| x.active)
            .count()
            > 0
    }
}

// Messages - events for users to change the application state
#[derive(Debug, Clone)]
enum Message {
    // Subcomponent messages
    Header(HeaderAreaMessage),
    StackEditor(StackEditorAreaMessage),
    Analysis(AnalysisAreaMessage),
    //
    AutoSave,
    Loaded(Result<(Option<PathBuf>, SavedState), io::saved_state::LoadError>),
    Saved(Result<Option<PathBuf>, io::saved_state::SaveError>),
    ExportComplete(Result<(), io::export_csv::SaveError>),
    EventOccurred(iced_native::Event),
    //
    StyleUpdateAvailable(bool),
    LoadedStyle(Result<style::IcedStyleSheet, style::LoadError>),
    StyleSaved(Result<(), style::SaveError>),
    //
    HelpOpened,
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
                if state.stack_editor.title.text.is_empty() {
                    String::from("New Stack")
                } else {
                    state.stack_editor.title.text.clone()
                }
            }
        };
        let path_str = match self {
            TolStack::Loading => String::from(""),
            TolStack::Loaded(state) => match &state.file_path {
                Some(path) => match path.to_str() {
                    Some(str) => format!(" - {}", String::from(str)),
                    None => String::from(""),
                },
                None => String::from(""),
            },
        };

        format!(
            " {}{}{} - TolStack Tolerance Analysis",
            project_name,
            if dirty { "*" } else { "" },
            path_str,
        )
    }

    // Update logic - how to react to messages sent through the application
    fn update(&mut self, message: Message) -> Command<Message> {
        let is_event = if let Message::EventOccurred(_) = message {
            true
        } else {
            false
        };
        if cfg!(debug_assertions) && !is_event {
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
                            stack_editor: StackEditor::new()
                                .tolerances(state.tolerances)
                                .title(state.name),
                            header: Header::new(),
                            analysis_state: AnalysisState::new()
                                .set_inputs(state.n_iteration, state.assy_sigma),
                            file_path: path,
                            dirty: false,
                            saving: false,
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
                match message {
                    Message::EventOccurred(iced_native::Event::Keyboard(event)) => match event {
                        keyboard::Event::KeyPressed {
                            key_code,
                            modifiers: _,
                        } => {
                            if key_code == keyboard::KeyCode::Tab {
                                for entry in &mut state.stack_editor.tolerances {
                                    match &mut entry.state {
                                        entry_tolerance::State::Idle {
                                            button_edit: _,
                                            button_move_up: _,
                                            button_move_down: _,
                                        } => {}
                                        entry_tolerance::State::Editing { form_tolentry } => {
                                            match form_tolentry {
                                                FormState::Linear {
                                                    button_save: _,
                                                    button_delete: _,
                                                    description,
                                                    dimension,
                                                    tolerance_pos,
                                                    tolerance_neg,
                                                    sigma,
                                                } => {
                                                    if description.is_focused() {
                                                        *description = text_input::State::default();
                                                        *dimension = text_input::State::focused();
                                                    } else if dimension.is_focused() {
                                                        *dimension = text_input::State::default();
                                                        *tolerance_pos =
                                                            text_input::State::focused();
                                                    } else if tolerance_pos.is_focused() {
                                                        *tolerance_pos =
                                                            text_input::State::default();
                                                        *tolerance_neg =
                                                            text_input::State::focused();
                                                    } else if tolerance_neg.is_focused() {
                                                        *tolerance_neg =
                                                            text_input::State::default();
                                                        *sigma = text_input::State::focused();
                                                    } else if sigma.is_focused() {
                                                        *sigma = text_input::State::default();
                                                        *description = text_input::State::focused();
                                                    }
                                                }
                                                FormState::Float {
                                                    button_save: _,
                                                    button_delete: _,
                                                    description,
                                                    diameter_hole,
                                                    diameter_pin,
                                                    tolerance_hole_pos,
                                                    tolerance_hole_neg,
                                                    tolerance_pin_pos,
                                                    tolerance_pin_neg,
                                                    sigma,
                                                } => {
                                                    if description.is_focused() {
                                                        *description = text_input::State::default();
                                                        *diameter_hole =
                                                            text_input::State::focused();
                                                    } else if diameter_hole.is_focused() {
                                                        *diameter_hole =
                                                            text_input::State::default();
                                                        *tolerance_hole_pos =
                                                            text_input::State::focused();
                                                    } else if tolerance_hole_pos.is_focused() {
                                                        *tolerance_hole_pos =
                                                            text_input::State::default();
                                                        *tolerance_hole_neg =
                                                            text_input::State::focused();
                                                    } else if tolerance_hole_neg.is_focused() {
                                                        *tolerance_hole_neg =
                                                            text_input::State::default();
                                                        *diameter_pin =
                                                            text_input::State::focused();
                                                    } else if diameter_pin.is_focused() {
                                                        *diameter_pin =
                                                            text_input::State::default();
                                                        *tolerance_pin_pos =
                                                            text_input::State::focused();
                                                    } else if tolerance_pin_pos.is_focused() {
                                                        *tolerance_pin_pos =
                                                            text_input::State::default();
                                                        *tolerance_pin_neg =
                                                            text_input::State::focused();
                                                    } else if tolerance_pin_neg.is_focused() {
                                                        *tolerance_pin_neg =
                                                            text_input::State::default();
                                                        *sigma = text_input::State::focused();
                                                    } else if sigma.is_focused() {
                                                        *sigma = text_input::State::default();
                                                        *description = text_input::State::focused();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                            }
                        }
                        _ => {}
                    },
                    Message::EventOccurred(_) => {}
                    Message::AutoSave => {
                        if let Some(path) = &state.file_path {
                            state.saving = true;
                            let save_data = SavedState {
                                name: state.stack_editor.title.text.clone(),
                                tolerances: state.stack_editor.tolerances.clone(),
                                n_iteration: state.analysis_state.entry_form.n_iteration,
                                assy_sigma: state.analysis_state.entry_form.assy_sigma,
                            };
                            return Command::perform(
                                SavedState::save(save_data, path.clone()),
                                Message::Saved,
                            );
                        } else {
                            return Command::none();
                        }
                    }
                    Message::Header(area_header::HeaderAreaMessage::NewFile) => {
                        return Command::perform(SavedState::new(), Message::Loaded)
                    }
                    Message::Header(area_header::HeaderAreaMessage::OpenFile) => {
                        return Command::perform(SavedState::open(), Message::Loaded)
                    }

                    Message::Header(area_header::HeaderAreaMessage::SaveFile) => {
                        let save_data = SavedState {
                            name: state.stack_editor.title.text.clone(),
                            tolerances: state.stack_editor.tolerances.clone(),
                            n_iteration: state.analysis_state.entry_form.n_iteration,
                            assy_sigma: state.analysis_state.entry_form.assy_sigma,
                        };

                        match &state.file_path {
                            Some(path) => {
                                return Command::perform(
                                    SavedState::save(save_data, path.clone()),
                                    Message::Saved,
                                )
                            }
                            None => {
                                return Command::perform(
                                    SavedState::save_as(save_data),
                                    Message::Saved,
                                )
                            }
                        };
                    }

                    Message::Header(area_header::HeaderAreaMessage::SaveAsFile) => {
                        let save_data = SavedState {
                            name: state.stack_editor.title.text.clone(),
                            tolerances: state.stack_editor.tolerances.clone(),
                            n_iteration: state.analysis_state.entry_form.n_iteration,
                            assy_sigma: state.analysis_state.entry_form.assy_sigma,
                        };

                        return Command::perform(SavedState::save_as(save_data), Message::Saved);
                    }

                    Message::Header(area_header::HeaderAreaMessage::ExportCSV) => {
                        return Command::perform(
                            export_csv::serialize_csv(
                                state.analysis_state.model_state.results.export(),
                            ),
                            Message::ExportComplete,
                        )
                    }

                    Message::Header(area_header::HeaderAreaMessage::AddTolLinear) => {
                        state.mark_unsaved_changes();
                        state.stack_editor.update(
                            area_stack_editor::StackEditorAreaMessage::NewEntryMessage((
                                String::from("New Linear Tolerance"),
                                Tolerance::Linear(LinearTL::default()),
                            )),
                        )
                    }

                    Message::Header(area_header::HeaderAreaMessage::AddTolFloat) => {
                        state.mark_unsaved_changes();
                        state.stack_editor.update(
                            area_stack_editor::StackEditorAreaMessage::NewEntryMessage((
                                String::from("New Float Tolerance"),
                                Tolerance::Float(FloatTL::default()),
                            )),
                        )
                    }

                    Message::Header(area_header::HeaderAreaMessage::Help) => {
                        return Command::perform(help(), |_| Message::HelpOpened);
                    }

                    Message::HelpOpened => {}

                    Message::ExportComplete(_) => {}

                    Message::StackEditor(message) => {
                        let recompute = match message {
                            area_stack_editor::StackEditorAreaMessage::LabelMessage(
                                editable_label::Message::FinishEditing,
                            ) => true,
                            area_stack_editor::StackEditorAreaMessage::EntryMessage(_, _) => true,
                            _ => false,
                        };
                        state.stack_editor.update(message);
                        if recompute {
                            state.mark_unsaved_changes();
                            return Command::perform(do_nothing(), |_| {
                                Message::Analysis(
                                    area_mc_analysis::AnalysisAreaMessage::NewMcAnalysisMessage(
                                        form_new_mc_analysis::Message::Calculate,
                                    ),
                                )
                            });
                        }
                    }

                    Message::Analysis(
                        area_mc_analysis::AnalysisAreaMessage::NewMcAnalysisMessage(
                            form_new_mc_analysis::Message::Calculate,
                        ),
                    ) => {
                        if state.stack_is_not_empty() {
                            // Clone the contents of the stack editor tolerance list into the monte
                            // carlo simulation's input tolerance list.
                            state.analysis_state.input_stack =
                                state.stack_editor.tolerances.clone();
                            // Pass this message into the child so the computation gets kicked off.
                            let calculate_message =
                                area_mc_analysis::AnalysisAreaMessage::NewMcAnalysisMessage(
                                    form_new_mc_analysis::Message::Calculate,
                                );
                            return state
                                .analysis_state
                                .update(calculate_message)
                                .map(Message::Analysis);
                        }
                    }

                    Message::Analysis(message) => {
                        // TODO collect commands and run at end instead of breaking at match arm.
                        return state.analysis_state.update(message).map(Message::Analysis);
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
                        state.saving = false;
                        match save_result {
                            Ok(path_result) => {
                                if let Some(path) = path_result {
                                    state.file_path = Some(path);
                                    state.last_save = std::time::Instant::now();
                                }
                                state.dirty = false;
                            }
                            Err(e) => {
                                state.dirty = true;
                                println!("Save failed with {:?}", e);
                            }
                        }
                    }

                    Message::Loaded(Ok((path, save_state))) => {
                        *state = State {
                            stack_editor: StackEditor::new()
                                .tolerances(save_state.tolerances)
                                .title(save_state.name),
                            header: Header::new(),
                            analysis_state: AnalysisState::new()
                                .set_inputs(save_state.n_iteration, save_state.assy_sigma),
                            file_path: path,
                            dirty: false,
                            saving: false,
                            ..State::default()
                        };
                    }

                    Message::Loaded(Err(_)) => println!(
                        "\n\n{}{}",
                        chrono::offset::Local::now(),
                        " Error loading save file".red()
                    ),
                }

                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        match self {
            TolStack::Loading => Subscription::none(),
            TolStack::Loaded(State {
                last_save: _,
                iss,
                header: _,
                stack_editor: _,
                analysis_state: _,
                dirty,
                saving,
                file_path,
            }) => {
                let auto_save = if *dirty && !saving && file_path.is_some()
                //&& last_save.elapsed().as_secs() > 5
                {
                    time::every(std::time::Duration::from_secs(5)).map(|_| Message::AutoSave)
                } else {
                    Subscription::none()
                };
                let style_reload = if cfg!(debug_assertions) {
                    iss.check_style_file().map(Message::StyleUpdateAvailable)
                } else {
                    Subscription::none()
                };
                let tab_field = iced_native::subscription::events().map(Message::EventOccurred);

                Subscription::batch(vec![auto_save, style_reload, tab_field])
            }
        }
    }

    // View logic - a way to display the state of the application as widgets that can produce messages
    fn view(&mut self) -> Element<Message> {
        match self {
            TolStack::Loading => loading_message(),
            TolStack::Loaded(State {
                last_save: _,
                iss,
                header,
                stack_editor,
                analysis_state,
                dirty: _,
                saving: _,
                file_path: _,
            }) => {
                let header = header.view(&iss).map(Message::Header);

                let stack_editor = stack_editor.view(&iss).map(Message::StackEditor);

                let analysis_state = analysis_state.view(&iss).map(Message::Analysis);

                let content = Column::new().push(
                    Row::new()
                        .push(
                            Container::new(stack_editor)
                                .padding(iss.padding(&iss.home_padding))
                                .width(Length::Fill),
                        )
                        .push(Container::new(analysis_state).width(Length::Units(400))),
                );

                let gui: Element<_> = Container::new(Column::new().push(header).push(content))
                    .style(iss.container(&iss.home_container))
                    .into();

                gui.explain(iced::Color::BLACK)
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

async fn help() {
    webbrowser::open("https://aevyrie.github.io/tolstack/book/getting-started").unwrap();
}

async fn do_nothing() {}
