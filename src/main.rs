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
use std::thread;
use std::sync::mpsc;

fn main() {

    let start = Instant::now();

    let params = SimulationParams::new(4.0, 3.0, 10000000);

    //println!("{:?} to start.", start.elapsed());

    // Load model data
    let mut lin_collect: Vec<LinearTL> = Vec::new();
    let mut flt_collect: Vec<FloatTL> = Vec::new();
    let mut cmp_collect: Vec<CompoundFloatTL> = Vec::new();


    lin_collect.push(LinearTL::new(
        //"A: Actuator tip to hard stop".to_string(),
        DimTol::new(5.58, 0.03, 0.03, params.part_sigma),
    ));
    lin_collect.push(LinearTL::new(
        //"B: Hard stop to MF pin".to_string(), 
        DimTol::new(-25.78, 0.07, 0.07, params.part_sigma),
    ));
    flt_collect.push(FloatTL::new(
        //"C: MF pin float".to_string(),
        DimTol::new(2.18, 0.03, 0.03, params.part_sigma),
        DimTol::new(2.13, 0.05, 0.05, params.part_sigma),
        params.part_sigma,
    ));
    lin_collect.push(LinearTL::new(
        //"D: PCB MF hole to component hole".to_string(),
        DimTol::new(14.58, 0.05, 0.05, params.part_sigma),
    ));
    cmp_collect.push(CompoundFloatTL::new(
        //"E: Switch component pins float".to_string(), 
        DimTol::new(1.2, 0.03, 0.03, params.part_sigma),
        DimTol::new(1.0, 0.03, 0.03, params.part_sigma),
        OffsetFloat::new(
            DimTol::new(0.972, 0.03, 0.03, params.part_sigma),
            DimTol::new(0.736, 0.03, 0.03, params.part_sigma),
            DimTol::new(2.5, 0.05, 0.05, params.part_sigma),
            DimTol::new(2.5, 0.3, 0.3, params.part_sigma),
        ),
        params.part_sigma,
    ));
    lin_collect.push(LinearTL::new(
        //"E': pin to pin".to_string(), 
        DimTol::new(2.5, 0.3, 0.3, params.part_sigma),
    ));
    lin_collect.push(LinearTL::new(
        //"F: Switch pin to button surface".to_string(), 
        DimTol::new(3.85, 0.25, 0.25, params.part_sigma),
    ));
    lin_collect.push(LinearTL::new(
        //"G: Button actuation".to_string(), 
        DimTol::new(-0.3, 0.15, 0.15, params.part_sigma),
    ));

    println!("{:?} to load data.", start.elapsed());

    let chunk_size = 1000000;
    let chunks = params.n_iterations/chunk_size;
    let real_iters = chunks * chunk_size;
    let mut result_mean = 0f32;
    let mut result_stddev = 0f32;
    //let mut sum_time: u128 = 0;
    for n in 0..chunks {
        //TODO return tol_collection from compute_stackup so that the vec can
        // move in then back out without using a borrow. The result vector would
        // instead be passed in as a &mut self.
        let stack1 = compute_stackup(&lin_collect, chunk_size);
        let stack2 = compute_stackup(&flt_collect, chunk_size);
        let stack3 = compute_stackup(&cmp_collect, chunk_size);
        //let start = Instant::now();
        let mut stack_total = stack1.clone();
        for i in 0..chunk_size {
            //println!("st: {} s1 {} s2{} s3{}", stack_total.len(), stack1.len(), stack2.len(), stack3.len());
            stack_total[i] = stack1[i] + stack2[i] + stack3[i];
        }
        //sum_time += start.elapsed().as_micros();
        //println!("{:?}", start.elapsed());
        let stack_mean = mean(&stack_total);
        let stack_stddev = standard_deviation(&stack_total, Some(stack_mean));
        // Weighted average
        result_mean = result_mean*(n as f32/(n as f32 + 1.0)) + stack_mean*(1.0/(n as f32 + 1.0));
        result_stddev = result_stddev*(n as f32/(n as f32 + 1.0)) + stack_stddev*(1.0/(n as f32 + 1.0));
    
    }
    //println!("compute_stackup: {}us",sum_time);
    let result_tol = result_stddev * params.assy_sigma;
    let duration = start.elapsed();
    println!("Result: {:.4} +/- {:.4}; Stddev: {:.4};\nSamples: {}; Duration: {:?}", 
        result_mean, 
        result_tol, 
        result_stddev, 
        params.n_iterations.to_formatted_string(&Locale::en), 
        duration,
    );
    println!("Rate: {:.2} iter/μs; Timing: {:.2} ns/iter", 
        (real_iters as f32)/(duration.as_micros() as f32),
        (duration.as_nanos() as f32)/(real_iters as f32),
    );
}

