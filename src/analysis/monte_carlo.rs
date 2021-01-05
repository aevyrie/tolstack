use super::structures::*;

use std::error::Error;
use std::sync::mpsc;
use std::thread;

use rand::prelude::*;
use rand_distr::{StandardNormal, Uniform};
use statistical::*;

pub async fn run(state: &State) -> Result<McResults, Box<dyn Error>> {
    // Divide the desired number of iterations into chunks. This is done [1] to avoid floating point
    //  errors (as the divisor gets large when averaging you lose precision) and [2] to prevent huge
    //  memory use for large numbers of iterations. This can also be used to tune performance.
    let chunk_size = 100000;
    let chunks = state.parameters.n_iterations / chunk_size;
    let real_iters = chunks * chunk_size;
    let mut result = Vec::new();
    let mut result_mean = 0f64;
    let mut result_stddev_pos = 0f64;
    let mut result_stddev_neg = 0f64;

    for _n in 0..chunks {
        // TODO: validate n_iterations is nicely divisible by chunk_size and n_threads.
        // Gather samples into a stack that is `chunk_size` long for each Tolerance
        let mut stack = compute_stackup(state.tolerance_loop.clone(), chunk_size);
        // Sum each
        let stack_mean: f64 = mean(&stack);
        let stack_stddev_pos = standard_deviation(
            &stack
                .iter()
                .cloned()
                .filter(|x| x > &stack_mean)
                .collect::<Vec<f64>>(),
            Some(stack_mean),
        );
        let stack_stddev_neg = standard_deviation(
            &stack
                .iter()
                .cloned()
                .filter(|x| x < &stack_mean)
                .collect::<Vec<f64>>(),
            Some(stack_mean),
        );

        result.append(&mut stack);
        result_mean += stack_mean;
        result_stddev_neg += stack_stddev_neg;
        result_stddev_pos += stack_stddev_pos;
    }

    //result_mean = result_mean / chunks as f64;
    result_stddev_neg /= chunks as f64;
    result_stddev_pos /= chunks as f64;

    result_mean = mean(&result);

    let result_tol_pos = result_stddev_pos * state.parameters.assy_sigma;
    let result_tol_neg = result_stddev_neg * state.parameters.assy_sigma;

    let worst_case_dim = state.tolerance_loop.iter().fold(0.0, |acc, tol| {
        acc + match tol {
            Tolerance::Linear(linear) => linear.distance.dim,
            Tolerance::Float(_) => 0.0,
        }
    });

    let worst_case_pos = state.tolerance_loop.iter().fold(0.0, |acc, tol| {
        acc + f64::abs(match tol {
            Tolerance::Linear(linear) => linear.distance.tol_pos,
            Tolerance::Float(float) => f64::max(
                0.0,
                ((float.hole.dim + float.hole.tol_pos) - (float.pin.dim - float.pin.tol_neg)) / 2.0,
            ),
        })
    });

    let worst_case_neg = state.tolerance_loop.iter().fold(0.0, |acc, tol| {
        acc + f64::abs(match tol {
            Tolerance::Linear(linear) => linear.distance.tol_neg,
            Tolerance::Float(float) => f64::max(
                0.0,
                ((float.hole.dim + float.hole.tol_pos) - (float.pin.dim - float.pin.tol_neg)) / 2.0,
            ),
        })
    });

    dbg!(
        worst_case_dim,
        worst_case_neg,
        worst_case_pos,
        result_stddev_neg,
        result_stddev_pos
    );

    let worst_case_upper = worst_case_dim + worst_case_pos;
    let worst_case_lower = worst_case_dim - worst_case_neg;

    Ok(McResults {
        mean: result_mean,
        tolerance_pos: result_tol_pos,
        tolerance_neg: result_tol_neg,
        stddev_pos: result_stddev_pos,
        stddev_neg: result_stddev_neg,
        iterations: real_iters,
        worst_case_upper,
        worst_case_lower,
    })
}

impl Tolerance {
    #[inline(always)]
    pub fn mc_tolerance(&self) -> f64 {
        match self {
            Tolerance::Linear(val) => val.mc_tolerance(),
            Tolerance::Float(val) => val.mc_tolerance(),
        }
    }
}

/// Generate a sample for each object in the tolerance collection, n_iterations times. Then sum
/// the results for each iteration, resulting in stackup for that iteration of the simulation.
pub fn compute_stackup(tol_collection: Vec<Tolerance>, n_iterations: usize) -> Vec<f64> {
    // Make a local clone of the tolerance collection so the borrow is not returned while the
    //  threads are using the collection.
    let tc_local = tol_collection;
    // Store output in `samples` vector, appending each tol_collection's output
    let n_tols = tc_local.len();
    let mut samples: Vec<f64> = Vec::with_capacity(n_tols * n_iterations);
    let (tx, rx) = mpsc::channel();
    // For each tolerance object generate n samples, dividing the work between multiple threads.
    for tol_struct in tc_local {
        let n_threads = 4;
        for _i in 0..n_threads {
            // Create a thread local copy of the thread communication sender for ownership reasons.
            let tx_local = mpsc::Sender::clone(&tx);
            thread::spawn(move || {
                // Make `result` thread local for better performance.
                let mut result: Vec<f64> = Vec::new();
                for _i in 0..n_iterations / n_threads {
                    result.push(tol_struct.mc_tolerance());
                }
                tx_local.send(result).unwrap();
            });
        }
        for _i in 0..n_threads {
            samples.extend_from_slice(&rx.recv().unwrap());
        }
    }

    let mut result: Vec<f64> = Vec::with_capacity(n_iterations);

    for i in 0..n_iterations {
        let mut stackup: f64 = 0.0;
        for j in 0..n_tols {
            stackup += samples[i + j * n_iterations];
        }
        result.push(stackup);
    }
    result
}

