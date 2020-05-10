use crate::tolerances::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::thread;
use std::sync::mpsc;

use csv::Writer;
use num::clamp;
use serde_derive::*;
use serde_json;
use rand::prelude::*;
use rand_distr::StandardNormal;
use statistical::*;

/// Structure used to hold simulation input parameters
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimulationParams{
    assy_sigma: f64,
    n_iterations: usize,
}

/// Structure used to hold the output of a simulaion
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelResults {
    pub mean: f64,
    pub tolerance: f64,
    pub stddev: f64,
    pub iterations: usize,
}
impl ModelResults {
    pub fn new() -> Self {
        // TODO: change this to use the Defaults derive
        ModelResults {
            mean: 0.0,
            tolerance: 0.0,
            stddev: 0.0,
            iterations: 0,
        }
    }
}

/// Holds the working state of the simulation, including inputs and outputs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimulationState {
    parameters: SimulationParams,
    pub tolerance_loop: Vec<Tolerance>,
    results: ModelResults,
}
impl SimulationState {
    pub fn new(parameters: SimulationParams) -> Self {
        SimulationState {
            parameters,
            tolerance_loop: Vec::new(),
            results: ModelResults::new(),
        }
    }
    pub fn serialize_json(&self, filename: &str)-> Result<(), Box<dyn Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let filename_full = &[filename, ".json"].concat();
        let path = Path::new(filename_full);
        file_write(path, data)?;
        Ok(())
    }
    pub fn add(&mut self, tolerance: Tolerance) {
        self.tolerance_loop.push(tolerance);
    }
    pub fn clear(&mut self) {
        self.tolerance_loop = Vec::new();
    }
    pub fn compute_multiplier (&mut self) {
        for tol in &mut self.tolerance_loop {
            tol.compute_multiplier();
        }
    }
}
impl Default for SimulationState {
    fn default() -> Self {
        let parameters = SimulationParams{
            assy_sigma: 4.0,
            n_iterations: 1000000,
        };
        SimulationState::new(parameters)
    }
}

pub fn run(state: &SimulationState) -> Result<ModelResults,Box<dyn Error>> {
    // Divide the desired number of iterations into chunks. This is done [1] to avoid floating point
    //  errors (as the divisor gets large when averaging you lose precision) and [2] to prevent huge 
    //  memory use for large numbers of iterations. This can also be used to tune performance.
    let chunk_size = 100000;
    let chunks = state.parameters.n_iterations/chunk_size;
    let real_iters = chunks * chunk_size;
    let mut result_mean = 0f64;
    let mut result_stddev = 0f64;
    for n in 0..chunks {
        // TODO: validate n_iterations is nicely divisible by chunk_size and n_threads.
        // Gather samples into a stack that is `chunk_size` long for each Tolerance
        let stack = compute_stackup(state.tolerance_loop.clone(), chunk_size);
        // Sum each
        let stack_mean = mean(&stack);
        let stack_stddev = standard_deviation(&stack, Some(stack_mean));
        // Weighted average
        result_mean = result_mean*(n as f64/(n as f64 + 1.0)) + stack_mean*(1.0/(n as f64 + 1.0));
        result_stddev = result_stddev*(n as f64/(n as f64 + 1.0)) + stack_stddev*(1.0/(n as f64 + 1.0));
        serialize_csv(stack, "data.csv")?;
    }
    let result_tol = result_stddev * state.parameters.assy_sigma;

    Ok(ModelResults{
        mean: result_mean,
        tolerance: result_tol,
        stddev: result_stddev,
        iterations: real_iters,
    })
}

