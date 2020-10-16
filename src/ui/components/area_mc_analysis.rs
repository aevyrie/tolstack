use crate::analysis::{monte_carlo, root_sum_square, structures};
use crate::ui::{components::*, style};
use iced::{Column, Command, Container, Element, Length, Row, Text};

#[derive(Debug, Clone)]
pub enum AnalysisAreaMessage {
    NewMcAnalysisMessage(form_new_mc_analysis::Message),
    CalculateComplete(Option<structures::AnalysisResults>),
    //RunRssCalcs(form_new_mc_analysis::Message),
    //RunMonteCarloCalcs(form_new_mc_analysis::Message),
    //RssCalcComplete(Option<structures::RssResults>),
    //MonteCarloCalcComplete(Option<structures::McResults>),
}

#[derive(Debug, Default, Clone)]
pub struct AnalysisState {
    pub entry_form: NewMonteCarloAnalysis,
    pub model_state: structures::State,
    pub input_stack: Vec<entry_tolerance::ToleranceEntry>,
}
impl AnalysisState {
    pub fn new() -> Self {
        AnalysisState::default()
    }
    pub fn update(&mut self, message: AnalysisAreaMessage) -> Command<AnalysisAreaMessage> {
        let AnalysisState {
            entry_form,
            model_state,
            input_stack: _,
        } = self;
        match message {
            AnalysisAreaMessage::NewMcAnalysisMessage(form_new_mc_analysis::Message::Calculate) => {
                let simulation_input = self.build_stack();
                if let Some(stack) = simulation_input {
                    return Command::perform(
                        AnalysisState::compute(stack),
                        AnalysisAreaMessage::CalculateComplete,
                    );
                }
            }
            AnalysisAreaMessage::NewMcAnalysisMessage(message) => {
                entry_form.update(message);
            }
            AnalysisAreaMessage::CalculateComplete(result) => {
                if let Some(result) = result {
                    model_state.results = result
                }
            }
        }
        Command::none()
    }
    pub fn view(&mut self, iss: &style::IcedStyleSheet) -> Element<AnalysisAreaMessage> {
        let AnalysisState {
            entry_form,
            model_state,
            input_stack: _,
        } = self;
        let mc_default = structures::McResults::default();
        let rss_default = structures::RssResults::default();

        let mc_results = match model_state.results.monte_carlo() {
            Some(mc_result) => mc_result,
            None => &mc_default,
        };

        let rss_results = match model_state.results.rss() {
            Some(rss_results) => rss_results,
            None => &rss_default,
        };

        let results_body = Column::new()
            .push(
                Row::new()
                    .push(Text::new("Mean:").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.mean))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Tolerance (+):").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.tolerance_pos))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Tolerance (-):").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.tolerance_neg))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Standard Deviation (+):").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.stddev_pos))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Standard Deviation (-):").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.stddev_neg))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Iterations:").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{}", mc_results.iterations))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Worst Case Lower:").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.worst_case_lower))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("Worst Case Upper:").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", mc_results.worst_case_upper))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("RSS Mean:").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", rss_results.mean()))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("RSS Tolerance (+):").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", rss_results.tolerance_pos()))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .push(
                Row::new()
                    .push(Text::new("RSS Tolerance (-):").size(iss.text_size(&iss.results)))
                    .push(
                        Text::new(format!("{:.2}", rss_results.tolerance_neg()))
                            .size(iss.text_size(&iss.results)),
                    )
                    .spacing(iss.spacing(&iss.mc_results_row_spacing)),
            )
            .spacing(iss.spacing(&iss.mc_results_col_spacing))
            .height(Length::Fill);

        let results_summary = Container::new(
            Column::new()
                .push(
                    entry_form
                        .view(&iss)
                        .map(move |message| AnalysisAreaMessage::NewMcAnalysisMessage(message)),
                )
                .push(results_body)
                .height(Length::Fill)
                .spacing(iss.spacing(&iss.mc_results_col_spacing)),
        )
        .padding(10);

        let tol_chain_output = Column::new()
            .push(
                Container::new(results_summary)
                    .style(iss.container(&iss.panel_container))
                    .padding(iss.padding(&iss.mc_results_container_inner_padding))
                    .height(Length::Fill)
                    .center_x(),
            )
            .height(Length::Fill)
            .width(Length::Fill);

        tol_chain_output.into()
    }

    fn build_stack(&mut self) -> Option<structures::State> {
        // Wipe the simulation input (tolerance stack) and build a new one
        self.model_state.clear_inputs();
        // Copy over the input parameterscalculate_message
        self.model_state.parameters.n_iterations = self.entry_form.n_iteration;
        self.model_state.parameters.assy_sigma = self.entry_form.assy_sigma;
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
                if entry.active {
                    self.model_state.add(entry.analysis_model.clone())
                }
            }
            Some(self.model_state.clone())
        } else {
            None
        }
    }

    /// Takes a monte carlo simulatio state, constructs a new tolerance model, and runs the simulation
    async fn compute(simulation: structures::State) -> Option<structures::AnalysisResults> {
        use std::time::Instant;
        // Each computation contains an owned simulation state, this allows multiple
        //  computations to be spawned independently, and run asynchronously
        let time_start = Instant::now();
        //todo - change unwrap to a match to prevent panic
        let mc_result = monte_carlo::run(&simulation).await.unwrap();
        let rss_result = root_sum_square::run(&simulation).await.unwrap();
        let duration = time_start.elapsed();
        println!("Simulation Duration: {:.3?}", duration,);
        let result = (mc_result, rss_result).into();
        Some(result)
    }

    pub fn set_inputs(&mut self, n_iterations: usize, assy_sigma: f64) -> Self {
        self.entry_form.n_iteration = n_iterations;
        self.entry_form.assy_sigma = assy_sigma;
        self.clone()
    }
}
