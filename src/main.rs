extern crate rand_distr;
extern crate rand;
extern crate statistical;
extern crate num;

use rand::prelude::*;
use rand_distr::StandardNormal;
use statistical::*;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};
use num::clamp;

fn main() {

    let start = Instant::now();

    let params = SimulationParams::new(4.0, 3.0, 1000000);

    //println!("{:?} to start.", start.elapsed());

    // Load model data
    let mut tol_collection:Vec<Box<dyn MonteCarlo>> = Vec::new();
    tol_collection.push(Box::from(LinearTL::new(
        "A: Actuator tip to hard stop".to_string(),
        DimTol::new(5.58, 0.03, 0.03, params.part_sigma),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "B: Hard stop to MF pin".to_string(), 
        DimTol::new(-25.78, 0.07, 0.07, params.part_sigma),
    )));
    tol_collection.push(Box::from(FloatTL::new(
        "C: MF pin float".to_string(),
        DimTol::new(2.18, 0.03, 0.03, params.part_sigma),
        DimTol::new(2.13, 0.05, 0.05, params.part_sigma),
        params.part_sigma,
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "D: PCB MF hole to component hole".to_string(),
        DimTol::new(14.58, 0.05, 0.05, params.part_sigma),
    )));
    tol_collection.push(Box::from(CompoundFloatTL::new(
        "E: Switch component pins float".to_string(), 
        DimTol::new(1.2, 0.03, 0.03, params.part_sigma),
        DimTol::new(1.0, 0.03, 0.03, params.part_sigma),
        vec!(OffsetFloat::new(
            DimTol::new(0.972, 0.03, 0.03, params.part_sigma),
            DimTol::new(0.736, 0.03, 0.03, params.part_sigma),
            DimTol::new(2.5, 0.05, 0.05, params.part_sigma),
            DimTol::new(2.5, 0.3, 0.3, params.part_sigma),
        )),
        params.part_sigma,
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "E': pin to pin".to_string(), 
        DimTol::new(2.5, 0.3, 0.3, params.part_sigma),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "F: Switch pin to button surface".to_string(), 
        DimTol::new(3.85, 0.25, 0.25, params.part_sigma),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "G: Button actuation".to_string(), 
        DimTol::new(-0.3, 0.15, 0.15, params.part_sigma),
    )));

    println!("{:?} to load data.", start.elapsed());

    //260ms
    
    let chunk_size = 1000;
    let n_cycles = params.n_iterations/chunk_size;
    let mut stack: Vec<f64> = Vec::new();
    for _i in 0..n_cycles {
        let mut result = compute_stackup(&tol_collection, chunk_size);
        stack.append(&mut result);
    }
    let duration = start.elapsed();
    let stack_mean = mean(&stack);
    let stack_stddev = standard_deviation(&stack, Some(stack_mean));
    let stack_tol = stack_stddev * params.assy_sigma;
    println!("Result: {:.4} +/- {:.4}; Stddev: {:.4}; Samples: {}; Duration: {:?}", 
        stack_mean, stack_tol, stack_stddev, stack.len().to_formatted_string(&Locale::en), duration);
    println!("Iterations: {}; Rate: {:.2} iterations/us", 
        params.n_iterations, stack.len() as f64/duration.subsec_micros() as f64);
        /*let start = Instant::now();
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<f64> = Vec::new();
        for _i in 0..1000000 {
            numbers.push(rng.sample(StandardNormal));
        }
        println!("For loop: {:?}", start.elapsed());

        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let numbers: Vec<f64> = (0..1000000).map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            rng.sample(StandardNormal)
        }).collect();
        println!("Map: {:?}", start.elapsed());*/
    
}

fn compute_stackup(tol_collection: &Vec<Box<dyn MonteCarlo>>, n_iterations: usize) -> Vec<f64> {
    let vec_length = n_iterations;
    let vec_height = tol_collection.len();
    let mut samples:Vec<f64> =  Vec::with_capacity(vec_height * vec_length);
    for tol_struct in tol_collection {
        let start = Instant::now();
        for _i in 0..n_iterations {
            samples.push(tol_struct.mc_tolerance())
        }
        //println!("{:?}",start.elapsed());
    }
    let mut result:Vec<f64> =  Vec::with_capacity(n_iterations);
    //println!("{:?}", samples);
    for i in 0..vec_length {
        let mut stackup:f64 = 0.0;
        for j in 0..vec_height {
            stackup += samples[i+j*vec_length];
            //println!("i:{}, j:{}, val:{}", i,j,samples[i+j*vec_length]);
        }
        result.push(stackup);
    }
    result
}

struct SimulationParams{
    part_sigma: f64,
    assy_sigma: f64,
    n_iterations: usize,
}
impl SimulationParams {
    fn new(part_sigma: f64, assy_sigma: f64, n_iterations: usize) -> Self {
        let mut rng = rand::thread_rng();
        //let rand_source: Vec<f64> = (0..n_iterations+rand_buffer).map(|_| {
        //    rng.sample(StandardNormal)
        //}).collect();
        SimulationParams{
            part_sigma,
            assy_sigma,
            n_iterations,
        }
    }
}

