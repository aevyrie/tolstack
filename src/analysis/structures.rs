/// Contains structures used to define tolerances in a tolerance loop.
use serde_derive::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct DimTol {
    pub dim: f64,
    pub tol_pos: f64,
    pub tol_neg: f64,
    pub tol_multiplier: f64,
    pub sigma: f64,
}
impl DimTol {
    pub fn new(dim: f64, tol_pos: f64, tol_neg: f64, sigma: f64) -> Self {
        let tol_multiplier: f64 = (tol_pos + tol_neg) / 2.0 / sigma;
        DimTol {
            dim,
            tol_pos,
            tol_neg,
            tol_multiplier,
            sigma,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Tolerance {
    Linear(LinearTL),
    Float(FloatTL),
    //Compound(CompoundFloatTL),
}
impl Default for Tolerance {
    fn default() -> Self {
        Tolerance::Linear(LinearTL::default())
    }
}
impl Tolerance {
    pub fn distance(&self) -> f64 {
        match self {
            Tolerance::Linear(linear) => linear.distance.dim,
            Tolerance::Float(_) => 0f64,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct LinearTL {
    pub distance: DimTol,
}
impl LinearTL {
    pub fn new(distance: DimTol) -> Self {
        LinearTL { distance }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct FloatTL {
    pub hole: DimTol,
    pub pin: DimTol,
    pub sigma: f64,
}
impl FloatTL {
    pub fn new(hole: DimTol, pin: DimTol, sigma: f64) -> Self {
        FloatTL { hole, pin, sigma }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct CompoundFloatTL {
    pub datum_start: DimTol,
    pub datum_end: DimTol,
    pub float_list: OffsetFloat,
    pub sigma: f64,
}
impl CompoundFloatTL {
    pub fn new(
        datumtime_start: DimTol,
        datumend: DimTol,
        floatlist: OffsetFloat,
        sigma: f64,
    ) -> Self {
        CompoundFloatTL {
            datum_start: datumtime_start,
            datum_end: datumend,
            float_list: floatlist,
            sigma,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct OffsetFloat {
    pub hole: DimTol,
    pub pin: DimTol,
    pub hole_spacing: DimTol,
    pub pin_spacing: DimTol,
}
impl OffsetFloat {
    pub fn new(hole: DimTol, pin: DimTol, hole_spacing: DimTol, pin_spacing: DimTol) -> Self {
        OffsetFloat {
            hole,
            pin,
            hole_spacing,
            pin_spacing,
        }
    }
}

/// Structure used to hold simulation input parameters
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameters {
    pub assy_sigma: f64,
    pub n_iterations: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnalysisResults {
    monte_carlo: Option<McResults>,
    rss: Option<RssResults>,
}
impl AnalysisResults {
    pub fn monte_carlo(&self) -> &Option<McResults> {
        &self.monte_carlo
    }
    pub fn rss(&self) -> &Option<RssResults> {
        &self.rss
    }
    pub fn export(&self) -> Vec<f64> {
        if let Some(mc_result) = &self.monte_carlo {
            if let Some(rss_result) = &self.rss {
                let mut result = Vec::new();
                result.push(mc_result.mean);
                result.push(mc_result.tolerance_pos);
                result.push(mc_result.tolerance_neg);
                result.push(mc_result.stddev_pos);
                result.push(mc_result.stddev_neg);
                result.push(mc_result.worst_case_lower);
                result.push(mc_result.worst_case_upper);
                result.push(rss_result.mean);
                result.push(rss_result.tolerance_pos);
                result.push(rss_result.tolerance_neg);
                return result;
            }
        }
        // If no result is generated, return an empty vec
        Vec::new()
    }
}
impl From<(McResults, RssResults)> for AnalysisResults {
    fn from(results: (McResults, RssResults)) -> Self {
        let (monte_carlo, rss) = results;
        AnalysisResults {
            monte_carlo: Some(monte_carlo),
            rss: Some(rss),
        }
    }
}
impl Default for AnalysisResults {
    fn default() -> Self {
        AnalysisResults {
            monte_carlo: None,
            rss: None,
        }
    }
}

//todo remove pub and add a getter
/// Structure used to hold the output of a Monte Carlo simulaion
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct McResults {
    pub mean: f64,
    pub tolerance_pos: f64,
    pub tolerance_neg: f64,
    pub stddev_pos: f64,
    pub stddev_neg: f64,
    pub iterations: usize,
    pub worst_case_upper: f64,
    pub worst_case_lower: f64,
}
impl McResults {}

/// Structure used to hold the output of an RSS calculation
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RssResults {
    mean: f64,
    tolerance_pos: f64,
    tolerance_neg: f64,
}
impl RssResults {
    pub fn new(mean: f64, tolerance_pos: f64, tolerance_neg: f64) -> Self {
        RssResults {
            mean,
            tolerance_pos,
            tolerance_neg,
        }
    }
    pub fn mean(&self) -> f64 {
        self.mean
    }
    pub fn tolerance_pos(&self) -> f64 {
        self.tolerance_pos
    }
    pub fn tolerance_neg(&self) -> f64 {
        self.tolerance_neg
    }
}

/// Holds the working state of the simulation, including inputs and outputs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct State {
    pub parameters: Parameters,
    pub tolerance_loop: Vec<Tolerance>,
    pub results: AnalysisResults,
}
impl State {
    pub fn new(parameters: Parameters) -> Self {
        State {
            parameters,
            tolerance_loop: Vec::new(),
            results: AnalysisResults::default(),
        }
    }
    pub fn serialize_json(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let filename_full = &[filename, ".json"].concat();
        let path = Path::new(filename_full);
        Self::file_write(path, data)?;
        Ok(())
    }
    pub fn file_write(path: &Path, data: String) -> Result<(), Box<dyn Error>> {
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("Couldn't create {}: {}", display, why.to_string()),
            Ok(file) => file,
        };

        match file.write_all(data.as_bytes()) {
            Err(why) => panic!("Couldn't write to {}: {}", display, why.to_string()),
            Ok(_) => println!("Saving data to {}", display),
        }
        Ok(())
    }
    pub fn add(&mut self, tolerance: Tolerance) {
        self.tolerance_loop.push(tolerance);
    }
    pub fn clear_inputs(&mut self) {
        self.tolerance_loop = Vec::new();
    }
}
impl Default for State {
    fn default() -> Self {
        let parameters = Parameters {
            assy_sigma: 4.0,
            n_iterations: 1000000,
        };
        State::new(parameters)
    }
}
