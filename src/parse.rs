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

    let mut sections = vec![
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
    let mut lines = s
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .peekable();
    let mut current_line = lines.next();
    while current_line != None {
        let line = current_line.unwrap();
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
        let mut parsed = false;
        match section {
            "[Preamble]" => 'preamble: {
                if !filedata {
                    break 'preamble;
                }
                // Check for version
                let version = match line.split_once('v') {
                    Some((_, rhs)) => rhs,
                    _ => break 'preamble,
                };
                // Parse version
                match version.parse() {
                    Ok(version) => file_format = Some(version),
                    _ => break 'preamble,
                };
                parsed = true;
            }
            "[General]" => 'general: {
                if !chart && !customization && !filedata {
                    break 'general;
                };
                let (key, value) = match line.split_once(':') {
                    Some((lhs, rhs)) => (lhs.trim(), rhs.trim()),
                    _ => break 'general,
                };
                let mut key_matched = false;
                if chart {
                    key_matched = true;
                    match key {
                        "Mode" => match value.parse() {
                            Ok(0) => mode = Some(Mode::Osu),
                            Ok(1) => mode = Some(Mode::Taiko),
                            Ok(2) => mode = Some(Mode::Catch),
                            Ok(3) => mode = Some(Mode::Mania),
                            _ => break 'general,
                        },
                        _ => key_matched = false,
                    };
                };
                if customization && !key_matched {
                    key_matched = true;
                    match key {
                        "SampleSet" => match value {
                            "Default" => sample_set = Some(SampleSet::Default),
                            "Normal" => sample_set = Some(SampleSet::Normal),
                            "Soft" => sample_set = Some(SampleSet::Soft),
                            "Drum" => sample_set = Some(SampleSet::Drum),
                            _ => break 'general,
                        },
                        "LetterboxInBreaks" => match value.parse() {
                            Ok(value) => letterbox_in_breaks = Some(value),
                            _ => break 'general,
                        },
                        "StoryFireInFront" => match value.parse() {
                            Ok(value) => story_fire_in_front = Some(value),
                            _ => break 'general,
                        },
                        "UseSkinSprites" => match value.parse() {
                            Ok(value) => use_skin_sprites = Some(value),
                            _ => break 'general,
                        },
                        "AlwaysShowPlayField" => match value.parse() {
                            Ok(value) => always_show_play_field = Some(value),
                            _ => break 'general,
                        },
                        "OverlayPosition" => match value {
                            "NoChange" => overlay_position = Some(OverlayPosition::NoChange),
                            "Below" => overlay_position = Some(OverlayPosition::Below),
                            "Above" => overlay_position = Some(OverlayPosition::Above),
                            _ => break 'general,
                        },
                        "SkinPreference" => match value.is_empty() {
                            false => skin_preference = Some(value.into()),
                            _ => break 'general,
                        },
                        "EpilepsyWarning" => match value.parse() {
                            Ok(value) => epilepsy_warning = Some(value),
                            _ => break 'general,
                        },
                        "Countdown" => match value.parse() {
                            Ok(0) => countdown = Some(Countdown::None),
                            Ok(1) => countdown = Some(Countdown::Normal),
                            Ok(2) => countdown = Some(Countdown::Half),
                            Ok(3) => countdown = Some(Countdown::Double),
                            _ => break 'general,
                        },
                        "SpecialStyle" => match value.parse() {
                            Ok(value) => special_style = Some(value),
                            _ => break 'general,
                        },
                        "WidescreenStoryboard" => match value.parse() {
                            Ok(value) => widescreen_storyboard = Some(value),
                            _ => break 'general,
                        },
                        "SamplesMatchPlaybackRate" => match value.parse() {
                            Ok(value) => samples_match_playback_rate = Some(value),
                            _ => break 'general,
                        },
                        _ => key_matched = false,
                    };
                };
                if filedata && !key_matched {
                    key_matched = true;
                    match key {
                        "AudioFilename" => match value.is_empty() {
                            false => audio_filename = Some(value.into()),
                            _ => break 'general,
                        },
                        "AudioLeadIn" => match value.parse() {
                            Ok(value) => audio_lead_in = Some(value),
                            _ => break 'general,
                        },
                        "AudioHash" => match value.is_empty() {
                            false => audio_hash = Some(value.into()),
                            _ => break 'general,
                        },
                        "PreviewTime" => match value.parse() {
                            Ok(value) => preview_time = Some(value),
                            _ => break 'general,
                        },
                        "CountdownOffset" => match value.parse() {
                            Ok(value) => countdown_offset = Some(value),
                            _ => break 'general,
                        },
                        _ => key_matched = false,
                    };
                };
                parsed = key_matched;
            }
            "[Editor]" => 'editor: {
                if !editor {
                    break 'editor;
                };
                let (key, value) = match line.split_once(':') {
                    Some((lhs, rhs)) => (lhs.trim(), rhs.trim()),
                    _ => break 'editor,
                };
                match key {
                    "[Bookmarks]" => {
                        let mut tokens: Vec<i64> = vec![];
                        for raw_token in value.split(',') {
                            match raw_token.parse() {
                                Ok(token) => tokens.push(token),
                                _ => break 'editor,
                            }
                        }
                        bookmarks.append(&mut tokens);
                    }
                    "[DistanceSpacing]" => match from_str_ratio(value) {
                        Ok(value) => distance_spacing = Some(value),
                        _ => break 'editor,
                    },
                    "[BeatDivisor]" => match value.parse() {
                        Ok(value) => beat_divisor = Some(value),
                        _ => break 'editor,
                    },
                    "[GridSize]" => match value.parse() {
                        Ok(value) => grid_size = Some(value),
                        _ => break 'editor,
                    },
                    "[TimelineZoom]" => match from_str_ratio(value) {
                        Ok(value) => timeline_zoom = Some(value),
                        _ => break 'editor,
                    },
                    _ => break 'editor,
                };
                parsed = true;
            }
            "[Metadata]" => 'metadata: {
                if !metadata {
                    break 'metadata;
                };
                let (key, value) = match line.split_once(':') {
                    Some((lhs, rhs)) => (lhs.trim(), rhs.trim()),
                    _ => break 'metadata,
                };
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
                        _ => break 'metadata,
                    },
                    "BeatmapSetID" => match value.parse() {
                        Ok(value) => beatmap_set_id = Some(value),
                        _ => break 'metadata,
                    },
                    _ => break 'metadata,
                };
                parsed = true;
            }
            "Difficulty" => 'difficulty: {
                if !chart && !difficulty {
                    break 'difficulty;
                };
                let (key, value) = match line.split_once(':') {
                    Some((lhs, rhs)) => (lhs.trim(), rhs.trim()),
                    _ => break 'difficulty,
                };
                let mut key_matched = false;
                if chart {
                    key_matched = true;
                    match key {
                        "StackLeniency" => match from_str_ratio(value) {
                            Ok(value) => stack_leniency = Some(value),
                            _ => break 'difficulty,
                        },
                        "SliderMultiplier" => match from_str_ratio(value) {
                            Ok(value) => slider_multiplier = Some(value),
                            _ => break 'difficulty,
                        },
                        "SliderTickRate" => match from_str_ratio(value) {
                            Ok(value) => slider_tick_rate = Some(value),
                            _ => break 'difficulty,
                        },
                        _ => key_matched = false,
                    }
                }
                if difficulty && !key_matched {
                    key_matched = true;
                    match key {
                        "CircleSize" => match from_str_one_decimal(value) {
                            Ok(value) => circle_size = Some(value),
                            _ => break 'difficulty,
                        },
                        "HPDrainRate" => match from_str_one_decimal(value) {
                            Ok(value) => hpdrain_rate = Some(value),
                            _ => break 'difficulty,
                        },
                        "OverallDifficulty" => match from_str_one_decimal(value) {
                            Ok(value) => overall_difficulty = Some(value),
                            _ => break 'difficulty,
                        },
                        "ApproachRate" => match from_str_one_decimal(value) {
                            Ok(value) => approach_rate = Some(value),
                            _ => break 'difficulty,
                        },
                        _ => key_matched = false,
                    }
                }
                parsed = key_matched;
            }
            "[Events]" => 'events: {
                /* // Basically broken TODO
                if !customization {
                    break 'events;
                };

                let mut split = line.splitn(2, ',');
                let event_type = match split.next() {
                    Some(value) => value,
                    _ => break 'events,
                };
                let start_time = match split.next() {
                    // This is broken for Storyboards TODO
                    Some(value) => match value.parse() {
                        Ok(time) => time,
                        _ => break 'events,
                    },
                    _ => break 'events,
                };
                let event_params = match split.next() {
                    Some(value) => value,
                    _ => break 'events,
                };
                match event_type {
                    "0" => {}           // Parse background TODO
                    "1" | "Video" => {} // Parse video TODO
                    "2" => {}           // Parse break TODO
                    // Enumerate and parse all events TODO
                    _ => break 'events,
                }
                parsed = true; */
            }
            "[TimingPoints]" => 'timing_points: {
                if !chart {
                    break 'timing_points;
                };
                let mut tokens = line.split(',').map(|t| t.trim());
                let time = match tokens.next() {
                    Some(token) => match token.trim().parse() {
                        Ok(time) => time,
                        _ => break 'timing_points,
                    },
                    _ => break 'timing_points,
                };
                let beat_length = match tokens.next() {
                    Some(token) => match token.trim().parse() {
                        Ok(length) => length,
                        _ => break 'timing_points,
                    },
                    _ => break 'timing_points,
                };
                let meter = match tokens.next() {
                    Some(token) => match token.trim().parse() {
                        Ok(meter) => meter,
                        _ => break 'timing_points,
                    },
                    _ => 4,
                };
                let sample_set = match tokens.next() {
                    Some(token) => match token.parse() {
                        Ok(0) => SampleSet::Default,
                        Ok(1) => SampleSet::Normal,
                        Ok(2) => SampleSet::Soft,
                        Ok(3) => SampleSet::Drum,
                        _ => break 'timing_points,
                    },
                    _ => SampleSet::Default,
                };
                let sample_index = match tokens.next() {
                    Some(token) => match token.trim().parse() {
                        Ok(index) => index,
                        _ => break 'timing_points,
                    },
                    _ => 0,
                };
                let volume = match tokens.next() {
                    Some(token) => match token.trim().parse() {
                        Ok(volume) => volume,
                        _ => break 'timing_points,
                    },
                    _ => 100,
                };
                let uninherited = match tokens.next() {
                    Some(token) => match token.trim().parse() {
                        Ok(uninherited) => uninherited,
                        _ => break 'timing_points,
                    },
                    _ => true,
                };
                let effects = match tokens.next() {
                    Some(token) => match token.trim().parse::<u8>() {
                        Ok(number) => {
                            let bits = number.view_bits::<Lsb0>();
                            Effects {
                                kiai: bits[0],
                                ommit_barline: bits[1],
                            }
                        }
                        _ => break 'timing_points,
                    },
                    _ => Effects {
                        kiai: false,
                        ommit_barline: false,
                    },
                };
                timing_points.push(TimingPoint {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    uninherited,
                    effects,
                });
                parsed = true;
            }
            "[Colours]" => 'colours: {
                // More research needs to be done on this field TODO
                if !customization {
                    break 'colours;
                };
                let mut rgb = match line.split_once(':') {
                    Some((_, rhs)) => rhs.split(','),
                    _ => break 'colours,
                };
                let (red, green, blue) = match (rgb.next(), rgb.next(), rgb.next()) {
                    (Some(r), Some(g), Some(b)) => match (r.parse(), g.parse(), b.parse()) {
                        (Ok(r), Ok(g), Ok(b)) => (r, g, b),
                        _ => break 'colours,
                    },
                    _ => break 'colours,
                };
                colors.push(Color { red, green, blue });
                parsed = true;
            }
            "[HitObjects]" => 'hit_objects: {
                if !chart {
                    break 'hit_objects;
                };
                let mut tokens = line.split(',').map(|t| t.trim());
                let (x, y, time, flags) =
                    match (tokens.next(), tokens.next(), tokens.next(), tokens.next()) {
                        (Some(x), Some(y), Some(time), Some(flags)) => {
                            match (x.parse(), y.parse(), time.parse(), flags.parse::<u8>()) {
                                (Ok(x), Ok(y), Ok(time), Ok(flags)) => (x, y, time, flags),
                                _ => break 'hit_objects,
                            }
                        }
                        _ => break 'hit_objects,
                    };
                let flag_bits = flags.view_bits::<Lsb0>();
                let object_type = match {
                    flag_bits[0] as usize * 2_usize.pow(0)
                        + flag_bits[1] as usize * 2_usize.pow(1)
                        + flag_bits[3] as usize * 2_usize.pow(3)
                        + flag_bits[7] as usize * 2_usize.pow(7)
                } {
                    1 => ObjectType::Circle,      // 2^0
                    2 => ObjectType::Slider,      // 2^1
                    8 => ObjectType::Spinner,     // 2^3
                    128 => ObjectType::ManiaHold, // 2^7
                    _ => break 'hit_objects,
                };
                let flags = Type {
                    object_type,
                    new_combo: flag_bits[2],
                    color_skip: flag_bits[4..7].load::<u8>(),
                };
                let hit_sound = match tokens.next() {
                    Some(value) => match value.parse::<u8>() {
                        Ok(value) => {
                            let bits = value.view_bits::<Lsb0>();
                            HitSound {
                                normal: bits[0],
                                whistle: bits[1],
                                finish: bits[2],
                                clap: bits[3],
                            }
                        }
                        _ => break 'hit_objects,
                    },
                    _ => break 'hit_objects,
                };
                fn parse_hit_sound(sound: u8) -> HitSound {
                    let bits = sound.view_bits::<Lsb0>();
                    HitSound {
                        normal: bits[0],
                        whistle: bits[1],
                        finish: bits[2],
                        clap: bits[3],
                    }
                }
                fn parse_sample_set(sample: &str) -> Option<SampleSet> {
                    match sample.parse() {
                        Ok(0) => Some(SampleSet::Default),
                        Ok(1) => Some(SampleSet::Normal),
                        Ok(2) => Some(SampleSet::Soft),
                        Ok(3) => Some(SampleSet::Drum),
                        _ => None,
                    }
                }
                fn parse_hit_sample(sample: &str) -> Option<HitSample> {
                    let mut tokens = sample.split(':').map(|t| t.trim());
                    let normal_set = match tokens.next() {
                        Some(value) => match parse_sample_set(value) {
                            Some(sample) => sample,
                            _ => return None,
                        },
                        _ => SampleSet::Default,
                    };
                    let addition_set = match tokens.next() {
                        Some(value) => match parse_sample_set(value) {
                            Some(sample) => sample,
                            _ => return None,
                        },
                        _ => SampleSet::Default,
                    };
                    let index = match tokens.next() {
                        Some(value) => match value.parse() {
                            Ok(value) => value,
                            _ => return None,
                        },
                        _ => 0,
                    };
                    let volume = match tokens.next() {
                        Some(value) => match value.parse() {
                            Ok(value) => value,
                            _ => return None,
                        },
                        _ => 100,
                    };
                    let filename = match tokens.next() {
                        Some(value) => match value.is_empty() {
                            false => Some(value.into()),
                            _ => return None,
                        },
                        _ => None,
                    };
                    Some(HitSample {
                        normal_set,
                        addition_set,
                        index,
                        volume,
                        filename,
                    })
                }
                let object: HitObject = match object_type {
                    ObjectType::Circle => {
                        // Hit objects have no objectParams
                        let hit_sample = match parse_hit_sample(tokens.next().unwrap_or("")) {
                            Some(sample) => sample,
                            _ => break 'hit_objects,
                        };
                        HitObject::Circle(Circle {
                            x,
                            y,
                            time,
                            flags,
                            hit_sound,
                            hit_sample,
                        })
                    }
                    ObjectType::Slider => {
                        let mut curve_split = match tokens.next() {
                            Some(token) => token.split('|').map(|s| s.trim()),
                            _ => break 'hit_objects,
                        };
                        let curve_type = match curve_split.next() {
                            Some("B") => CurveType::Bezier,
                            Some("C") => CurveType::Centripetal,
                            Some("L") => CurveType::Linear,
                            Some("P") => CurveType::Perfect,
                            _ => break 'hit_objects,
                        };
                        let mut curve_points: Vec<(i64, i64)> = vec![];
                        let mut point = curve_split.next();
                        while point != None {
                            match point.unwrap().split_once(':') {
                                Some((x, y)) => match (x.trim().parse(), y.trim().parse()) {
                                    (Ok(x), Ok(y)) => curve_points.push((x, y)),
                                    _ => break 'hit_objects,
                                },
                                _ => break 'hit_objects,
                            }
                            point = curve_split.next();
                        }
                        let curve = Curve {
                            _type: curve_type,
                            points: curve_points,
                        };
                        let (slides, length) = match (tokens.next(), tokens.next()) {
                            (Some(slides), Some(length)) => {
                                match (slides.trim().parse(), length.trim().parse()) {
                                    (Ok(slides), Ok(length)) => (slides, length),
                                    _ => break 'hit_objects,
                                }
                            }
                            _ => break 'hit_objects,
                        };
                        let mut sounds_split = match tokens.next() {
                            Some(token) => token.split('|').map(|t| t.trim()),
                            _ => break 'hit_objects,
                        };
                        let mut edge_sounds: Vec<HitSound> = vec![];
                        let mut sound = sounds_split.next();
                        while sound != None {
                            match sound.unwrap().parse::<u8>() {
                                Ok(number) => edge_sounds.push(parse_hit_sound(number)),
                                _ => break 'hit_objects,
                            }
                            sound = sounds_split.next();
                        }
                        let mut sets_split = match tokens.next() {
                            Some(token) => token.split('|').map(|t| t.trim()),
                            _ => break 'hit_objects,
                        };
                        let mut edge_sets: Vec<(SampleSet, SampleSet)> = vec![];
                        let set = sets_split.next();
                        while set != None {
                            match set.unwrap().split_once(':') {
                                Some((normal, addition)) => {
                                    match (parse_sample_set(normal), parse_sample_set(addition)) {
                                        (Some(normal), Some(addition)) => {
                                            edge_sets.push((normal, addition))
                                        }
                                        _ => break 'hit_objects,
                                    }
                                }
                                _ => break 'hit_objects,
                            };
                        }
                        let hit_sample = match parse_hit_sample(tokens.next().unwrap_or("")) {
                            Some(sample) => sample,
                            _ => break 'hit_objects,
                        };
                        HitObject::Slider(Slider {
                            x,
                            y,
                            time,
                            flags,
                            hit_sound,
                            curve,
                            slides,
                            length,
                            edge_sounds,
                            edge_sets,
                            hit_sample,
                        })
                    }
                    ObjectType::Spinner => {
                        let (end_time, hit_sample) =
                            match (tokens.next(), tokens.next().unwrap_or("")) {
                                (Some(time), sample) => {
                                    match (time.parse(), parse_hit_sample(sample)) {
                                        (Ok(time), Some(sample)) => (time, sample),
                                        _ => break 'hit_objects,
                                    }
                                }
                                _ => break 'hit_objects,
                            };
                        HitObject::Spinner(Spinner {
                            x,
                            y,
                            time,
                            flags,
                            hit_sound,
                            end_time,
                            hit_sample,
                        })
                    }
                    ObjectType::ManiaHold => {
                        let (end_time, hit_sample) = match tokens.next() {
                            Some(token) => match token.split_once(':') {
                                Some((time, sample)) => {
                                    match (time.parse(), parse_hit_sample(sample)) {
                                        (Ok(time), Some(sample)) => (time, sample),
                                        _ => break 'hit_objects,
                                    }
                                }
                                _ => break 'hit_objects,
                            },
                            _ => break 'hit_objects,
                        };
                        HitObject::ManiaHold(ManiaHold {
                            x,
                            y,
                            time,
                            flags,
                            hit_sound,
                            end_time,
                            hit_sample,
                        })
                    }
                };
                hit_objects.push(object);
                parsed = true;
            }
            _ => {}
        }
        if !parsed {
            return Err(ParseError::InvalidLine {
                line: line.into(),
                section: section.into(),
            });
        };
        current_line = lines.next();
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
    let numerator;
    let denominator;
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
