use iced::{
    Align, Container, Element, HorizontalAlignment, Length, Row, Text, Column,
};
use crate::ui::{ components::* };
use crate::analysis::*;

#[derive(Debug, Clone)]
pub enum Message {
    NewMcAnalysisMessage(form_new_mc_analysis::Message),
    
}

#[derive(Debug, Default, Clone)]
pub struct MonteCarloAnalysis {
    entry_form: NewMonteCarloAnalysis
}
impl MonteCarloAnalysis {
    pub fn new() -> Self {
        AnalysisExplorer::default()
    }
    pub fn update(&mut self, message: Message) {
        let Header {
            title,
        } = self;
        match message {
            Message::Calculate => {
                return Command::perform(compute(state.clone()), Message::CalculateComplete)
            }
            Message::CalculateComplete(result) => {
                match result {
                    Some(result) => state.simulation_state.results = result,
                    None => {}
                }
            }
        }
    }
    pub fn  view(&mut self) -> Element<Message> {

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
        
        tol_chain_output.into()
    }
}

/// Takes the application state, constructs a new tolerance model, and runs the simulation
async fn compute(mut state: monte_carlo::State) -> Option<monte_carlo::Results> {
    state.clear();
    // Make sure all active entries are valid
    let mut valid = true;
    for entry in &state..tolerances {
        if entry.active && !entry.valid { 
            valid = false;
        }
    }
    // Build the model
    if valid {
        for entry in &state.stack_editor.tolerances {
            if entry.active {
                state.simulation.add(entry.analysis_model.clone());
            }
        }
    }


    //let time_start = Instant::now();
    let result = monte_carlo::run(&state.simulation).await.unwrap();
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