#[allow(dead_code)]
impl Tolerance {
    fn compute_multiplier(&mut self) {
        match self {
            Tolerance::Linear(tol) => tol.compute_multiplier(),
            Tolerance::Float(tol) => tol.compute_multiplier(),
        }
    }
}

pub trait MonteCarlo: Send + Sync + 'static {
    fn mc_tolerance(&self) -> f64;
    fn compute_multiplier(&mut self);
    //fn get_name(&self) -> &str;
}
impl MonteCarlo for LinearTL {
    fn mc_tolerance(&self) -> f64 {
        self.distance.dim
            + self
                .distance
                .sample_mc(DistributionParam::Normal, BoundingParam::KeepAll)
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
    fn compute_multiplier(&mut self) {
        self.distance.compute_multiplier();
    }
}
impl MonteCarlo for FloatTL {
    fn mc_tolerance(&self) -> f64 {
        let hole_sample = self
            .hole
            .sample_mc(DistributionParam::Normal, BoundingParam::KeepAll);
        let pin_sample = self
            .pin
            .sample_mc(DistributionParam::Normal, BoundingParam::KeepAll);
        let hole_pin_slop = (hole_sample - pin_sample) / 2.0;
        if hole_pin_slop <= 0.0 {
            0.0
        } else {
            DimTol::new_normal(0.0, hole_pin_slop, hole_pin_slop, self.sigma)
                .sample_mc(DistributionParam::Uniform, BoundingParam::KeepAll)
        }
    }
    fn compute_multiplier(&mut self) {
        self.hole.compute_multiplier();
        self.pin.compute_multiplier();
    }
    //fn get_name(&self) -> &str {
    //    &self.name
    //}
}

pub enum DistributionParam {
    Normal,
    Uniform,
}

#[allow(dead_code)]
pub enum BoundingParam {
    DiscardOutOfSpec,
    KeepAll,
}

impl DimTol {
    /// Generate a random sample of a given dimension
    fn sample_mc(&self, distribution: DistributionParam, bounding: BoundingParam) -> f64 {
        match distribution {
            DistributionParam::Normal => match bounding {
                BoundingParam::DiscardOutOfSpec => self.rand_bound_norm(),
                BoundingParam::KeepAll => self.rand_unbound_norm(),
            },
            DistributionParam::Uniform => match bounding {
                BoundingParam::DiscardOutOfSpec => self.rand_bound_uniform(),
                BoundingParam::KeepAll => self.rand_unbounded_uniform(),
            },
        }
    }

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
    fn rand_unbound_norm(&self) -> f64 {
        let mut sample: f64 = thread_rng().sample(StandardNormal);
        sample *= self.tol_multiplier;
        sample
    }
    fn rand_bound_uniform(&self) -> f64 {
        let mut sample: f64 = thread_rng().sample(Uniform::new_inclusive(-1.0, 1.0));
        sample = (sample * (self.tol_neg + self.tol_pos)) - self.tol_neg;
        // TODO: limit number of checks and error out if needed to escape infinite loop
        while sample < -self.tol_neg || sample > self.tol_pos {
            sample = thread_rng().sample(StandardNormal);
            sample = (sample * (self.tol_neg + self.tol_pos)) - self.tol_neg;
        }
        sample
    }
    fn rand_unbounded_uniform(&self) -> f64 {
        let mut sample: f64 = thread_rng().sample(Uniform::new_inclusive(-1.0, 1.0));
        sample = (sample * (self.tol_neg + self.tol_pos)) - self.tol_neg;
        sample
    }

    /// Precompute constant in monte carlo equation
    fn compute_multiplier(&mut self) {
        self.tol_multiplier = (self.tol_pos + self.tol_neg) / 2.0 / self.sigma;
    }
}

/// Data for testing purposes
#[allow(dead_code)]
pub fn test_data() -> State {
    let parameters = Parameters {
        assy_sigma: 4.0,
        n_iterations: 10000000,
    };

    let mut model = State::new(parameters);

    model.add(Tolerance::Linear(LinearTL::new(DimTol::new_normal(
        65.88, 0.17, 0.17, 3.0,
    ))));

    model.add(Tolerance::Float(FloatTL::new(
        DimTol::new_normal(2.50, 0.1, 0.0, 3.0),
        DimTol::new_normal(3.0, 0.08, 0.22, 3.0),
        3.0,
    )));

    model.add(Tolerance::Float(FloatTL::new(
        DimTol::new_normal(2.50, 0.1, 0.0, 3.0),
        DimTol::new_normal(3.0, 0.08, 0.22, 3.0),
        3.0,
    )));

    model
}
