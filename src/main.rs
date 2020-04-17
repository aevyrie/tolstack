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

    let params = SimulationParams::new(4.0, 3.0, 1000000, 1000);

    println!("{:?}", start.elapsed());

    // Load model data
    let mut tol_collection:Vec<Box<dyn MonteCarlo>> = Vec::new();
    tol_collection.push(Box::from(LinearTL::new(
        "A: Actuator tip to hard stop".to_string(),
        DimTol::new(5.58, 0.03, 0.03, &params),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "B: Hard stop to MF pin".to_string(), 
        DimTol::new(-25.78, 0.07, 0.07, &params),
    )));
    tol_collection.push(Box::from(FloatTL::new(
        "C: MF pin float".to_string(),
        DimTol::new(2.18, 0.03, 0.03, &params),
        DimTol::new(2.13, 0.05, 0.05, &params),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "D: PCB MF hole to component hole".to_string(),
        DimTol::new(14.58, 0.05, 0.05, &params),
    )));
    tol_collection.push(Box::from(CompoundFloatTL::new(
        "E: Switch component pins float".to_string(), 
        DimTol::new(1.2, 0.03, 0.03, &params),
        DimTol::new(1.0, 0.03, 0.03, &params),
        vec!(OffsetFloat::new(
            DimTol::new(0.972, 0.03, 0.03, &params),
            DimTol::new(0.736, 0.03, 0.03, &params),
            DimTol::new(2.5, 0.05, 0.05, &params),
            DimTol::new(2.5, 0.3, 0.3, &params),
        ))
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "E': pin to pin".to_string(), 
        DimTol::new(2.5, 0.3, 0.3, &params),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "F: Switch pin to button surface".to_string(), 
        DimTol::new(3.85, 0.25, 0.25, &params),
    )));
    tol_collection.push(Box::from(LinearTL::new(
        "G: Button actuation".to_string(), 
        DimTol::new(-0.3, 0.15, 0.15, &params),
    )));

    println!("{:?}", start.elapsed());

    //260ms
    /*
    for _1 in 0..1 {
        let start = Instant::now();
        let stack = compute_stackup(&tol_collection, &part_sigma, &n_iterations);
        let stack_mean = mean(&stack);
        let stack_stddev = standard_deviation(&stack, Some(stack_mean));
        let stack_tol = stack_stddev * assy_sigma;

        let duration = start.elapsed();
        println!("Result: {:.4} +/- {:.4}; Stddev: {:.4}; Samples: {}; Duration: {:?}", 
            stack_mean, stack_tol, stack_stddev, stack.len().to_formatted_string(&Locale::en), duration);

        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<f64> = Vec::new();
        for _i in 0..14000000 {
            numbers.push(rng.sample(StandardNormal));
        }
        println!("For loop: {:?}", start.elapsed());

        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let numbers: Vec<f64> = (0..100000).map(|_| {
            // 1 (inclusive) to 21 (exclusive)
            rng.sample(StandardNormal)
        }).collect();
        println!("Map: {:?}", start.elapsed());
    }
    */
}

fn rand_bound_norm(tol_up:f64, tol_down:f64, sigma_level:&f64) -> f64 {
    let mut sample:f64 = std::f64::MAX;
    //let mut rng = rand::thread_rng();
    while sample < -tol_down || sample > tol_up {
        sample = thread_rng().sample(StandardNormal);
        sample *= ((tol_up+tol_down)/2.0) / sigma_level;
        //precalculate tolup+toldown/2 on object creation
    }
    //println!("{}", sample);
    sample
}

fn compute_stackup(tol_collection:&Vec<Box<dyn MonteCarlo>>, part_sigma:&f64, n_iterations:&usize) -> Vec<f64> {
    let vec_length = n_iterations;
    let vec_height = tol_collection.len();
    let mut samples:Vec<f64> =  Vec::with_capacity(vec_height * vec_length);
    for tol_struct in tol_collection {
        //let start = Instant::now();
        for _i in 0..*n_iterations {
            samples.push(tol_struct.mc_tolerance(part_sigma));
        }
        //println!("{:?}",start.elapsed());
    }
    let mut result:Vec<f64> =  Vec::with_capacity(*n_iterations);
    //println!("{:?}", samples);
    for i in 0..*vec_length {
        let mut stackup:f64 = 0.0;
        for j in 0..vec_height {
            stackup += samples[i+j*vec_length];
            //println!("i:{}, j:{}, val:{}", i,j,samples[i+j*vec_length]);
        }
        result.push(stackup);
    }
    result
}

trait MonteCarlo {
    /// Compute a monte carlo sample for a given olerance
    fn mc_tolerance(&self) -> Vec<f64>;
}

struct SimulationParams{
    part_sigma: f64,
    assy_sigma: f64,
    n_iterations: usize,
    rand_source: Vec<f64>,
    rand_buffer: usize,
}
impl SimulationParams {
    fn new(part_sigma: f64, assy_sigma: f64, n_iterations: usize, rand_buffer: usize) -> Self {
        let mut rng = rand::thread_rng();
        let rand_source: Vec<f64> = (0..n_iterations+rand_buffer).map(|_| {
            rng.sample(StandardNormal)
        }).collect();
        SimulationParams{
            part_sigma,
            assy_sigma,
            n_iterations,
            rand_source,
            rand_buffer,
        }
    }
}

