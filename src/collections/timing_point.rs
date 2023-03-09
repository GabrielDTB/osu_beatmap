pub use crate::parts::Effects;
use crate::parts::SampleSet;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimingPoint {
    time: i64,
    beat_length: f64,
    meter: i64,
    sample_set: SampleSet,
    sample_index: i64,
    volume: i64,
    uninherited: bool,
    effects: Effects,
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
