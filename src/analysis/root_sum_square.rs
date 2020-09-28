use super::structures::*;
use std::error::Error;

pub async fn run(state: &State) -> Result<RssResults, Box<dyn Error>> {
    let mean: f64 = state
        .tolerance_loop
        .iter()
        .fold(0.0, |acc, tol| return acc + tol.distance());
    let tolerance_neg = state
        .tolerance_loop
        .iter()
        .fold(0.0, |acc, tol| {
            return acc
                + match tol {
                    Tolerance::Linear(linear) => {
                        (linear.distance.tol_neg / linear.distance.sigma).powi(2)
                    }
                    Tolerance::Float(float) => {
                        let hole_avg = (float.hole.tol_neg + float.hole.tol_pos) / 2.0;
                        // Divide by two because the hole dim is diametric
                        let hole_squared = ((hole_avg / 2.0) / float.pin.sigma).powi(2);
                        let pin_avg = (float.pin.tol_neg + float.pin.tol_pos) / 2.0;
                        // Divide by two because the pin dim is diametric
                        let pin_squared = ((pin_avg / 2.0) / float.pin.sigma).powi(2);
                        hole_squared + pin_squared
                    }
                };
        })
        .sqrt()
        * state.parameters.assy_sigma;
    let tolerance_pos = state
        .tolerance_loop
        .iter()
        .fold(0.0, |acc, tol| {
            return acc
                + match tol {
                    Tolerance::Linear(linear) => {
                        (linear.distance.tol_pos / linear.distance.sigma).powi(2)
                    }
                    Tolerance::Float(float) => {
                        let hole_avg = (float.hole.tol_neg + float.hole.tol_pos) / 2.0;
                        let hole_squared = ((hole_avg / 2.0) / float.pin.sigma).powi(2);
                        let pin_avg = (float.pin.tol_neg + float.pin.tol_pos) / 2.0;
                        let pin_squared = ((pin_avg / 2.0) / float.pin.sigma).powi(2);
                        hole_squared + pin_squared
                    }
                };
        })
        .sqrt()
        * state.parameters.assy_sigma;

    Ok(RssResults::new(mean, tolerance_pos, tolerance_neg))
}