struct DimTol{
    dim: f64,
    tol_pos: f64,
    tol_neg: f64,
    tol_multiplier: f64,
    sigma: f64,
}
impl DimTol{
    fn new(dim: f64, tol_pos: f64, tol_neg: f64, sigma: f64) -> Self {
        //let start = Instant::now();
        //let slice_point = rand::thread_rng().gen_range(0, params.rand_buffer);
        //let rand_source = &params.rand_source[slice_point..slice_point+params.n_iterations];
        let tol_multiplier: f64 = (tol_pos + tol_neg) / 2.0 / sigma;
        //let sampled_dims: Vec<f64> = rand_source.iter().map(|v| v * tol_multiplier + dim).collect();
        //println!("Length:{:?}; First Entry:{:.4}", sampled_dims.len(),sampled_dims[0]);
        //println!("{:?}", start.elapsed());

        DimTol{
            dim,
            tol_pos,
            tol_neg,
            tol_multiplier,
            sigma,
        }
    }
    fn rand_bound_norm(&self) -> f64 {
        let mut sample:f64 = std::f64::MAX;
        sample = thread_rng().sample(StandardNormal);
        sample *= self.tol_multiplier;
        //let mut rng = rand::thread_rng();
        while sample < -self.tol_neg || sample > self.tol_pos {
            sample = thread_rng().sample(StandardNormal);
            sample *= self.tol_multiplier;
            //precalculate tolup+toldown/2 on object creation
        }
        //println!("{}", sample);
        sample
    }

    fn sample(&self) -> f64 {
        self.dim + self.rand_bound_norm()
    }
}

struct OffsetFloat {
    hole: DimTol,
    pin: DimTol,
    hole_spacing: DimTol,
    pin_spacing: DimTol,
    sampled_results: Vec<f64>,
}
impl  OffsetFloat {
    fn new(hole: DimTol, pin: DimTol, hole_spacing: DimTol, pin_spacing: DimTol) -> Self {
        OffsetFloat {
            hole,
            pin,
            hole_spacing,
            pin_spacing,
            sampled_results: Vec::new(),
        }
    }
}

trait MonteCarlo {
    /// Compute a monte carlo sample for a given tolerance
    fn mc_tolerance(&self) -> f64;
}

struct LinearTL {
    name: String,
    distance: DimTol,
    sampled_results: Vec<f64>,
}
impl  LinearTL {
    fn new(name: String, distance: DimTol) -> Self {
        LinearTL {
            name,
            distance,
            sampled_results: Vec::new(),
        }
    }
}
impl  MonteCarlo for LinearTL {
    fn mc_tolerance(&self) -> f64 {
        self.distance.sample()
    }
}

struct FloatTL {
    name: String,
    hole: DimTol,
    pin: DimTol,
    sigma: f64,
}
impl  FloatTL {
    fn new(name: String, hole: DimTol, pin: DimTol, sigma: f64) -> Self {
        FloatTL {
            name,
            hole,
            pin,
            sigma,
        }
    }
}
impl  MonteCarlo for FloatTL {
    fn mc_tolerance(&self) -> f64 {
        //let mut result:Vec<f64> = Vec::new();
        let hole_sample = self.hole.rand_bound_norm();
        let pin_sample = self.pin.rand_bound_norm();
        let hole_pin_slop = ( hole_sample - pin_sample ) / 2.0;
        if hole_pin_slop <= 0.0 {
            0.0
        } else {
            DimTol::new(0.0, hole_pin_slop, hole_pin_slop, self.sigma).rand_bound_norm()
        }
    }
}

struct CompoundFloatTL {
    name: String,
    datum_start: DimTol,
    datum_end: DimTol,
    float_list: Vec<OffsetFloat>,
    sigma: f64,
}
impl  CompoundFloatTL {
    fn new(name: String, datumstart: DimTol, datumend: DimTol, floatlist: Vec<OffsetFloat>, sigma: f64) -> Self {
        CompoundFloatTL{
            name,
            datum_start: datumstart,
            datum_end: datumend,
            float_list: floatlist,
            sigma,
        }
    }
}
impl  MonteCarlo for CompoundFloatTL {
    fn mc_tolerance(&self) -> f64 {
        //let mut result:Vec<f64> = Vec::new();
        let ds = self.datum_start.sample();
        let de = self.datum_end.sample();
        let datum_hole = if self.datum_start.dim > self.datum_end.dim {ds} else {de};
        let datum_pin = if self.datum_start.dim < self.datum_end.dim {ds} else {de};
        //let d2 = if self.datumstart.dim < self.datumend.dim {1.0} else {-1.0};
        let clearance_dia = datum_hole - datum_pin;

        let mut min_clearance_l = clearance_dia;
        let mut min_clearance_r = clearance_dia;

        for float_pair in self.float_list.iter() {
            let hole_sample = float_pair.hole.sample();
            let pin_sample = float_pair.pin.sample();
            let hole_spacing_sample = float_pair.hole_spacing.sample();
            let pin_spacing_sample = float_pair.pin_spacing.sample();
            let clearance_dia = hole_sample - pin_sample;
            let misalignment = hole_spacing_sample - pin_spacing_sample;
            let clearance_r = clearance_dia + misalignment;
            let clearance_l = clearance_dia - misalignment;
            min_clearance_r = clamp(clearance_r, 0.0, min_clearance_r);
            min_clearance_l = clamp(clearance_l, 0.0, min_clearance_l);
        }

        //let clearance_total = min_clearance_r + min_clearance_l;
        let mut bias = (min_clearance_r - min_clearance_l)/2.0;
        let bias_dir = if self.datum_start.dim > self.datum_end.dim {1.0} else {-1.0};
        bias *= bias_dir;

        DimTol::new(bias, min_clearance_r, min_clearance_l, self.sigma).sample()
    }
}