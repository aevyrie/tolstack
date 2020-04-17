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
    // Simulation parameters
    let part_sigma_level:f64 = 4.0;
    let assy_sigma_level:f64 = 3.0;
    let n_iterations:usize = 1000000;
    //println!("Mean: {}", mean(&rand_sample));

    let mut tol_collection:Vec<Box<dyn MonteCarlo>> = Vec::new();
    tol_collection.push( Box::from(
        LinearTL{ 
            name: "A: Actuator tip to hard stop".to_string(), 
            distance: Dim{ 
                dim: 5.58, 
                tol_up: 0.03,
                tol_down: 0.03,
            },
        }
    ));
    tol_collection.push( Box::from(
        LinearTL{ 
            name: "B: Hard stop to MF pin".to_string(), 
            distance: Dim{ 
                dim: -25.78, 
                tol_up: 0.07,
                tol_down: 0.07,
            },
        }
    ));
    tol_collection.push( Box::from(
        FloatTL{ 
            name: "C: MF pin float".to_string(), 
            hole: Dim{ 
                dim: 2.18, 
                tol_up: 0.03,
                tol_down: 0.03,
            },
            pin: Dim{
                dim: 2.13,
                tol_up: 0.05,
                tol_down: 0.05,
            }
        }
    ));
    tol_collection.push( Box::from(
        LinearTL{ 
            name: "D: PCB MF hole to component hole".to_string(), 
            distance: Dim{ 
                dim: 14.58, 
                tol_up: 0.05,
                tol_down: 0.05,
            },
        }
    ));
    tol_collection.push( Box::from(
        CompoundFloatTL{ 
            name: "E: Switch component pins float".to_string(), 
            datumstart: Dim{ 
                dim: 1.2, 
                tol_up: 0.03,
                tol_down: 0.03,
            },
            datumend: Dim{
                dim: 1.0,
                tol_up: 0.03,
                tol_down: 0.03,
            },
            floatlist: vec!(OffsetFloat{
                hole: Dim{ 
                    dim: 0.972, 
                    tol_up: 0.03,
                    tol_down: 0.03,
                },
                pin: Dim{
                    dim: 0.736,
                    tol_up: 0.03,
                    tol_down: 0.03,
                },
                holespacing: Dim{ 
                    dim: 2.5, 
                    tol_up: 0.05,
                    tol_down: 0.05,
                },
                pinspacing: Dim{
                    dim: 2.5,
                    tol_up: 0.3,
                    tol_down: 0.3,
                },
            })

        }
    ));
    tol_collection.push( Box::from(
        LinearTL{ 
            name: "E': pin to pin".to_string(), 
            distance: Dim{ 
                dim: 2.5, 
                tol_up: 0.3,
                tol_down: 0.3,
            },
        }
    ));
    tol_collection.push( Box::from(
        LinearTL{ 
            name: "F: Switch pin to button surface".to_string(), 
            distance: Dim{ 
                dim: 3.85, 
                tol_up: 0.25,
                tol_down: 0.25,
            },
        }
    ));
    tol_collection.push( Box::from(
        LinearTL{ 
            name: "G: Button actuation".to_string(), 
            distance: Dim{ 
                dim: -0.3, 
                tol_up: 0.15,
                tol_down: 0.15,
            },
        }
    ));

    //360ms

    for _1 in 0..1 {
        let start = Instant::now();
        let stack = compute_stackup(&tol_collection, &part_sigma_level, &n_iterations);
        let stack_mean = mean(&stack);
        let stack_stddev = standard_deviation(&stack, Some(stack_mean));
        let stack_tol = stack_stddev * assy_sigma_level;

        let duration = start.elapsed();
        println!("Mean:{:.4} Stddev: {:.4}; Tol: {:.4}; Samples: {}; Duration: {:?}", 
            stack_mean, stack_stddev, stack_tol, stack.len().to_formatted_string(&Locale::en), duration);

        //println!("{:?}", stack);
    }
}

fn rand_bound_norm(tol_up:f64, tol_down:f64, sigma_level:&f64) -> f64 {
    let mut sample:f64 = std::f64::MAX;
    while sample < -tol_down || sample > tol_up {
        sample = thread_rng().sample(StandardNormal);
        sample *= ((tol_up+tol_down)/2.0) / sigma_level;
    }
    //println!("{}", sample);
    sample
}

