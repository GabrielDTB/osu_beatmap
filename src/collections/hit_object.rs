pub use crate::parts::Curve;
pub use crate::parts::CurveType;
pub use crate::parts::HitSample;
pub use crate::parts::HitSound;
pub use crate::parts::ObjectType;
pub use crate::parts::SampleSet;
pub use crate::parts::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum HitObject {
    Circle(Circle),
    Slider(Slider),
    Spinner(Spinner),
    ManiaHold(ManiaHold),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub x: i64,
    pub y: i64,
    pub time: i64,
    pub flags: Type,
    pub hit_sound: HitSound,
    pub hit_sample: HitSample,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    pub x: i64,
    pub y: i64,
    pub time: i64,
    pub flags: Type,
    pub hit_sound: HitSound,
    pub curve: Curve,
    pub slides: i64,
    pub length: f64,
    pub edge_sounds: Vec<HitSound>,
    pub edge_sets: Vec<(SampleSet, SampleSet)>,
    pub hit_sample: HitSample,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spinner {
    pub x: i64,
    pub y: i64,
    pub time: i64,
    pub flags: Type,
    pub hit_sound: HitSound,
    pub end_time: i64,
    pub hit_sample: HitSample,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManiaHold {
    pub x: i64,
    pub y: i64,
    pub time: i64,
    pub flags: Type,
    pub hit_sound: HitSound,
    pub end_time: i64,
    pub hit_sample: HitSample,
}
