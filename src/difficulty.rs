#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Difficulty {
    pub circle_size: u8,
    pub hpdrain_rate: u8,
    pub overall_difficulty: u8,
    pub approach_rate: u8,
}
