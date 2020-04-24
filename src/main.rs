mod simulation_model;
mod tolerances;
mod ui;

use num_format::{Locale, ToFormattedString};
use statistical::*;
use simulation_model::*;
use std::time::Instant;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    //ui::run();

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

    let results = run_model(&model)?;

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

pub struct ModelResults {
    pub mean: f64,
    pub tolerance: f64,
    pub stddev: f64,
    pub iterations: usize,
}

fn run_model(model: &SimulationModel) -> Result<ModelResults,Box<dyn Error>> {
    // Divide the desired number of iterations into chunks. This is done [1] to avoid floating point
    //  errors (as the divisor gets large when averaging you lose precision) and [2] to prevent huge 
    //  memory use for large numbers of iterations. This can also be used to tune performance.
    let chunk_size = 100000;
    let chunks = model.params.n_iterations/chunk_size;
    let real_iters = chunks * chunk_size;
    let mut result_mean = 0f64;
    let mut result_stddev = 0f64;
    for n in 0..chunks {
        // TODO: validate n_iterations is nicely divisible by chunk_size and n_threads.
        // Gather samples into a stack that is `chunk_size` long for each ToleranceType
        let stack = compute_stackup(model.tolerance_loop.clone(), chunk_size);
        // Sum each
        let stack_mean = mean(&stack);
        let stack_stddev = standard_deviation(&stack, Some(stack_mean));
        // Weighted average
        result_mean = result_mean*(n as f64/(n as f64 + 1.0)) + stack_mean*(1.0/(n as f64 + 1.0));
        result_stddev = result_stddev*(n as f64/(n as f64 + 1.0)) + stack_stddev*(1.0/(n as f64 + 1.0));
        serialize_csv(stack, "data.csv")?;
    }
    let result_tol = result_stddev * model.params.assy_sigma;

    Ok(ModelResults{
        mean: result_mean,
        tolerance: result_tol,
        stddev: result_stddev,
        iterations: real_iters,
    })
}