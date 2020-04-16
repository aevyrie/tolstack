extern crate rand_distr;
extern crate rand;
extern crate statistical;

use rand::prelude::*;
use rand_distr::StandardNormal;
use statistical::*;

fn main() {
    // Simulation parameters
    let part_sigma_level:f64 = 4.0;
    let assy_sigma_level:f64 = 3.0;
    let n_interations:usize = 1000000;
    let mut rand_sample = vec![0f64; n_interations];
    for i in 0..n_interations {
        rand_sample[i] = rand_bound_norm(1.0, part_sigma_level);
    }

    println!("Mean: {}", mean(&rand_sample));

    for val in rand_sample.iter() {
        //println!("{}", val);
    }
}

fn rand_bound_norm(tolerance:f64, sigma_level:f64) -> f64 {
    let mut sample:f64 = thread_rng().sample(StandardNormal);
    while sample.abs() > tolerance {
        sample = thread_rng().sample(StandardNormal);
    }
    sample
}

pub struct Dim{
    dimension:f64,
    tolerance:f64,
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

struct FloatTL {
    name:String,
    hole:Dim,
    pin:Dim,
}

struct CompoundFloatTL {
    name:String,
    datumstart:Dim,
    datumend:Dim,
    floatlist:Vec<OffsetFloat>,
}