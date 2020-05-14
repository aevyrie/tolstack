use iced::{
    button, text_input, Align, Button, HorizontalAlignment, Length, Element,
    Row, Column, Text, TextInput, Command,
};
use crate::ui::{ style };
use crate::analysis::{
    structures::*,
};

#[derive(Debug, Clone)]
enum Message {
    IterEdited(String),
    SigmaEdited(String),
    Calculate,
    CalculateComplete(Option<monte_carlo::Results>),
}

#[derive(Debug, Default, Clone)]
pub struct NewMonteCarloAnalysis {
    n_iteration: usize,
    assy_sigma: f64,
    state_calculate_button: button::State,
    state_input_assy_sigma: text_input::State,
    state_input_iterations: text_input::State,
}
impl NewMonteCarloAnalysis {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IterEdited(input) => {
                if input.parse::<usize>().is_ok() {
                    let mut number = input.parse::<usize>().unwrap();
                    if number < 100000 { number = 100000 };
                    self.n_iteration = number;
                }
            }
            Message::SigmaEdited(input) => {
                if input.parse::<f64>().is_ok() {
                    let mut number = input.parse::<f64>().unwrap();
                    if number <= 1.0 { number = 1.0 };
                    self.assy_sigma = number;
                }
            }
            Message::Calculate => {}
            Message::CalculateComplete(_) => {}
        }
        Command::none()
    }
    pub fn view(&mut self) -> Element<Message> {
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

        results_header.into()
    }
}