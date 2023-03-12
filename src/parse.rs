pub mod errors;

pub use super::chart::*;
pub use super::customization::*;
pub use super::difficulty::*;
pub use super::editor::*;
pub use super::filedata::*;
pub use super::metadata::*;
pub use errors::*;

pub fn parse_str(
    s: &str,
    chart: bool, //                   [General]                  [Difficulty]        [TimingPoints]         [HitObjects]
    customization: bool, //           [General]                              [Events]              [Colours]
    difficulty: bool,    //                                      [Difficulty]
    editor: bool,        //                    [Editor]
    filedata: bool,      // [Preamble][General]
    metadata: bool,      //                            [Metadata]
) -> Result<
    (
        Option<Chart>,
        Option<Customization>,
        Option<Difficulty>,
        Option<Editor>,
        Option<Filedata>,
        Option<Metadata>,
    ),
    ParseError,
> {
    // Chart section
    let mut mode = None;
    let mut stack_leniency = None;
    let mut slider_multiplier = None;
    let mut slider_tick_rate = None;
    // Random initial capacities.
    // Could run analytics to find a more optimal value.
    let mut timing_points = Vec::<TimingPoint>::with_capacity(100);
    let mut hit_objects = Vec::<HitObject>::with_capacity(100);
    // Customization section
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
    let mut backgrounds = vec![];
    let mut breaks = vec![];
    let mut colors = vec![];
    // Difficulty section
    let mut circle_size = None;
    let mut hpdrain_rate = None;
    let mut overall_difficulty = None;
    let mut approach_rate = None;
    // Editor section
    let mut bookmarks = None;
    let mut distance_spacing = None;
    let mut beat_divisor = None;
    let mut grid_size = None;
    let mut timeline_zoom = None;
    // Filedata section
    let mut file_format = None;
    let mut audio_filename = None;
    let mut audio_lead_in = None;
    let mut audio_hash = None;
    let mut preview_time = None;
    let mut countdown_offset = None;
    // Metadata section
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
        } else {
            // Otherwise, parse the line.
            match section {
                "[Preamble]" => {
                    if filedata {
                        // Check for the version.
                        if let Some((_, rhs)) = line.split_once('v') {
                            if let Ok(version) = rhs.parse::<u8>() {
                                file_format = Some(version);
                                break;
                            }
                        } else {
                            return Err(ParseError::InvalidLine {
                                line: line.into(),
                                section: section.into(),
                            });
                        }
                    }
                }
                "[General]" => {
                    if chart | customization | filedata {
                        if let Some((key, value)) = line.split_once(':') {
                            let key = key.trim();
                            let value = value.trim();
                            if chart {
                                match key {
                                    "Mode" => {
                                        mode = match key {
                                            "0" => Some(Mode::Osu),
                                            "1" => Some(Mode::Taiko),
                                            "2" => Some(Mode::Catch),
                                            "3" => Some(Mode::Mania),
                                            _ => {
                                                return Err(ParseError::InvalidToken {
                                                    token: key.into(),
                                                    type_name: "Mode".into(),
                                                })
                                            }
                                        };
                                    }
                                }
                            }
                            if customization {
                                match key {
                                    "SampleSet" => {}
                                }
                            }
                            if filedata {
                                match key {
                                    _ => {}
                                }
                            }
                        } else {
                            return Err(ParseError::InvalidLine {
                                line: line.into(),
                                section: section.into(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Create all the collections
    let chart = if chart {
        Some(Chart {
            mode: match mode {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "mode".into(),
                        collection: "chart".into(),
                    })
                }
            },
            stack_leniency: match stack_leniency {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "stack_leniency".into(),
                        collection: "chart".into(),
                    })
                }
            },
            slider_multiplier: match slider_multiplier {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "slider_multiplier".into(),
                        collection: "chart".into(),
                    })
                }
            },
            slider_tick_rate: match slider_tick_rate {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "slider_tick_rate".into(),
                        collection: "chart".into(),
                    })
                }
            },
            timing_points,
            hit_objects,
        })
    } else {
        None
    };

    let customization = if customization {
        Some(Customization {
            sample_set: sample_set.unwrap_or_else(|| SampleSet::Normal),
            letterbox_in_breaks: letterbox_in_breaks.unwrap_or_else(|| false),
            story_fire_in_front: story_fire_in_front.unwrap_or_else(|| true),
            use_skin_sprites: use_skin_sprites.unwrap_or_else(|| false),
            always_show_play_field: always_show_play_field.unwrap_or_else(|| false),
            overlay_position: overlay_position.unwrap_or_else(|| OverlayPosition::NoChange),
            skin_preference,
            epilepsy_warning: epilepsy_warning.unwrap_or_else(|| false),
            countdown: countdown.unwrap_or_else(|| Countdown::Normal),
            special_style: special_style.unwrap_or_else(|| false),
            widescreen_storyboard: widescreen_storyboard.unwrap_or_else(|| false),
            samples_match_playback_rate: samples_match_playback_rate.unwrap_or_else(|| false),
            backgrounds,
            breaks,
            colors,
        })
    } else {
        None
    };

    let difficulty = if difficulty {
        Some(Difficulty {
            circle_size: match circle_size {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "circle_size".into(),
                        collection: "difficulty".into(),
                    })
                }
            },
            hpdrain_rate: match hpdrain_rate {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "hpdrain_rate".into(),
                        collection: "difficulty".into(),
                    })
                }
            },
            overall_difficulty: match overall_difficulty {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "overall_difficulty".into(),
                        collection: "difficulty".into(),
                    })
                }
            },
            approach_rate: match approach_rate {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "approach_rate".into(),
                        collection: "difficulty".into(),
                    })
                }
            },
        })
    } else {
        None
    };

    let editor = if editor {
        Some(Editor {
            bookmarks,
            distance_spacing,
            beat_divisor,
            grid_size,
            timeline_zoom,
        })
    } else {
        None
    };

    let filedata = if filedata {
        Some(Filedata {
            file_format: match file_format {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "file_format".into(),
                        collection: "filedata".into(),
                    })
                }
            },
            audio_filename: match audio_filename {
                Some(value) => value,
                None => {
                    return Err(ParseError::MissingField {
                        field: "audio_filename".into(),
                        collection: "filedata".into(),
                    })
                }
            },
            audio_lead_in: audio_lead_in.unwrap_or_else(|| 0),
            audio_hash,
            preview_time: preview_time.unwrap_or_else(|| -1),
            countdown_offset: countdown_offset.unwrap_or_else(|| 0),
        })
    } else {
        None
    };

    let metadata = if metadata {
        Some(Metadata {
            title,
            title_unicode,
            artist,
            artist_unicode,
            creator,
            version,
            source,
            tags,
            beatmap_id,
            beatmap_set_id,
        })
    } else {
        None
    };
    Ok((chart, customization, difficulty, editor, filedata, metadata))
}