struct DimTol{
    dim: f64,
    tol_pos: f64,
    tol_neg: f64,
    sampled_dims: Vec<f64>,
}
impl DimTol{
    fn new(dim: f64, tol_pos: f64, tol_neg: f64, params: &SimulationParams) -> Self {
        let start = Instant::now();

        let slice_point = rand::thread_rng().gen_range(0, params.rand_buffer);
        let rand_source = &params.rand_source[slice_point..slice_point+params.n_iterations];
        let tol_multiplier: f64 = (tol_pos + tol_neg) / 2.0 / (params.part_sigma as f64);
        let sampled_dims: Vec<f64> = rand_source.iter().map(|v| v * tol_multiplier + dim).collect();
        println!("Length:{:?}; First Entry:{:.4}", sampled_dims.len(),sampled_dims[0]);
        println!("{:?}", start.elapsed());

        DimTol{
            dim,
            tol_pos,
            tol_neg,
            sampled_dims,
        }
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
    fn mc_tolerance(&self) -> Vec<f64> {
        self.sampled_results
    }
}

struct FloatTL {
    name: String,
    hole: DimTol,
    pin: DimTol,
    sampled_results: Vec<f64>,
}
impl  FloatTL {
    fn new(name: String, hole: DimTol, pin: DimTol) -> Self {
        FloatTL {
            name,
            hole,
            pin,
            sampled_results: Vec::new(),
        }
    }
}
impl  MonteCarlo for FloatTL {
    fn mc_tolerance(&self) -> Vec<f64> {
        //let mut result:Vec<f64> = Vec::new();
        let hole_dim = self.hole.sampled_dims;
        let pin_dim = self.pin.sampled_dims;
        let hole_pin_slop = (hole_dim - pin_dim)/2.0;
        if hole_pin_slop <= 0.0 {
            0.0
        } else {
            rand_bound_norm(hole_pin_slop, hole_pin_slop, sigma_level)
        }
    }
}

struct CompoundFloatTL {
    name: String,
    datum_start: DimTol,
    datum_end: DimTol,
    float_list: Vec<OffsetFloat>,
    sampled_results: Vec<f64>,
}
impl  CompoundFloatTL {
    fn new(name: String, datumstart: DimTol, datumend: DimTol, floatlist: Vec<OffsetFloat>) -> Self {
        CompoundFloatTL{
            name,
            datum_start: datumstart,
            datum_end: datumend,
            float_list: floatlist,
            sampled_results: Vec::new(),
        }
    }
}
impl  MonteCarlo for CompoundFloatTL {
    fn mc_tolerance(&self, sigma_level:&f64) -> f64 {
        //let mut result:Vec<f64> = Vec::new();
        let ds = self.datum_start.dim + rand_bound_norm(self.datum_start.tol_pos, self.datum_start.tol_neg, sigma_level);
        let de = self.datum_end.dim + rand_bound_norm(self.datum_end.tol_pos, self.datum_end.tol_neg, sigma_level);
        let datum_hole:f64;
        let datum_pin:f64;
        if self.datum_start.dim > self.datum_end.dim {
            datum_hole = ds;
            datum_pin = de;
        } else {
            datum_hole = de;
            datum_pin = ds;
        }
        //let d2 = if self.datumstart.dim < self.datumend.dim {1.0} else {-1.0};
        let clearance_dia = datum_hole - datum_pin;

        let mut min_clearance_l = clearance_dia;
        let mut min_clearance_r = clearance_dia;

        for float_pair in self.float_list.iter() {
            let h = float_pair.hole.dim + rand_bound_norm(float_pair.hole.tol_pos, float_pair.hole.tol_neg, sigma_level);
            let p = float_pair.pin.dim + rand_bound_norm(float_pair.pin.tol_pos, float_pair.pin.tol_neg, sigma_level);
            let hs = float_pair.hole_spacing.dim + rand_bound_norm(float_pair.hole_spacing.tol_pos, float_pair.hole_spacing.tol_neg, sigma_level);
            let ps = float_pair.pin_spacing.dim + rand_bound_norm(float_pair.pin_spacing.tol_pos, float_pair.pin_spacing.tol_neg, sigma_level);
            let clearance_dia = h - p;
            let misalignment = hs - ps;
            let clearance_r = clearance_dia + misalignment;
            let clearance_l = clearance_dia - misalignment;
            min_clearance_r = clamp(clearance_r, 0.0, min_clearance_r);
            min_clearance_l = clamp(clearance_l, 0.0, min_clearance_l);
        }

        //let clearance_total = min_clearance_r + min_clearance_l;
        let mut bias = (min_clearance_r - min_clearance_l)/2.0;
        let bias_dir = if self.datum_start.dim > self.datum_end.dim {1.0} else {-1.0};
        bias *= bias_dir;

        bias + rand_bound_norm(min_clearance_r, min_clearance_l, sigma_level)
    }
}