use super::parts::errors::CollectionError;
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

pub struct ChartBuilder {
    stack_leniency: Option<Ratio<i64>>,
    slider_multiplier: Option<Ratio<i64>>,
    slider_tick_rate: Option<Ratio<i64>>,
    timing_points: Option<Vec<TimingPoint>>,
    hit_objects: Option<Vec<HitObject>>,
}
impl ChartBuilder {
    pub fn new() -> Self {
        Self {
            stack_leniency: None,
            slider_multiplier: None,
            slider_tick_rate: None,
            timing_points: None,
            hit_objects: None,
        }
    }
    pub fn chart(self) -> Result<Chart, CollectionError> {
        let missing_fields = [
            if self.stack_leniency.is_none() {
                Some("StackLeniency")
            } else {
                None
            },
            if self.slider_multiplier.is_none() {
                Some("SliderMultiplier")
            } else {
                None
            },
            if self.slider_tick_rate.is_none() {
                Some("SliderTickRate")
            } else {
                None
            },
            if self.timing_points.is_none() {
                Some("TimingPoints")
            } else {
                None
            },
            if self.hit_objects.is_none() {
                Some("HitObjects")
            } else {
                None
            },
        ]
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<&str>>()
        .join(", ");
        if missing_fields.len() > 0 {
            return Err(CollectionError::MissingField {
                field: format!("{{{}}}", missing_fields),
                collection: "Chart".into(),
            });
        }
        Ok(Chart {
            stack_leniency: self.stack_leniency.unwrap(),
            slider_multiplier: self.slider_multiplier.unwrap(),
            slider_tick_rate: self.slider_tick_rate.unwrap(),
            timing_points: self.timing_points.unwrap(),
            hit_objects: self.hit_objects.unwrap(),
        })
    }
}
