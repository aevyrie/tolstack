use iced::{
    Container, Element, Length, Row, Text, Column, Command,
};
use crate::ui::{ style, components::* };
use crate::analysis::monte_carlo;

#[derive(Debug, Clone)]
pub enum Message {
    NewMcAnalysisMessage(form_new_mc_analysis::Message),
    CalculateComplete(Option<monte_carlo::Results>),
}

#[derive(Debug, Default, Clone)]
pub struct MonteCarloAnalysis {
    pub entry_form: NewMonteCarloAnalysis,
    simulation: monte_carlo::State,
    pub input_stack: Vec<entry_tolerance::ToleranceEntry>,
}
impl MonteCarloAnalysis {

    pub fn new() -> Self {
        MonteCarloAnalysis::default()
    }
    pub fn update(&mut self, message: Message) -> Command<Message> {
        let MonteCarloAnalysis {
            entry_form,
            simulation,
            input_stack,
        } = self;
        match message {
            Message::NewMcAnalysisMessage(form_new_mc_analysis::Message::Calculate) => {
                let simulation_input = self.build_stack();
                if let Some(stack) = simulation_input {
                    return Command::perform(
                        MonteCarloAnalysis::compute(stack), Message::CalculateComplete
                    )
                }
            }
            Message::NewMcAnalysisMessage(message) => {
                entry_form.update(message);
            }
            Message::CalculateComplete(result) => {
                match result {
                    Some(result) => simulation.results = result,
                    None => {}
                }
            }
        }
        Command::none()
    }
    pub fn  view(&mut self, iss: &style::IcedStyleSheet) -> Element<Message> {
        let MonteCarloAnalysis {
            entry_form,
            simulation,
            input_stack,
        } = self;
        let results_body = Column::new()
            .push(Row::new()
                .push(Text::new("Mean:"))
                .push(Text::new(format!("{:.3}",simulation.results.mean)))
                .spacing(iss.spacing(&iss.mc_results_row_spacing))
            )
            .push(Row::new()
                .push(Text::new("Tolerance:"))
                .push(Text::new(format!("{:.3}",simulation.results.tolerance)))
                .spacing(iss.spacing(&iss.mc_results_row_spacing))
            )
            .push(Row::new()
                .push(Text::new("Standard Deviation:"))
                .push(Text::new(format!("{:.3}",simulation.results.stddev)))
                .spacing(iss.spacing(&iss.mc_results_row_spacing))
            )
            .push(Row::new()
                .push(Text::new("Iterations:"))
                .push(Text::new(format!("{}",simulation.results.iterations)))
                .spacing(iss.spacing(&iss.mc_results_row_spacing))
            )
            .spacing(iss.spacing(&iss.mc_results_col_spacing));

        let results_summary = Container::new(Column::new()
                .push(entry_form.view()
                    .map( move |message| { Message::NewMcAnalysisMessage(message) })
                )
                .push(results_body)
                .height(Length::Fill)
                .spacing(iss.spacing(&iss.mc_results_col_spacing))
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

    fn build_stack(&mut self) -> Option<monte_carlo::State> {
        // Wipe the simulation input (tolerance stack) and build a new one
        self.simulation.clear_inputs();
        // Copy over the input parameterscalculate_message
        self.simulation.parameters.n_iterations = self.entry_form.n_iteration;
        self.simulation.parameters.assy_sigma = self.entry_form.assy_sigma;
        // Make sure all active entries are valid
        let mut valid = true;
        for entry in &self.input_stack {
            if entry.active && !entry.valid { 
                valid = false;
            }
        }
        // Build the tolerance stack
        if valid {
            for entry in &self.input_stack {
                if entry.active { self.simulation.add(entry.analysis_model.clone()) }
            }
            Some(self.simulation.clone())
        } else {
            None
        }      
    }

    /// Takes a monte carlo simulatio state, constructs a new tolerance model, and runs the simulation
    async fn compute(simulation: monte_carlo::State) -> Option<monte_carlo::Results> {
        use std::time::Instant;
        // Each computation contains an owned simulation state, this allows multiple
        //  computations to be spawned independently, and run asynchronously
        let time_start = Instant::now();
        let result = monte_carlo::run(&simulation).await.unwrap();
        let duration = time_start.elapsed();
        println!("Simulation Duration: {:.3?}", duration,);
        //println!("Result: {:.3} +/- {:.3}; Stddev: {:.3};\nSamples: {}; Duration: {:.3?}", 
        //    simulation.results.mean, 
        //    simulation.results.tolerance, 
        //    simulation.results.stddev, 
        //    simulation.results.iterations,
        //    duration,
        //);

        Some(result)
    }

    pub fn set_inputs(&mut self, n_iterations: usize, assy_sigma: f64) -> Self {
        self.entry_form.n_iteration = n_iterations;
        self.entry_form.assy_sigma = assy_sigma;
        self.clone()
    }
}