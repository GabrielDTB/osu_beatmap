use super::chart::*;
use super::customization::*;
use super::difficulty::*;
use super::editor::*;
use super::filedata::*;
use super::metadata::*;
use super::mode::*;

pub fn parse_str(
    s: &str,
    chart: bool, // Chart requires [General], [Difficulty, [TimingPoints], [HitObjects]
    customization: bool, // Customization requires [General], [Colours]
    difficulty: bool,
    editor: bool,
    filedata: bool,
    metadata: bool,
    mode: bool,
) -> Result<
    (
        Option<Chart>,
        Option<Customization>,
        Option<Difficulty>,
        Option<Editor>,
        Option<Filedata>,
        Option<Metadata>,
        Option<Mode>,
    ),
    ParseError,
> {
    // Chart section

    // Customization section
    let mut customization = None;
    let mut sample_set = None;
    let mut letterbox_in_breaks = None;
    let mut story_fire_in_front = None;
    let mut use_skin_sprites = None;
    let mut always_show_play_field = None;
    let mut overlay_position = None;
    let mut skin_preference = None;
    let mut epilepsy_warning = None;
    let mut countdown = None;
    let mut special_style = None;
    let mut widescreen_storyboard = None;
    let mut samples_match_playback_rate = None;
    let mut background = None;
    let mut breaks = None;
    let mut colors = None;
    // Difficulty section
    let mut difficulty = None;
    let mut circle_size = None;
    let mut hpdrain_rate = None;
    let mut overall_difficulty = None;
    let mut approach_rate = None;
    // Editor section
    let mut editor = None;
    let mut bookmarks = None;
    let mut distance_spacing = None;
    let mut beat_divisor = None;
    let mut grid_size = None;
    let mut timeline_zoom = None;
    // Filedata section
    let mut filedata = None;
    let mut file_format = None;
    let mut audio_filename = None;
    let mut audio_lead_in = None;
    let mut audio_hash = None;
    let mut preview_time = None;
    let mut countdown_offset = None;
    // Metadata section
    let mut metadata = None;
    let mut title = None;
    let mut title_unicode = None;
    let mut artist = None;
    let mut artist_unicode = None;
    let mut creator = None;
    let mut version = None;
    let mut source = None;
    let mut tags = None;
    let mut beatmap_id = None;
    let mut beatmap_set_id = None;
    // Mode section
    let mut mode = None;

    let sections = vec![
        "[General]",
        "[Editor]",
        "[Metadata]",
        "[Difficulty]",
        "[Events]",
        "[TimingPoints]",
        "[Colours]",
        "[HitObjects]",
    ];

    let mut section = "[Preamble]";
    for line in s.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        // Go the the next section if we find a section header.
        if let Some(position) = sections
            .iter()
            // Hopfully filtering by ascii fixes some encoding issues.
            .position(|x| *x == line.chars().filter(|l| l.is_ascii()).collect::<String>())
        {
            section = sections[position];
            sections = sections.drain(..position).collect();
            break;
        }
        // Otherwise, parse the line.
        match section {
            "[Preamble]" => {
                // Check for the version.
                if let Some((_, rhs)) = line.split_once('v') {
                    if let Ok(version) = rhs.parse::<u8>() {
                        file_format = Some(version);
                        break;
                    }
                }
                // If the line doesn't fit above, raise an error.
                return Err(ParseError::InvalidLine {
                    line: line.into(),
                    section: section.into(),
                });
            }

            "[General]" => {
                if let Some((key, value)) = line.split_once(':') {
                    match key {
                        "AudioFilename" => {
                            audio_filename = Some(value.into());
                            break;
                        }
                    }
                }
                // If the line doesn't fit above, raise an error.
                return Err(ParseError::InvalidLine {
                    line: line.into(),
                    section: section.into(),
                });
            }
        }
    }
    Ok((None, None, None, None, None, None, None))
}