fn compute_stackup<T: MonteCarlo + Copy>(tol_collection: &Vec<T>, n_iterations: usize) -> Vec<f32> {
    //let start = Instant::now();
    let tc2 = tol_collection.clone();
    let vec_length = n_iterations;
    let vec_height = tc2.len();
    let mut samples:Vec<f32> =  Vec::with_capacity(vec_height * vec_length);
    let (tx, rx) = mpsc::channel();
    for tol_struct in tc2 {
        let n_threads = 5;
        for _i in 0..n_threads {
            let tx1 = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                //let start = Instant::now();
                //println!("{:}",tol_struct.get_name());
                let mut result: Vec<f32> = Vec::new();
                for _i in 0..n_iterations/n_threads {
                    result.push(tol_struct.mc_tolerance());
                }
                //println!("Thread {}: {:?}",tol_struct.get_name(), start.elapsed());
                tx1.send(result).unwrap();
            });
        }
        for _i in  0..n_threads {
            samples.extend_from_slice(&rx.recv().unwrap());
        }
    }
    //println!("Threads spawned: {:?}",start.elapsed());

    //println!("Threads collected: {:?}",start.elapsed());


    let mut result:Vec<f32> =  Vec::with_capacity(n_iterations);

    for i in 0..vec_length {
        let mut stackup:f32 = 0.0;
        for j in 0..vec_height {
            stackup += samples[i+j*vec_length];
        }
        result.push(stackup);
    }
    result
}

struct SimulationParams{
    part_sigma: f32,
    assy_sigma: f32,
    n_iterations: usize,
}
impl SimulationParams {
    fn new(part_sigma: f32, assy_sigma: f32, n_iterations: usize) -> Self {
        SimulationParams{
            part_sigma,
            assy_sigma,
            n_iterations,
        }
    }
}

#[derive(Copy, Clone)]
struct DimTol{
    dim: f32,
    tol_pos: f32,
    tol_neg: f32,
    tol_multiplier: f32,
}
impl DimTol{
    fn new(dim: f32, tol_pos: f32, tol_neg: f32, sigma: f32) -> Self {
        let tol_multiplier: f32 = (tol_pos + tol_neg) / 2.0 / sigma;
        DimTol{
            dim,
            tol_pos,
            tol_neg,
            tol_multiplier,
        }
    }

    fn rand_bound_norm(&self) -> f32 {
        let mut sample: f32 = thread_rng().sample(StandardNormal);
        sample *= self.tol_multiplier;
        while sample < -self.tol_neg || sample > self.tol_pos {
            sample = thread_rng().sample(StandardNormal);
            sample *= self.tol_multiplier;
        }
        sample
    }

    fn sample(&self) -> f32 {
        self.dim + self.rand_bound_norm()
    }
}


#[derive(Copy, Clone)]
struct OffsetFloat {
    hole: DimTol,
    pin: DimTol,
    hole_spacing: DimTol,
    pin_spacing: DimTol,
}
impl  OffsetFloat {
    fn new(hole: DimTol, pin: DimTol, hole_spacing: DimTol, pin_spacing: DimTol) -> Self {
        OffsetFloat {
            hole,
            pin,
            hole_spacing,
            pin_spacing,
        }
    }
}

trait MonteCarlo: Send + Sync + Sized + 'static {
    fn mc_tolerance(&self) -> f32;
    //fn get_name(&self) -> &str;
}

#[derive(Copy, Clone)]
struct LinearTL {
    //name: String,
    distance: DimTol,
}
impl  LinearTL {
    fn new(distance: DimTol) -> Self {
        LinearTL {
            //name,
            distance,
        }
    }
}
impl  MonteCarlo for LinearTL {
    fn mc_tolerance(&self) -> f32 {
        self.distance.sample()
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
}


#[derive(Copy, Clone)]
struct FloatTL {
    //name: String,
    hole: DimTol,
    pin: DimTol,
    sigma: f32,
}
impl  FloatTL {
    fn new(hole: DimTol, pin: DimTol, sigma: f32) -> Self {
        FloatTL {
            //name,
            hole,
            pin,
            sigma,
        }
    }
}
impl  MonteCarlo for FloatTL {
    fn mc_tolerance(&self) -> f32 {
        let hole_sample = self.hole.rand_bound_norm();
        let pin_sample = self.pin.rand_bound_norm();
        let hole_pin_slop = ( hole_sample - pin_sample ) / 2.0;
        if hole_pin_slop <= 0.0 {
            0.0
        } else {
            DimTol::new(0.0, hole_pin_slop, hole_pin_slop, self.sigma).rand_bound_norm()
        }
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
}


#[derive(Copy, Clone)]
struct CompoundFloatTL {
    //name: String,
    datum_start: DimTol,
    datum_end: DimTol,
    float_list: OffsetFloat,
    sigma: f32,
}
impl  CompoundFloatTL {
    fn new(datumstart: DimTol, datumend: DimTol, floatlist: OffsetFloat, sigma: f32) -> Self {
        CompoundFloatTL{
            //name,
            datum_start: datumstart,
            datum_end: datumend,
            float_list: floatlist,
            sigma,
        }
    }
}
impl  MonteCarlo for CompoundFloatTL {
    fn mc_tolerance(&self) -> f32 {
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
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
}