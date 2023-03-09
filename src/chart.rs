pub use crate::collections::HitObject;
pub use crate::collections::TimingPoint;
use num::rational::Ratio;

#[derive(Debug, Clone, PartialEq)]
pub struct Chart {
    pub stack_leniency: Ratio<i64>,
    pub slider_multiplier: Ratio<i64>,
    pub slider_tick_rate: Ratio<i64>,
    pub timing_points: Vec<TimingPoint>,
    pub hit_objects: Vec<HitObject>,
}