/// Generate a sample for each object in the tolerance collection, n_iterations times. Then sum
/// the results for each iteration, resulting in stackup for that iteration of the simulation.
pub fn compute_stackup(tol_collection: Vec<Tolerance>, n_iterations: usize) -> Vec<f64> {
    // Make a local clone of the tolerance collection so the borrow is not returned while the
    //  threads are using the collection.
    let tc_local = tol_collection.clone();
    // Store output in `samples` vector, appending each tol_collection's output
    let n_tols = tc_local.len();
    let mut samples:Vec<f64> =  Vec::with_capacity(n_tols * n_iterations);
    let (tx, rx) = mpsc::channel();
    // For each tolerance object generate n samples, dividing the work between multiple threads.
    for tol_struct in tc_local {
        let n_threads = 5;
        for _i in 0..n_threads {
            // Create a thread local copy of the thread communication sender for ownership reasons.
            let tx_local = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                // Make `result` thread local for better performance.
                let mut result: Vec<f64> = Vec::new();
                for _i in 0..n_iterations/n_threads {
                    result.push(
                        match tol_struct {
                            
                            // I thought this would result in branching, but it didn't impact perf.
                            Tolerance::Linear(val) => val.mc_tolerance(),
                            Tolerance::Float(val) => val.mc_tolerance(),
                            Tolerance::Compound(val) => val.mc_tolerance(),
                        }
                    );
                }
                tx_local.send(result).unwrap();
            });
        }
        for _i in  0..n_threads {
            samples.extend_from_slice(&rx.recv().unwrap());
        }
    }

    let mut result:Vec<f64> =  Vec::with_capacity(n_iterations);

    for i in 0..n_iterations {
        let mut stackup:f64 = 0.0;
        for j in 0..n_tols {
            stackup += samples[i+j*n_iterations];
        }
        result.push(stackup);
    }
    result
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Tolerance{
    Linear(LinearTL),
    Float(FloatTL),
    Compound(CompoundFloatTL),
}
impl Tolerance {
    pub fn is_linear(&self) -> bool {
        match self {
            Tolerance::Linear(_) => true,
            _ => false   
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Tolerance::Float(_) => true,
            _ => false   
        }
    }
    pub fn is_compound(&self) -> bool {
        match self {
            Tolerance::Compound(_) => true,
            _ => false   
        }
    }
    fn compute_multiplier(&mut self) {
        match self {
            Tolerance::Linear(tol) => tol.compute_multiplier(),
            Tolerance::Float(tol) => tol.compute_multiplier(),
            Tolerance::Compound(tol) => tol.compute_multiplier(),
        }
    }
}

pub trait MonteCarlo: Send + Sync + 'static {
    fn mc_tolerance(&self) -> f64;
    fn compute_multiplier (&mut self);
    //fn get_name(&self) -> &str;
}
impl  MonteCarlo for LinearTL {
    fn mc_tolerance(&self) -> f64 {
        self.distance.sample()
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
    fn compute_multiplier (&mut self) {
        self.distance.compute_multiplier();
    }
}
impl  MonteCarlo for FloatTL {
    fn mc_tolerance(&self) -> f64 {
        let hole_sample = self.hole.rand_bound_norm();
        let pin_sample = self.pin.rand_bound_norm();
        let hole_pin_slop = ( hole_sample - pin_sample ) / 2.0;
        if hole_pin_slop <= 0.0 {
            0.0
        } else {
            DimTol::new(0.0, 
                hole_pin_slop, 
                hole_pin_slop, 
                self.sigma).rand_bound_norm()
        }
    }
    fn compute_multiplier (&mut self) {
        self.hole.compute_multiplier();
        self.pin.compute_multiplier();
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
}
impl  MonteCarlo for CompoundFloatTL {
    fn mc_tolerance(&self) -> f64 {
        let ds = self.datum_start.sample();
        let de = self.datum_end.sample();
        let datum_hole = if self.datum_start.dim > self.datum_end.dim {ds} else {de};
        let datum_pin = if self.datum_start.dim < self.datum_end.dim {ds} else {de};
        let clearance_dia = datum_hole - datum_pin;

        let mut min_clearance_l = clearance_dia;
        let mut min_clearance_r = clearance_dia;

        //for float_pair in self.float_list.iter() {
            let hole_sample = self.float_list.hole.sample();
            let pin_sample = self.float_list.pin.sample();
            let hole_spacing_sample = self.float_list.hole_spacing.sample();
            let pin_spacing_sample = self.float_list.pin_spacing.sample();
            let clearance_dia = hole_sample - pin_sample;
            let misalignment = hole_spacing_sample - pin_spacing_sample;
            let clearance_r = clearance_dia + misalignment;
            let clearance_l = clearance_dia - misalignment;
            min_clearance_r = clamp(clearance_r, 0.0, min_clearance_r);
            min_clearance_l = clamp(clearance_l, 0.0, min_clearance_l);
        //}

        let mut bias = (min_clearance_r - min_clearance_l)/2.0;
        let bias_dir = if self.datum_start.dim > self.datum_end.dim {1.0} else {-1.0};
        bias *= bias_dir;

        DimTol::new(bias, min_clearance_r, min_clearance_l, self.sigma).sample()
    }
    fn compute_multiplier (&mut self) {
        self.datum_start.compute_multiplier();
        self.datum_end.compute_multiplier();
        self.float_list.hole.compute_multiplier();
        self.float_list.pin.compute_multiplier();
        self.float_list.hole_spacing.compute_multiplier();
        self.float_list.pin_spacing.compute_multiplier();
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
}

trait DimTolSampling {
    fn rand_bound_norm(&self) -> f64;
    fn sample(&self) -> f64;
    fn compute_multiplier(&mut self);
}
impl DimTolSampling for DimTol {
    /// Generate a normally distributed random value, discarding values outside of limits
    fn rand_bound_norm(&self) -> f64 {
        let mut sample: f64 = thread_rng().sample(StandardNormal);
        sample *= self.tol_multiplier;
        // TODO: limit number of checks and error out if needed to escape infinite loop
        while sample < -self.tol_neg || sample > self.tol_pos {
            sample = thread_rng().sample(StandardNormal);
            sample *= self.tol_multiplier;
        }
        sample
    }
    /// Generate a random sample of a given dimension
    fn sample(&self) -> f64 {
        self.dim + self.rand_bound_norm()
    }

