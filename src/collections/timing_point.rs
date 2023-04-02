pub use crate::parts::Effects;
use crate::parts::SampleSet;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimingPoint {
    pub time: i64,
    pub beat_length: f64,
    pub meter: i64,
    pub sample_set: SampleSet,
    pub sample_index: i64,
    pub volume: i64,
    pub uninherited: bool,
    pub effects: Effects,
}
impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: 0,
            beat_length: 0.0,
            meter: 4,
            sample_set: SampleSet::Default,
            sample_index: 0,
            volume: 100,
            uninherited: true,
            effects: Effects {
                kiai: false,
                ommit_barline: false,
            },
        }
    }
}
