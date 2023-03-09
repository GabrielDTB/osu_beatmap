#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    filename: String,
    xoffset: i64,
    yoffset: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Break {
    start_time: i64,
    end_time: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Countdown {
    None,
    Normal,
    Half,
    Double,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Curve {
    _type: CurveType,
    points: Vec<(i64, i64)>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CurveType {
    Bezier,
    Centripetal,
    Linear,
    Perfect,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Effects {
    kiai: bool, // 1 on
    // 2 is unused
    ommit_barline: bool, // 4 on
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HalfHitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
}
impl Default for HalfHitSample {
    fn default() -> Self {
        Self {
            normal_set: SampleSet::Default,
            addition_set: SampleSet::Default,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitSample {
    normal_set: SampleSet,
    addition_set: SampleSet,
    index: i64,
    volume: i64, // From 0 to 100.
    filename: Option<String>,
}
impl Default for HitSample {
    fn default() -> Self {
        Self {
            normal_set: SampleSet::Default,
            addition_set: SampleSet::Default,
            index: 0,
            volume: 0,
            filename: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitSound {
    normal: bool,
    whistle: bool,
    finish: bool,
    clap: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SampleSet {
    Default,
    Normal,
    Soft,
    Drum,
}

/// The Type struct is a direct representation of the type field in the .osu file.
// TODO: Decide whether to only use this as an internal type. Reason: it sucks
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    object_type: ObjectType, // 0 circle, 1 slider, 3 spinner, 7 mania hold
    new_combo: bool,         // 2
    color_skip: u8,          // 4-6 -- Actually a 3 bit big-endian uint
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
    ManiaHold,
}
