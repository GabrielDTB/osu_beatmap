pub use crate::parts::Background;
pub use crate::parts::Break;
pub use crate::parts::Color;
pub use crate::parts::Countdown;
pub use crate::parts::OverlayPosition;
pub use crate::parts::SampleSet;

pub struct Customization {
    pub sample_set: SampleSet,             // Default = Normal
    pub letterbox_in_breaks: bool,         // Default = 0
    pub story_fire_in_front: bool,         // Deprecated, Default = 1
    pub use_skin_sprites: bool,            // Default = 0
    pub always_show_play_field: bool,      // Deprecated, Default = 0
    pub overlay_position: OverlayPosition, // Default = NoChange
    pub skin_preference: Option<String>,   // No default. Is it optional? TODO
    pub epilepsy_warning: bool,            // Default = 0
    pub countdown: Countdown,              // Default = 1 (normal)
    pub special_style: bool,               // Default = 0
    pub widescreen_storyboard: bool,       // Default = 0
    pub samples_match_playback_rate: bool, // Default = 0
    pub backgrounds: Vec<Background>,
    pub breaks: Vec<Break>,
    pub colors: Vec<Color>,
}
