#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub filename: String,
    pub xoffset: i64,
    pub yoffset: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Break {
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
    pub _type: CurveType,
    pub points: Vec<(i64, i64)>,
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
    pub kiai: bool, // 1 on
    // 2 is unused
    pub ommit_barline: bool, // 4 on
}

#[derive(Debug, Clone, PartialEq)]
pub struct HitSample {
    pub normal_set: SampleSet,
    pub addition_set: SampleSet,
    pub index: i64,
    pub volume: i64, // From 0 to 100.
    pub filename: Option<String>,
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
    pub normal: bool,
    pub whistle: bool,
    pub finish: bool,
    pub clap: bool,
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
    pub object_type: ObjectType, // 0 circle, 1 slider, 3 spinner, 7 mania hold
    pub new_combo: bool,         // 2
    pub color_skip: u8,          // 4-6 -- Actually a 3 bit big-endian uint
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
    ManiaHold,
}