fn compute_stackup(tol_collection:&Vec<Box<dyn MonteCarlo>>, part_sigma_level:&f64, n_iterations:&usize) -> Vec<f64> {
    let vec_length = n_iterations;
    let vec_height = tol_collection.len();
    let mut samples:Vec<f64> =  Vec::with_capacity(vec_height * vec_length);
    for tol_struct in tol_collection {
        //let start = Instant::now();
        for _i in 0..*n_iterations {
            samples.push(tol_struct.mc_tolerance(part_sigma_level));
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
    fn mc_tolerance(&self, sigma_level:&f64) -> f64;
}

pub struct Dim{
    dim:f64,
    tol_up:f64,
    tol_down:f64,
}

struct OffsetFloat {
    hole:Dim,
    pin:Dim,
    holespacing:Dim,
    pinspacing:Dim,
}

struct LinearTL {
    name:String,
    distance:Dim,
}
impl MonteCarlo for LinearTL {
    fn mc_tolerance(&self, sigma_level:&f64) -> f64 {
        //let mut result:Vec<f64> = Vec::new();
        self.distance.dim + rand_bound_norm(self.distance.tol_up, self.distance.tol_down, sigma_level)
    }
}

struct FloatTL {
    name:String,
    hole:Dim,
    pin:Dim,
}
impl MonteCarlo for FloatTL {
    fn mc_tolerance(&self, sigma_level:&f64) -> f64 {
        //let mut result:Vec<f64> = Vec::new();
        let hole_tol = rand_bound_norm(self.hole.tol_up, self.hole.tol_down, sigma_level);
        let pin_tol = rand_bound_norm(self.pin.tol_up, self.pin.tol_down, sigma_level);
        let hole_pin_slop = ((self.hole.dim + hole_tol) - (self.pin.dim + pin_tol))/2.0;
        if hole_pin_slop <= 0.0 {
            0.0
        } else {
            rand_bound_norm(hole_pin_slop, hole_pin_slop, sigma_level)
        }
    }
}

struct CompoundFloatTL {
    name:String,
    datumstart:Dim,
    datumend:Dim,
    floatlist:Vec<OffsetFloat>,
}
impl MonteCarlo for CompoundFloatTL {
    fn mc_tolerance(&self, sigma_level:&f64) -> f64 {
        //let mut result:Vec<f64> = Vec::new();
        let ds = self.datumstart.dim + rand_bound_norm(self.datumstart.tol_up, self.datumstart.tol_down, sigma_level);
        let de = self.datumend.dim + rand_bound_norm(self.datumend.tol_up, self.datumend.tol_down, sigma_level);
        let datum_hole:f64;
        let datum_pin:f64;
        if self.datumstart.dim > self.datumend.dim {
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

        for float_pair in self.floatlist.iter() {
            let h = float_pair.hole.dim + rand_bound_norm(float_pair.holespacing.tol_up, float_pair.holespacing.tol_down, sigma_level);
            let p = float_pair.pin.dim + rand_bound_norm(float_pair.holespacing.tol_up, float_pair.holespacing.tol_down, sigma_level);
            let hs = float_pair.holespacing.dim + rand_bound_norm(float_pair.holespacing.tol_up, float_pair.holespacing.tol_down, sigma_level);
            let ps = float_pair.pinspacing.dim + rand_bound_norm(float_pair.holespacing.tol_up, float_pair.holespacing.tol_down, sigma_level);
            let clearance_dia = h - p;
            let misalignment = hs - ps;
            let clearance_r = clearance_dia + misalignment;
            let clearance_l = clearance_dia - misalignment;
            min_clearance_r = clamp(clearance_r, 0.0, min_clearance_r);
            min_clearance_l = clamp(clearance_l, 0.0, min_clearance_l);
        }

        //let clearance_total = min_clearance_r + min_clearance_l;
        let mut bias = (min_clearance_r - min_clearance_l)/2.0;
        let bias_dir = if self.datumstart.dim > self.datumend.dim {1.0} else {-1.0};
        bias *= bias_dir;

        bias + rand_bound_norm(min_clearance_r, min_clearance_l, sigma_level)
    }
}