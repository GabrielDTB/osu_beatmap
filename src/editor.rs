pub use num::rational::Ratio;

pub struct Editor {
    pub bookmarks: Vec<i64>,
    pub distance_spacing: Option<Ratio<i64>>,
    pub beat_divisor: Option<i64>,
    pub grid_size: Option<i64>,
    pub timeline_zoom: Option<Ratio<i64>>,
}
