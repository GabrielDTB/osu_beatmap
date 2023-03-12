pub struct Filedata {
    pub file_format: u8,
    pub audio_filename: String,
    pub audio_lead_in: i64, // Default = 0
    pub audio_hash: Option<String>, // Deprecated
    pub preview_time: i64, // Default = -1
    pub countdown_offset: i64, // Default = 0
}
