use statistical::*;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};
use std::thread;
use std::sync::mpsc;
use tolerance_structures::*;

fn main() {

    let time_start = Instant::now();

    // Input simulation paramters
    let params = SimulationParams{
        part_sigma: 4.0,
        assy_sigma: 3.0,
        n_iterations: 10000000,
    };

    // Load model data
    let mut tolerance_loop = ToleranceLoop::new();

    tolerance_loop.add(ToleranceType::Linear(LinearTL::new(
        DimTol::new(5.58, 0.03, 0.03, params.part_sigma),
    )));
    tolerance_loop.add(ToleranceType::Linear(LinearTL::new(
        DimTol::new(-25.78, 0.07, 0.07, params.part_sigma),
    )));
    tolerance_loop.add(ToleranceType::Float(FloatTL::new(
        DimTol::new(2.18, 0.03, 0.03, params.part_sigma),
        DimTol::new(2.13, 0.05, 0.05, params.part_sigma),
        params.part_sigma,
    )));
    tolerance_loop.add(ToleranceType::Linear(LinearTL::new(
        DimTol::new(14.58, 0.05, 0.05, params.part_sigma),
    )));
    tolerance_loop.add(ToleranceType::Compound(CompoundFloatTL::new(
        DimTol::new(1.2, 0.03, 0.03, params.part_sigma),
        DimTol::new(1.0, 0.03, 0.03, params.part_sigma),
        OffsetFloat::new(
            DimTol::new(0.972, 0.03, 0.03, params.part_sigma),
            DimTol::new(0.736, 0.03, 0.03, params.part_sigma),
            DimTol::new(2.5, 0.05, 0.05, params.part_sigma),
            DimTol::new(2.5, 0.3, 0.3, params.part_sigma),
        ),
        params.part_sigma,
    )));
    tolerance_loop.add(ToleranceType::Linear(LinearTL::new(
        DimTol::new(2.5, 0.3, 0.3, params.part_sigma),
    )));
    tolerance_loop.add(ToleranceType::Linear(LinearTL::new(
        DimTol::new(3.85, 0.25, 0.25, params.part_sigma),
    )));
    tolerance_loop.add(ToleranceType::Linear(LinearTL::new(
        DimTol::new(-0.3, 0.15, 0.15, params.part_sigma),
    )));
    println!("{:?} to load data.", time_start.elapsed());

    // Divide the desired number of iterations into chunks. This is done [1] to avoid floating point
    //  errors (as the divisor gets large when averaging you lose precision) and [2] to prevent huge 
    //  memory use for large numbers of iterations. This can also be used to tune performance.
    let chunk_size = 1000000;
    let chunks = params.n_iterations/chunk_size;
    let real_iters = chunks * chunk_size;
    let mut result_mean = 0f32;
    let mut result_stddev = 0f32;
    for n in 0..chunks {
        // TODO: validate n_iterations is nicely divisible by chunk_size and n_threads.

        // Gather samples into a stack that is `chunk_size` long for each ToleranceType
        let mut stacks = vec![Vec::new();tolerance_loop.iter().len()];
        for (i, tolerance_type) in tolerance_loop.iter() {
            stacks[i] = compute_stackup(tolerance_type, chunk_size);
        }
        // Sum each
        let mut stack_total = stacks[0].clone();
        for i in 0..chunk_size {
            stack_total[i] = stacks[0][i] + stacks[1][i] + stacks[2][i];
        }
        let stack_mean = mean(&stack_total);
        let stack_stddev = standard_deviation(&stack_total, Some(stack_mean));
        // Weighted average
        result_mean = result_mean*(n as f32/(n as f32 + 1.0)) + stack_mean*(1.0/(n as f32 + 1.0));
        result_stddev = result_stddev*(n as f32/(n as f32 + 1.0)) + stack_stddev*(1.0/(n as f32 + 1.0));
    
    }
    let result_tol = result_stddev * params.assy_sigma;
    let duration = time_start.elapsed();

    println!("Result: {:.3} +/- {:.3}; Stddev: {:.3};\nSamples: {}; Duration: {:?}", 
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

/// Generate a sample for each object in the tolerance collection, n_iterations times. Then, sum
///     the results for each iteration, resulting in stackup for that iteration of the simulation.
fn compute_stackup(tol_collection: Vec<ToleranceType>, n_iterations: usize) -> Vec<f32> {
    // Make a local clone of the tolerance collection so the borrow is not returned while the
    //  threads are using the collection.
    let tc_local = tol_collection.clone();
    // Store output in `samples` vector, appending each tol_collection's output
    let n_tols = tc_local.len();
    let mut samples:Vec<f32> =  Vec::with_capacity(n_tols * n_iterations);
    let (tx, rx) = mpsc::channel();
    // For each tolerance object generate n samples, dividing the work between multiple threads.
    for tol_struct in tc_local {
        let n_threads = 5;
        for _i in 0..n_threads {
            // Create a thread local copy of the thread communication sender for ownership reasons.
            let tx_local = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                // Make `result` thread local for better performance.
                let mut result: Vec<f32> = Vec::new();
                for _i in 0..n_iterations/n_threads {
                    result.push(
                        match tol_struct {
                            // I thought this would result in branching, but it didn't impact perf.
                            ToleranceType::Linear(val) => val.mc_tolerance(),
                            ToleranceType::Float(val) => val.mc_tolerance(),
                            ToleranceType::Compound(val) => val.mc_tolerance(),
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

    let mut result:Vec<f32> =  Vec::with_capacity(n_iterations);

    for i in 0..n_iterations {
        let mut stackup:f32 = 0.0;
        for j in 0..n_tols {
            stackup += samples[i+j*n_iterations];
        }
        result.push(stackup);
    }
    result
}

pub struct SimulationParams{
    pub part_sigma: f32,
    pub assy_sigma: f32,
    pub n_iterations: usize,
}



/// Contains structures used to define tolerances in a tolerance loop.
pub mod tolerance_structures {

    use num::clamp;
    use rand::prelude::*;
    use rand_distr::StandardNormal;

    #[derive(Copy, Clone)]
    pub enum ToleranceType{
        Linear(LinearTL),
        Float(FloatTL),
        Compound(CompoundFloatTL),
    }

    pub struct ToleranceLoop{
        linear: Vec<ToleranceType>,
        float: Vec<ToleranceType>,
        compound: Vec<ToleranceType>,
    }
    impl  ToleranceLoop {
        pub fn new() -> Self {
            ToleranceLoop{
                linear: Vec::new(),
                float: Vec::new(),
                compound: Vec::new(),
            }
        }
        pub fn add(&mut self, tol_collection: ToleranceType) {
            match tol_collection {
                ToleranceType::Linear(_) => self.linear.push(tol_collection),
                ToleranceType::Float(_) => self.float.push(tol_collection),
                ToleranceType::Compound(_) => self.compound.push(tol_collection),
            }
        }
        pub fn iter(&mut self) -> Vec<(usize, Vec<ToleranceType>)> {
            let mut result = Vec::new();
            result.push((0, self.linear.clone()));
            result.push((1, self.float.clone()));
            result.push((2, self.compound.clone()));
            result
        }
    }

    pub trait MonteCarlo: Send + Sync + 'static {
        fn mc_tolerance(&self) -> f32;
        //fn get_name(&self) -> &str;
    }

    #[derive(Copy, Clone)]
    pub struct DimTol{
        dim: f32,
        tol_pos: f32,
        tol_neg: f32,
        tol_multiplier: f32,
    }
    impl DimTol{
        pub fn new(dim: f32, tol_pos: f32, tol_neg: f32, sigma: f32) -> Self {
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
    pub struct LinearTL {
        //name: String,
        distance: DimTol,
    }
    impl  LinearTL {
        pub fn new(distance: DimTol) -> Self {
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
    pub struct FloatTL {
        //name: String,
        hole: DimTol,
        pin: DimTol,
        sigma: f32,
    }
    impl  FloatTL {
        pub fn new(hole: DimTol, pin: DimTol, sigma: f32) -> Self {
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
                DimTol::new(0.0, 
                    hole_pin_slop, 
                    hole_pin_slop, 
                    self.sigma).rand_bound_norm()
            }
        }
        //fn get_name(&self) -> &str {
        //    &self.name
        //}
    }


    #[derive(Copy, Clone)]
    pub struct CompoundFloatTL {
        //name: String,
        datum_time_start: DimTol,
        datum_end: DimTol,
        float_list: OffsetFloat,
        sigma: f32,
    }
    impl  CompoundFloatTL {
        pub fn new(datumtime_start: DimTol, datumend: DimTol, floatlist: OffsetFloat, sigma: f32) -> Self {
            CompoundFloatTL{
                //name,
                datum_time_start: datumtime_start,
                datum_end: datumend,
                float_list: floatlist,
                sigma,
            }
        }
    }
    impl  MonteCarlo for CompoundFloatTL {
        fn mc_tolerance(&self) -> f32 {
            let ds = self.datum_time_start.sample();
            let de = self.datum_end.sample();
            let datum_hole = if self.datum_time_start.dim > self.datum_end.dim {ds} else {de};
            let datum_pin = if self.datum_time_start.dim < self.datum_end.dim {ds} else {de};
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
            let bias_dir = if self.datum_time_start.dim > self.datum_end.dim {1.0} else {-1.0};
            bias *= bias_dir;

            DimTol::new(bias, min_clearance_r, min_clearance_l, self.sigma).sample()
        }
        //fn get_name(&self) -> &str {
        //    &self.name
        //}
    }

    #[derive(Copy, Clone)]
    pub struct OffsetFloat {
        hole: DimTol,
        pin: DimTol,
        hole_spacing: DimTol,
        pin_spacing: DimTol,
    }
    impl  OffsetFloat {
        pub fn new(hole: DimTol, pin: DimTol, hole_spacing: DimTol, pin_spacing: DimTol) -> Self {
            OffsetFloat {
                hole,
                pin,
                hole_spacing,
                pin_spacing,
            }
        }
    }
}