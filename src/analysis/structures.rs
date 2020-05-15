/// Contains structures used to define tolerances in a tolerance loop.
use serde_derive::*;

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct DimTol{
    pub dim: f64,
    pub tol_pos: f64,
    pub tol_neg: f64,
    pub tol_multiplier: f64,
    pub sigma: f64,
}
impl DimTol{
    pub fn new(dim: f64, tol_pos: f64, tol_neg: f64, sigma: f64) -> Self {
        let tol_multiplier: f64 = (tol_pos + tol_neg) / 2.0 / sigma;
        DimTol{
            dim,
            tol_pos,
            tol_neg,
            tol_multiplier,
            sigma,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Tolerance{
    Linear(LinearTL),
    Float(FloatTL),
    //Compound(CompoundFloatTL),
}
impl Default for Tolerance {
    fn default() -> Self {
        Tolerance::Linear(LinearTL::default())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct LinearTL {
    pub distance: DimTol,
}
impl  LinearTL {
    pub fn new(distance: DimTol) -> Self {
        LinearTL {
            distance,
        }
    }
}


#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct FloatTL {
    pub hole: DimTol,
    pub pin: DimTol,
    pub sigma: f64,
}
impl  FloatTL {
    pub fn new(hole: DimTol, pin: DimTol, sigma: f64) -> Self {
        FloatTL {
            hole,
            pin,
            sigma,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct CompoundFloatTL {
    pub datum_start: DimTol,
    pub datum_end: DimTol,
    pub float_list: OffsetFloat,
    pub sigma: f64,
}
impl  CompoundFloatTL {
    pub fn new(datumtime_start: DimTol, datumend: DimTol, floatlist: OffsetFloat, sigma: f64) -> Self {
        CompoundFloatTL{
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