    /// Precompute constant in monte carlo equation
    fn compute_multiplier(&mut self) {
        self.tol_multiplier = (self.tol_pos + self.tol_neg) / 2.0 / self.sigma;
    }
}

pub fn serialize_csv(data: Vec<f64>, filename: &str)-> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(filename)?;
    for entry in data {
        wtr.serialize(entry)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn file_write(path: &Path, data: String)-> Result<(), Box<dyn Error>> {
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Saving data to {}", display),
    }
    Ok(())
}

pub fn deserialize_json(filename: &str) -> Result<SimulationState, Box<dyn Error>> {
    let filename_full = &[filename, ".json"].concat();
    let path = Path::new(filename_full);
    let file = File::open(path)?;
    let mut result: SimulationState = serde_json::from_reader(file)?;
    result.compute_multiplier();
    Ok(result)
}

/// Data for testing purposes
pub fn dummy_data() -> SimulationState {

    let parameters = SimulationParams{
        assy_sigma: 4.0,
        n_iterations: 10000000,
    };

    let mut model = SimulationState::new(parameters);

    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(5.58, 0.03, 0.03, 3.0),
    )));
    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(-25.78, 0.07, 0.07, 3.0),
    )));
    model.add(Tolerance::Float(FloatTL::new(
        DimTol::new(2.18, 0.03, 0.03, 3.0),
        DimTol::new(2.13, 0.05, 0.05, 3.0),
        3.0,
    )));
    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(14.58, 0.05, 0.05, 3.0),
    )));
    model.add(Tolerance::Compound(CompoundFloatTL::new(
        DimTol::new(1.2, 0.03, 0.03, 3.0),
        DimTol::new(1.0, 0.03, 0.03, 3.0),
        OffsetFloat::new(
            DimTol::new(0.972, 0.03, 0.03, 3.0),
            DimTol::new(0.736, 0.03, 0.03, 3.0),
            DimTol::new(2.5, 0.05, 0.05, 3.0),
            DimTol::new(2.5, 0.3, 0.3, 3.0),
        ),
        3.0,
    )));
    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(2.5, 0.3, 0.3, 3.0),
    )));
    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(3.85, 0.25, 0.25, 3.0),
    )));
    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(-0.3, 0.15, 0.15, 3.0),
    )));
    
    model
}

/// Data for testing purposes
pub fn data() -> SimulationState {

    let parameters = SimulationParams{
        assy_sigma: 4.0,
        n_iterations: 10000000,
    };

    let mut model = SimulationState::new(parameters);

    model.add(Tolerance::Linear(LinearTL::new(
        DimTol::new(65.88, 0.17, 0.17, 3.0),
    )));
    
    model.add(Tolerance::Float(FloatTL::new(
        DimTol::new(2.50, 0.1, 0.0, 3.0),
        DimTol::new(3.0, 0.08, 0.22, 3.0),
        3.0,
    )));

    model.add(Tolerance::Float(FloatTL::new(
        DimTol::new(2.50, 0.1, 0.0, 3.0),
        DimTol::new(3.0, 0.08, 0.22, 3.0),
        3.0,
    )));

    model
}
