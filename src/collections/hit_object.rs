pub use crate::parts::Curve;
pub use crate::parts::HalfHitSample;
pub use crate::parts::HitSample;
pub use crate::parts::HitSound;
pub use crate::parts::Type;

#[derive(Debug, Clone, PartialEq)]
pub enum HitObject {
    Circle(Circle),
    Slider(Slider),
    Spinner(Spinner),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    x: i64,
    y: i64,
    time: i64,
    flags: Type,
    hit_sound: HitSound,
    hit_sample: HitSample,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slider {
    x: i64,
    y: i64,
    time: i64,
    flags: Type,
    hit_sound: HitSound,
    curve: Curve,
    slides: i64,
    length: f64,
    edge_sounds: Vec<HitSound>,
    edge_sets: Vec<HalfHitSample>,
    hit_sample: HitSample,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spinner {
    x: i64,
    y: i64,
    time: i64,
    flags: Type,
    hit_sound: HitSound,
    end_time: i64,
    hit_sample: HitSample,
}
