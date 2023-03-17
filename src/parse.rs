pub mod errors;

pub use super::chart::*;
pub use super::customization::*;
pub use super::difficulty::*;
pub use super::editor::*;
pub use super::filedata::*;
pub use super::metadata::*;
use bitvec::prelude::*;
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
    let mut timing_points = Vec::with_capacity(100);
    let mut hit_objects = Vec::with_capacity(100);
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
    let mut backgrounds = Vec::new();
    let mut breaks = Vec::new();
    let mut colors = Vec::new();
    // Difficulty section
    let mut circle_size = None;
    let mut hpdrain_rate = None;
    let mut overall_difficulty = None;
    let mut approach_rate = None;
    // Editor section
    let mut bookmarks = Vec::new();
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
    let mut tags = Vec::new();
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
            continue;
        } // Otherwise try to parse the line.
        match section {
            "[Preamble]" => {
                if !filedata {
                    continue;
                }
                // Check for the version.
                if let Some((_, rhs)) = line.split_once('v') {
                    if let Ok(version) = rhs.parse() {
                        file_format = Some(version);
                    }
                } else {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[General]" => {
                if !chart && !customization && !filedata {
                    continue;
                }
                let mut invalid_line = false;
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();
                    let mut key_matched = false;
                    if chart {
                        key_matched = true;
                        match key {
                            "Mode" => match value {
                                "0" => mode = Some(Mode::Osu),
                                "1" => mode = Some(Mode::Taiko),
                                "2" => mode = Some(Mode::Catch),
                                "3" => mode = Some(Mode::Mania),
                                _ => invalid_line = true,
                            },
                            _ => key_matched = false,
                        }
                    }
                    if customization && !key_matched {
                        key_matched = true;
                        match key {
                            "SampleSet" => match value {
                                "Default" => sample_set = Some(SampleSet::Default),
                                "Normal" => sample_set = Some(SampleSet::Normal),
                                "Soft" => sample_set = Some(SampleSet::Soft),
                                "Drum" => sample_set = Some(SampleSet::Drum),
                                _ => invalid_line = true,
                            },
                            "LetterboxInBreaks" => match value.parse() {
                                Ok(value) => letterbox_in_breaks = Some(value),
                                _ => invalid_line = true,
                            },
                            "StoryFireInFront" => match value.parse() {
                                Ok(value) => story_fire_in_front = Some(value),
                                _ => invalid_line = true,
                            },
                            "UseSkinSprites" => match value.parse() {
                                Ok(value) => story_fire_in_front = Some(value),
                                _ => invalid_line = true,
                            },
                            "AlwaysShowPlayField" => match value.parse() {
                                Ok(value) => always_show_play_field = Some(value),
                                _ => invalid_line = true,
                            },
                            "OverlayPosition" => match value {
                                "NoChange" => overlay_position = Some(OverlayPosition::NoChange),
                                "Below" => overlay_position = Some(OverlayPosition::Below),
                                "Above" => overlay_position = Some(OverlayPosition::Above),
                                _ => invalid_line = true,
                            },
                            "SkinPreference" => match value.is_empty() {
                                false => skin_preference = Some(value.into()),
                                _ => invalid_line = true,
                            },
                            "EpilepsyWarning" => match value.parse() {
                                Ok(value) => epilepsy_warning = Some(value),
                                _ => invalid_line = true,
                            },
                            "Countdown" => match value {
                                "0" => countdown = Some(Countdown::None),
                                "1" => countdown = Some(Countdown::Normal),
                                "2" => countdown = Some(Countdown::Half),
                                "3" => countdown = Some(Countdown::Double),
                                _ => invalid_line = true,
                            },
                            "SpecialStyle" => match value.parse() {
                                Ok(value) => special_style = Some(value),
                                _ => invalid_line = true,
                            },
                            "WidescreenStoryboard" => match value.parse() {
                                Ok(value) => widescreen_storyboard = Some(value),
                                _ => invalid_line = true,
                            },
                            "SamplesMatchPlaybackRate" => match value.parse() {
                                Ok(value) => samples_match_playback_rate = Some(value),
                                _ => invalid_line = true,
                            },
                            _ => key_matched = false,
                        }
                    }
                    if filedata && !key_matched {
                        key_matched = true;
                        match key {
                            "AudioFilename" => match value.is_empty() {
                                false => audio_filename = Some(value.into()),
                                _ => invalid_line = true,
                            },
                            "AudioLeadIn" => match value.parse() {
                                Ok(value) => audio_lead_in = Some(value),
                                _ => invalid_line = true,
                            },
                            "AudioHash" => match value.is_empty() {
                                false => audio_hash = Some(value.into()),
                                _ => invalid_line = true,
                            },
                            "PreviewTime" => match value.parse() {
                                Ok(value) => preview_time = Some(value),
                                _ => invalid_line = true,
                            },
                            "CountdownOffset" => match value.parse() {
                                Ok(value) => countdown_offset = Some(value),
                                _ => invalid_line = true,
                            },
                            _ => key_matched = false,
                        }
                    }
                } else {
                    invalid_line = true;
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[Editor]" => {
                if !editor {
                    continue;
                }
                let mut invalid_line = false;
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();
                    match key {
                        "[Bookmarks]" => {
                            for token in value.split(',') {
                                // Check delimiter TODO
                                match token.parse() {
                                    Ok(token) => bookmarks.push(token),
                                    _ => {
                                        invalid_line = true;
                                        break;
                                    }
                                }
                            }
                        }
                        "[DistanceSpacing]" => match from_str_ratio(value) {
                            Ok(value) => distance_spacing = Some(value),
                            _ => invalid_line = true,
                        },
                        "[BeatDivisor]" => match value.parse() {
                            Ok(value) => beat_divisor = Some(value),
                            _ => invalid_line = true,
                        },
                        "[GridSize]" => match value.parse() {
                            Ok(value) => grid_size = Some(value),
                            _ => invalid_line = true,
                        },
                        "[TimelineZoom]" => match from_str_ratio(value) {
                            Ok(value) => timeline_zoom = Some(value),
                            _ => invalid_line = true,
                        },
                    }
                } else {
                    invalid_line = true;
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[Metadata]" => {
                if !metadata {
                    continue;
                }
                let mut invalid_line = false;
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();
                    match key {
                        "Title" => title = Some(value.into()),
                        "TitleUnicode" => title_unicode = Some(value.into()),
                        "Artist" => artist = Some(value.into()),
                        "ArtistUnicode" => artist_unicode = Some(value.into()),
                        "Creator" => creator = Some(value.into()),
                        "Version" => version = Some(value.into()),
                        "Source" => source = Some(value.into()),
                        "Tags" => {
                            for token in value.split(',') {
                                // Check delimiter TODO
                                tags.push(token.into());
                            }
                        }
                        "BeatmapID" => match value.parse() {
                            Ok(value) => beatmap_id = Some(value),
                            _ => invalid_line = true,
                        },
                        "BeatmapSetID" => match value.parse() {
                            Ok(value) => beatmap_set_id = Some(value),
                            _ => invalid_line = true,
                        },
                    }
                } else {
                    invalid_line = true;
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "Difficulty" => {
                if !chart && !difficulty {
                    continue;
                }
                let invalid_line = false;
                if let Some((key, value)) = line.split_once(':') {
                    let key_matched = false;
                    if chart {
                        key_matched = true;
                        match key {
                            "StackLeniency" => match from_str_ratio(value) {
                                Ok(value) => stack_leniency = Some(value),
                                _ => invalid_line = true,
                            },
                            "SliderMultiplier" => match from_str_ratio(value) {
                                Ok(value) => slider_multiplier = Some(value),
                                _ => invalid_line = true,
                            },
                            "SliderTickRate" => match from_str_ratio(value) {
                                Ok(value) => slider_tick_rate = Some(value),
                                _ => invalid_line = true,
                            },
                            _ => key_matched = false,
                        }
                    }
                    if difficulty && !key_matched {
                        key_matched = true;
                        match key {
                            "CircleSize" => match from_str_one_decimal(value) {
                                Ok(value) => circle_size = Some(value),
                                _ => invalid_line = true,
                            },
                            "HPDrainRate" => match from_str_one_decimal(value) {
                                Ok(value) => hpdrain_rate = Some(value),
                                _ => invalid_line = true,
                            },
                            "OverallDifficulty" => match from_str_one_decimal(value) {
                                Ok(value) => overall_difficulty = Some(value),
                                _ => invalid_line = true,
                            },
                            "ApproachRate" => match from_str_one_decimal(value) {
                                Ok(value) => approach_rate = Some(value),
                                _ => invalid_line = true,
                            },
                            _ => key_matched = false,
                        }
                    }
                } else {
                    invalid_line = true;
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[Events]" => {
                if !customization {
                    continue;
                }
                let mut invalid_line = false;
                match line.chars().next() {
                    Some('0') => {} // Parse background TODO
                    Some('2') => {} // Parse break TODO
                    // Enumerate and parse all events TODO
                    _ => invalid_line = true,
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[TimingPoints]" => {
                if !chart {
                    continue;
                }
                let meter = 4;
                let sample_set = SampleSet::Default;
                let sample_index = 0;
                let volume = 100;
                let uninherited = true;
                let effects = Effects {
                    kiai: false,
                    ommit_barline: false,
                };
                let mut invalid_line = false;
                match (
                    line.split(',').nth(0),
                    line.split(',').nth(1),
                    line.split(',').nth(2),
                    line.split(',').nth(3),
                    line.split(',').nth(4),
                    line.split(',').nth(5),
                    line.split(',').nth(6),
                    line.split(',').nth(7),
                ) {
                    (Some(time), Some(beat_length), None, None, None, None, None, None) => {
                        match (time.trim().parse(), beat_length.trim().parse()) {
                            (Ok(time), Ok(beat_length)) => timing_points.push(TimingPoint {
                                time,
                                beat_length,
                                meter,
                                sample_set,
                                sample_index,
                                volume,
                                uninherited,
                                effects,
                            }),
                            _ => invalid_line = true,
                        }
                    }
                    (
                        Some(time),
                        Some(beat_length),
                        Some(meter),
                        Some(sample_set),
                        Some(sample_index),
                        Some(volume),
                        None,
                        None,
                    ) => {
                        match (
                            time.trim().parse(),
                            beat_length.trim().parse(),
                            meter.trim().parse(),
                            match sample_set.trim() {
                                "0" => Some(SampleSet::Default),
                                "1" => Some(SampleSet::Normal),
                                "2" => Some(SampleSet::Soft),
                                "3" => Some(SampleSet::Drum),
                                _ => None,
                            },
                            sample_index.trim().parse(),
                            volume.trim().parse(),
                        ) {
                            (
                                Ok(time),
                                Ok(beat_length),
                                Ok(meter),
                                Some(sample_set),
                                Ok(sample_index),
                                Ok(volume),
                            ) => timing_points.push(TimingPoint {
                                time,
                                beat_length,
                                meter,
                                sample_set,
                                sample_index,
                                volume,
                                uninherited,
                                effects,
                            }),
                            _ => invalid_line = true,
                        }
                    }
                    (
                        Some(time),
                        Some(beat_length),
                        Some(meter),
                        Some(sample_set),
                        Some(sample_index),
                        Some(volume),
                        Some(uninherited),
                        None,
                    ) => {
                        match (
                            time.trim().parse(),
                            beat_length.trim().parse(),
                            meter.trim().parse(),
                            match sample_set.trim() {
                                "0" => Some(SampleSet::Default),
                                "1" => Some(SampleSet::Normal),
                                "2" => Some(SampleSet::Soft),
                                "3" => Some(SampleSet::Drum),
                                _ => None,
                            },
                            sample_index.trim().parse(),
                            volume.trim().parse(),
                            uninherited.trim().parse(),
                        ) {
                            (
                                Ok(time),
                                Ok(beat_length),
                                Ok(meter),
                                Some(sample_set),
                                Ok(sample_index),
                                Ok(volume),
                                Ok(uninherited),
                            ) => timing_points.push(TimingPoint {
                                time,
                                beat_length,
                                meter,
                                sample_set,
                                sample_index,
                                volume,
                                uninherited,
                                effects,
                            }),
                            _ => invalid_line = true,
                        }
                    }
                    (
                        Some(time),
                        Some(beat_length),
                        Some(meter),
                        Some(sample_set),
                        Some(sample_index),
                        Some(volume),
                        Some(uninherited),
                        Some(effects),
                    ) => {
                        match (
                            time.trim().parse(),
                            beat_length.trim().parse(),
                            meter.trim().parse(),
                            match sample_set.trim() {
                                "0" => Some(SampleSet::Default),
                                "1" => Some(SampleSet::Normal),
                                "2" => Some(SampleSet::Soft),
                                "3" => Some(SampleSet::Drum),
                                _ => None,
                            },
                            sample_index.trim().parse(),
                            volume.trim().parse(),
                            uninherited.trim().parse(),
                            match effects.trim() {
                                "0" => Some(Effects {
                                    kiai: false,
                                    ommit_barline: false,
                                }),
                                "1" => Some(Effects {
                                    kiai: true,
                                    ommit_barline: false,
                                }),
                                "4" => Some(Effects {
                                    kiai: false,
                                    ommit_barline: true,
                                }),
                                "5" => Some(Effects {
                                    kiai: true,
                                    ommit_barline: true,
                                }),
                                _ => None,
                            },
                        ) {
                            (
                                Ok(time),
                                Ok(beat_length),
                                Ok(meter),
                                Some(sample_set),
                                Ok(sample_index),
                                Ok(volume),
                                Ok(uninherited),
                                Some(effects),
                            ) => timing_points.push(TimingPoint {
                                time,
                                beat_length,
                                meter,
                                sample_set,
                                sample_index,
                                volume,
                                uninherited,
                                effects,
                            }),
                            _ => invalid_line = true,
                        }
                    }
                    _ => invalid_line = true,
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[Colours]" => {
                if !customization {
                    continue;
                }
                let mut invalid_line = false;
                if let Some((_, value)) = line.split_once(':') {
                    if let (Some(red), Some(green), Some(blue)) = (
                        value.split(',').nth(0),
                        value.split(',').nth(1),
                        value.split(',').nth(2),
                    ) {
                        match (red.parse(), green.parse(), blue.parse()) {
                            (Ok(red), Ok(green), Ok(blue)) => {
                                colors.push(Color { red, green, blue })
                            }
                            _ => invalid_line = true,
                        }
                    } else {
                        invalid_line = true;
                    }
                } else {
                    invalid_line = true;
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
                }
            }
            "[HitObjects]" => {
                if !chart {
                    continue;
                }
                let mut invalid_line = false;
                let split = line.split(',');
                let x = match split.nth(0) {
                    Some(value) => match value.trim().parse::<i64>() {
                        Ok(value) => Some(value),
                        _ => None,
                    },
                    _ => None,
                };
                let y = match x {
                    Some(_) => match split.nth(1) {
                        Some(value) => match value.trim().parse::<i64>() {
                            Ok(value) => Some(value),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                };
                let time = match y {
                    Some(_) => match split.nth(2) {
                        Some(value) => match value.trim().parse::<i64>() {
                            Ok(value) => Some(value),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                };
                let flag_bits = match time {
                    Some(_) => match split.nth(3) {
                        Some(value) => match value.trim().parse::<u8>() {
                            Ok(value) => Some(value.view_bits::<Lsb0>()),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                };
                let object_type = match flag_bits {
                    Some(bits) => match {
                        bits[0] as usize * 2_usize.pow(0)
                            + bits[1] as usize * 2_usize.pow(1)
                            + bits[3] as usize * 2_usize.pow(3)
                            + bits[7] as usize * 2_usize.pow(7)
                    } {
                        1 => Some(ObjectType::Circle),
                        2 => Some(ObjectType::Slider),
                        8 => Some(ObjectType::Spinner),
                        128 => Some(ObjectType::ManiaHold),
                        _ => None,
                    },
                    _ => None,
                };
                let flags = match object_type {
                    Some(object_type) => Some(Type {
                        object_type,
                        new_combo: flag_bits?[2],
                        color_skip: flag_bits?[4..7].load::<u8>(),
                    }),
                    None => None,
                };
                let hit_sound = match flags {
                    Some(_) => match split.nth(4) {
                        Some(value) => match value.trim().parse::<u4>() {
                            Some(value) => {
                                let bits = value.view_bits::<Lsb0>();
                                Some(HitSound {
                                    normal: bits[0],
                                    whistle: bits[1],
                                    finish: bits[2],
                                    clap: bits[3],
                                })
                            }
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                };
                fn parse_hit_sample(sample: &str) -> Option<HitSample> {
                    let split = sample.split(':');
                    let normal_set = match split.nth(0) {
                        Some(value) => match value.trim() {
                            "0" => Some(SampleSet::Default),
                            "1" => Some(SampleSet::Normal),
                            "2" => Some(SampleSet::Soft),
                            "3" => Some(SampleSet::Drum),
                        },
                        _ => None,
                    };
                    let addition_set = match split.nth(1) {
                        Some(value) => match value.trim() {
                            "0" => Some(SampleSet::Default),
                            "1" => Some(SampleSet::Normal),
                            "2" => Some(SampleSet::Soft),
                            "3" => Some(SampleSet::Drum),
                        },
                        _ => None,
                    };
                    let index = match split.nth(2) {
                        Some(value) => match value.trim().parse() {
                            Ok(value) => Some(value),
                            _ => None,
                        },
                        _ => None,
                    };
                    let volume = match split.nth(3) {
                        Some(value) => match value.trim().parse() {
                            Ok(value) => Some(value),
                            _ => None,
                        },
                        _ => None,
                    };
                    let filename = match split.nth(4) {
                        Some(value) => Some(value.trim()),
                        _ => None,
                    };
                    HitSample {
                        normal_set: match normal_set {
                            Some(value) => value,
                            _ => SampleSet::Default,
                        },
                        addition_set: match addition_set {
                            Some(value) => value,
                            _ => SampleSet::Default,
                        },
                        index: match index {
                            Some(value) => value,
                            _ => 0,
                        },
                        v,
                    }
                }
                match hit_sound {
                    Some(_) => match object_type? {
                        ObjectType::Circle => {
                            // Parse hit circle TODO
                        }
                        ObjectType::Slider => {
                            // Parse slider TODO
                        }
                        ObjectType::Spinner => {
                            // Parse spinner TODO
                        }
                        ObjectType::ManiaHold => {
                            // Parse mania hold TODO
                        }
                    },
                    _ => invalid_line = true,
                }
                if invalid_line {
                    return Err(ParseError::InvalidLine {
                        line: line.into(),
                        section: section.into(),
                    });
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
            sample_set: sample_set.unwrap_or(SampleSet::Normal),
            letterbox_in_breaks: letterbox_in_breaks.unwrap_or(false),
            story_fire_in_front: story_fire_in_front.unwrap_or(true),
            use_skin_sprites: use_skin_sprites.unwrap_or(false),
            always_show_play_field: always_show_play_field.unwrap_or(false),
            overlay_position: overlay_position.unwrap_or(OverlayPosition::NoChange),
            skin_preference,
            epilepsy_warning: epilepsy_warning.unwrap_or(false),
            countdown: countdown.unwrap_or(Countdown::Normal),
            special_style: special_style.unwrap_or(false),
            widescreen_storyboard: widescreen_storyboard.unwrap_or(false),
            samples_match_playback_rate: samples_match_playback_rate.unwrap_or(false),
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
            audio_lead_in: audio_lead_in.unwrap_or(0),
            audio_hash,
            preview_time: preview_time.unwrap_or(-1),
            countdown_offset: countdown_offset.unwrap_or(0),
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

fn from_str_ratio(decimal: &str) -> Result<Ratio<i64>, Box<dyn std::error::Error>> {
    let mut numerator;
    let mut denominator;
    if let Some((lhs, rhs)) = decimal.split_once('.') {
        let (lhs, rhs) = match (lhs.is_empty(), rhs.is_empty()) {
            (false, false) => (lhs, rhs),
            (true, false) => ("0", rhs),
            (false, true) => (lhs, "0"),
            (true, true) => ("0", "0"),
        };
        let magnitude: u32 = rhs.len().try_into()?;
        numerator = (lhs.parse::<i64>()? * 10_i64.pow(magnitude)) + rhs.parse::<i64>()?;
        denominator = 10_i64.pow(magnitude);
    } else {
        let decimal = if decimal.is_empty() { "0" } else { decimal };
        numerator = decimal.parse::<i64>()?;
        denominator = 1;
    }

    Ok(Ratio::new(numerator, denominator))
}

fn from_str_one_decimal(decimal: &str) -> Result<u8, Box<dyn std::error::Error>> {
    if let Some((lhs, rhs)) = decimal.split_once('.') {
        match (lhs.is_empty(), rhs.is_empty()) {
            (false, false) => Ok(lhs.parse::<u8>()? * 10 + rhs.parse::<u8>()?),
            (true, false) => Ok(rhs.parse::<u8>()?),
            (false, true) => Ok(lhs.parse::<u8>()? * 10),
            (true, true) => Ok(0),
        }
    } else {
        Ok(decimal.parse::<u8>()? * 10)
    }
}
