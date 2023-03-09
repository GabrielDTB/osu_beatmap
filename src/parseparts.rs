impl std::str::FromStr for Background {
    type Err = anyhow::Error;
    /// Takes in a string that should be a comma-separated triple of values and returns a Background object.
    ///
    /// In a .osu file, background lines are located in the [Events] section and begin with "0,0,".
    /// The background line should be shucked of its identifier then parsed.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We reverse here to handle filepaths that may contain commas.
        // By working from the back we can treat the remainder of the line as the filepath.
        let rest = s.chars().rev().collect::<String>();
        let split_one = rest.split_once(',').context(format!(
            "Failed to split rear element in Background parsing {s}"
        ))?;
        let yoffset = split_one
            .0
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i64>()
            .context(format!(
                "Failed to parse yoffset as i64 in Background parsing {s}"
            ))?;
        let split_two = split_one.1.split_once(',').context(format!(
            "Failed to split middle and first elements in Background parsing {s}"
        ))?;
        let xoffset = split_two
            .0
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i64>()
            .context(format!(
                "Failed to parse xoffset as i64 in Background parsing {s}"
            ))?;
        let filename = split_two.1.chars().rev().collect::<String>();
        Ok(Self {
            filename,
            xoffset,
            yoffset,
        })
    }
}

impl std::str::FromStr for Color {
    type Err = anyhow::Error;
    /// Takes in a string that should be a comma-separated triple of numbers and returns a Color object.
    ///
    /// In a .osu file, color lines are located in the [Colours] section and begin with "ComboX : ", where X is a number.
    /// The color line should be shucked of its identifier then parsed.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_split = s.split_once(',').context(format!(
            "Failed to split the first element in Color parsing {s}"
        ))?;
        Ok(Self { red, green, blue })
    }
}

impl std::str::FromStr for Countdown {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Countdown::None),
            "1" => Ok(Countdown::Normal),
            "2" => Ok(Countdown::Half),
            "3" => Ok(Countdown::Double),
            _ => Err("Invalid Countdown".into()),
        }
    }
}

impl std::str::FromStr for Curve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split('|');
        let _type = line
            .next()
            .expect("at _type assignment in Curve parsing")
            .parse::<CurveType>()
            .expect("at CurveType parsing in _type assignment in Curve parsing");
        let mut points = Vec::new();
        let mut count = 0;
        for pair in line {
            match (_type, count) {
                (CurveType::Perfect, 2) => {
                    return Result::Err(format!(
                        "Invalid Curve: Perfect curve {} has more than 2 points",
                        s
                    ))
                }
                _ => count += 1,
            }
            let mut pair = pair.split(':');
            points.push((
                match pair
                    .next()
                    .expect("at x assignment in points pushing in Curve parsing")
                    .parse::<i64>() {
                        Ok(x) => x,
                        Err(error) => return Result::Err(format!(
                                "at i64 parsing in x assignment in points pushing in Curve parsing: error: {} with input: {}",
                                error, s
                        )),
                },
                match pair
                    .next()
                    .expect("at y assignment in points pushing in Curve parsing")
                    .parse::<i64>() {
                        Ok(y) => y,
                        Err(error) => return Result::Err(format!(
                            "at i64 parsing in y assignment in points pushing in Curve parsing: error: {} with input: {}",
                            error, s
                        )),
                },
            ));
        }
        Ok(Self { _type, points })
    }
}

impl std::str::FromStr for CurveType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::Bezier),
            "C" => Ok(Self::Centripetal),
            "L" => Ok(Self::Linear),
            "P" => Ok(Self::Perfect),
            _ => return Result::Err(format!("Invalid CurveType: {}", s)),
        }
    }
}

impl std::str::FromStr for Effects {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kiai, ommit_barline) = match s {
            "0" => (false, false),
            "1" => (true, false),
            "4" => (false, true),
            "5" => (true, true),
            _ => return Result::Err(format!("Invalid effect")),
        };
        Ok(Self {
            kiai,
            ommit_barline,
        })
    }
}

impl std::str::FromStr for HalfHitSample {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !(s.matches(':').count() == 1) {
            return Result::Err(format!("Invalid HalfHitSample: {}", s));
        }
        let mut values = s.split(':');
        let normal_set = values
            .next()
            .expect("at normal_set assignment in HalfHitSample parsing")
            .parse::<SampleSet>()
            .expect("at SampleSet parsing in normal_set assignment in HalfHitSample parsing");
        let addition_set = values
            .next()
            .expect("at addition_set assignment in HalfHitSample parsing")
            .parse::<SampleSet>()
            .expect("at SampleSet parsing in addition_set assignment in HalfHitSample parsing");
        Ok(Self {
            normal_set,
            addition_set,
        })
    }
}

impl std::str::FromStr for HitSample {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut volume = 0;
        let mut filename = None;

        let mut values = match s.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before normal_set in HitSample parsing: {}",
                    s
                ))
            }
        };
        let normal_set = values
            .0
            .parse::<SampleSet>()
            .expect("at normal_set assignment with SampleSet parsing in HitSample parsing");
        values = match values.1.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before addition_set in HitSample parsing: {}",
                    s
                ))
            }
        };
        let addition_set = values
            .0
            .parse::<SampleSet>()
            .expect("at addition_set assignment with SampleSet parsing in HitSample parsing");
        values = match values.1.split_once(":") {
            Some(value) => value,
            None => {
                return Result::Err(format!(
                    "at values assignment before index in HitSample parsing: {}",
                    s
                ))
            }
        };
        let index = values
            .0
            .parse::<i64>()
            .expect("at index assignment with i64 parsing in HitSample parsing");
        if s.matches(':').count() > 3 {
            values = match values.1.split_once(":") {
                Some(value) => value,
                None => {
                    return Result::Err(format!(
                        "at values assignment before volume in HitSample parsing: {}",
                        s
                    ))
                }
            };
            volume = values
                .0
                .parse::<i64>()
                .expect("at volume assignment with i64 parsing in HitSample parsing");
        }
        if (s.matches(':').count() > 3) & !values.1.trim().is_empty() {
            filename = Some(values.1.to_string());
        }
        Ok(Self {
            normal_set,
            addition_set,
            index,
            volume,
            filename,
        })
    }
}

impl std::str::FromStr for SampleSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "Default" => Ok(SampleSet::Default),
            "1" | "Normal" => Ok(SampleSet::Normal),
            "2" | "Soft" => Ok(SampleSet::Soft),
            "3" | "Drum" => Ok(SampleSet::Drum),
            _ => return Result::Err(format!("Invalid str during SampleSet parsing: {}", s)),
        }
    }
}

impl std::str::FromStr for HitSound {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: i64 = s
            .parse()
            .expect("at num assignment and i64 parsing in HitSound parsing");
        let mut bits = [false; 8];
        if num > 2_i64.pow(4) - 1 {
            return Result::Err(format!("Invalid HitSound: {}", s));
        }
        if num > 2_i64.pow(3) - 1 {
            bits[3] = true;
            num -= 2_i64.pow(3);
        }
        if num > 2_i64.pow(2) - 1 {
            bits[2] = true;
            num -= 2_i64.pow(2);
        }
        if num > 2_i64.pow(1) - 1 {
            bits[1] = true;
            num -= 2_i64.pow(1);
        }
        if num > 2_i64.pow(0) - 1 {
            bits[0] = true;
            num -= 2_i64.pow(0);
        }
        if num > 0 {
            return Result::Err(format!("Logic error in HitSound creation"));
        }
        Ok(Self {
            normal: bits[0],
            whistle: bits[1],
            finish: bits[2],
            clap: bits[3],
        })
    }
}

impl std::str::FromStr for OverlayPosition {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoChange" => Ok(Self::NoChange),
            "Below" => Ok(Self::Below),
            "Above" => Ok(Self::Above),
            _ => Err("Invalid OverlayPosition".into()),
        }
    }
}

impl std::str::FromStr for SampleSet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "Default" => Ok(SampleSet::Default),
            "1" | "Normal" => Ok(SampleSet::Normal),
            "2" | "Soft" => Ok(SampleSet::Soft),
            "3" | "Drum" => Ok(SampleSet::Drum),
            _ => return Result::Err(format!("Invalid str during SampleSet parsing: {}", s)),
        }
    }
}

impl std::str::FromStr for Type {
    type Err = ParseError;
    /// Takes in a string that should be a single number and returns a Type object.
    ///
    /// In a .osu file, type is the 4th field in every hit-object.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = format!(
            "{:08b}",
            match s.parse::<u8>() {
                Ok(n) => n,
                Err(_) =>
                    return Err(ParseError::InvalidToken {
                        token: s.to_string(),
                        type_name: "u8".to_string(),
                    }),
            }
        )
        .chars()
        .map(|c| c == '1')
        .collect::<Vec<bool>>();
        let object_type = match (bits[0], bits[1], bits[3], bits[7]) {
            (true, false, false, false) => ObjectType::Circle,
            (false, true, false, false) => ObjectType::Slider,
            (false, false, true, false) => ObjectType::Spinner,
            (false, false, false, true) => ObjectType::ManiaHold,
            _ => bail!(
                "Bits did not match any one object type in Type parsing {:?}, {s}",
                (bits[0], bits[1], bits[3])
            ),
        };
        let new_combo = bits[2];
        let color_skip = (bits[4] as u8) << 2 | (bits[5] as u8) << 1 | (bits[6] as u8) << 0;
        Ok(Self {
            object_type,
            new_combo,
            color_skip,
        })
    }
}

impl std::str::FromStr for Spinner {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        let x = line
            .next()
            .expect("in x assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in x assignment in Spinner parsing");
        let y = line
            .next()
            .expect("in y assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in y assignment in Spinner parsing");
        let time = line
            .next()
            .expect("in time assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in time assignment in Spinner parsing");
        let flags = line
            .next()
            .expect("in flags assignment in Spinner parsing")
            .parse::<Type>()
            .expect("in Type parsing in flags assignment in Spinner parsing");
        let hit_sound = line
            .next()
            .expect("in hit_sound assignment in Spinner parsing")
            .parse::<HitSound>()
            .expect("in HitSound parsing in hit_sound assignment in Spinner parsing");
        let end_time = line
            .next()
            .expect("in end_time assignment in Spinner parsing")
            .parse::<i64>()
            .expect("in i64 parsing in end_time assignment in Spinner parsing");
        Ok(Spinner {
            x,
            y,
            time,
            flags,
            hit_sound,
            end_time,
            hit_sample,
        })
    }
}

impl std::str::FromStr for Slider {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let x = line
            .next()
            .expect("in x assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in x assignment in Slider parsing");
        let y = line
            .next()
            .expect("in y assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in y assignment in Slider parsing");
        let time = line
            .next()
            .expect("in time assignment in Slider parsing")
            .parse::<i64>()
            .expect("in i64 parsing in time assignment in Slider parsing");
        let flags = line
            .next()
            .expect("in flags assignment in Slider parsing")
            .parse::<Type>()
            .expect("in Type parsing in flags assignment in Slider parsing");
        let hit_sound = line
            .next()
            .expect("in hit_sound assignment in Slider parsing")
            .parse::<HitSound>()
            .expect("in HitSound parsing in hit_sound assignment in Slider parsing");
        let collected = line.collect::<Vec<&str>>();
        let commas = collected.len();
        let mut line = collected.into_iter();
        let curve = match commas {
            3 | 4 | 6 => line
                .next()
                .expect("in curve assignment in Slider parsing")
                .parse::<Curve>()
                .expect("in Curve parsing in curve assignment in Slider parsing"),
            _ => {
                return Result::Err(format!(
                    "Invalid slider: wrong remaining line size: {} in line: {} at curve assignment",
                    commas, s
                ))
            }
        };
        let slides = match commas {
            3 | 4 | 6 => line
                .next()
                .expect("in slides assignment in Slider parsing")
                .parse::<i64>()
                .expect("in i64 parsing in slides assignment in Slider parsing"),
            _ => {
                return Result::Err(format!(
                "Invalid slider: wrong remaining line size: {} in line: {} at slides assignment",
                commas, s
            ))
            }
        };
        let length = match commas {
            3 | 4 | 6 => line
                .next()
                .expect("in length assignment in Slider parsing")
                .parse::<f64>()
                .expect("in f64 parsing in length assignment in Slider parsing"),
            _ => {
                return Result::Err(format!(
                "Invalid slider: wrong remaining line size: {} in line: {} at length assignment",
                commas, s
            ))
            }
        };
        let edge_sounds = match commas {
            4 | 6 => {
                let mut sounds = Vec::new();
                for sound in line
                    .next()
                    .expect("in sound assignment in edge_sounds assignment in Slider parsing")
                    .split('|')
                {
                    sounds.push(
                        sound
                            .parse::<HitSound>()
                            .expect("in sounds pushing with HitSound parsing in edge_sounds assignment in Slider parsing"),
                    );
                }
                sounds
            }
            _ => vec![
                "0".parse::<HitSound>().expect(
                    "at edge_sounds assignment with HitSound parsing of \"0\" in Slider parsing",
                ),
                "2".parse::<HitSound>().expect(
                    "at edge_sounds assignment with HitSound parsing of \"2\" in Slider parsing",
                ),
            ],
        };
        let edge_sets = match commas {
            6 => {
                let mut sounds = Vec::new();
                for sound in line
                    .next()
                    .expect("in sound assignment in edge_sets assignment in Slider parsing")
                    .split('|')
                {
                    sounds.push(
                        sound
                            .parse::<HalfHitSample>()
                            .expect("in sounds pushing with HalfHitSample parsing in edge_sets assignment in Slider parsing"),
                    );
                }
                sounds
            }
            _ => vec![
                "0:0".parse::<HalfHitSample>().expect(
                    "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
                ),
                "0:0".parse::<HalfHitSample>().expect(
                    "at edge_sets assignment with HalfHitSample parsing of \"0:0\" in Slider parsing",
                ),
            ],
        };
        let hit_sample = match commas {
            6 => line
                .next()
                .expect("in hit_sample assignment in Slider parsing")
                .parse::<HitSample>()
                .expect("in HitSample parsing in hit_sample assignment in Slider parsing"),
            _ => HitSample {
                ..Default::default()
            },
        };
        Ok(Slider {
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
}

impl std::str::FromStr for Circle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(',');
        let hit_sample = HitSample {
            ..Default::default()
        };
        // TODO: Parse HitSamples
        let x = line
            .next()
            .expect("at x assignment in Circle parsing")
            .parse::<i64>()
            .expect("at i64 parsing in x assignment in Circle parsing");
        let y = line
            .next()
            .expect("at y assignment in Circle parsing")
            .parse::<i64>()
            .expect("at i64 parsing in y assignment in Circle parsing");
        let time = line
            .next()
            .expect("at time assignment in Circle parsing")
            .parse::<i64>()
            .expect("at i64 parsing in time assignment in Circle parsing");
        let flags = line
            .next()
            .expect("at flags assignment in Circle parsing")
            .parse::<Type>()
            .expect("at Type parsing in flags assignment in Circle parsing");
        let hit_sound = line
            .next()
            .expect("at hit_sound assignment in Circle parsing")
            .parse::<HitSound>()
            .expect("at HitSound parsing in hit_sound assignment in Circle parsing");
        Ok(Circle {
            x,
            y,
            time,
            flags,
            hit_sound,
            hit_sample,
        })
    }
}

impl std::str::FromStr for TimingPoint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.matches(',').count() {
            1 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 1 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 1 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 1 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 1 branch");
                Ok(Self {
                    time,
                    beat_length,
                    ..Default::default()
                })
            }
            5 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    ..Default::default()
                })
            }
            6 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    uninherited,
                    ..Default::default()
                })
            }
            7 => {
                let mut line = s.split(',');
                let time = line
                    .next()
                    .expect("at time assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of time in TimingPoint parsing, 7 branch");
                let beat_length = line
                    .next()
                    .expect("at beat_length assignment in TimingPoint parsing, 7 branch")
                    .parse::<f64>()
                    .expect("at f64 parsing of beat_length in TimingPoint parsing, 7 branch");
                let meter = line
                    .next()
                    .expect("at meter assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of meter in TimingPoint parsing, 7 branch");
                let sample_set = line
                    .next()
                    .expect("at sample_set assignment in TimingPoint parsing, 7 branch")
                    .parse::<SampleSet>()
                    .expect("at SampleSet parsing of sample_set in TimingPoint parsing, 7 branch");
                let sample_index = line
                    .next()
                    .expect("at sample_index assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of sample_index in TimingPoint parsing, 7 branch");
                //sample index
                let volume = line
                    .next()
                    .expect("at volume assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of volume in TimingPoint parsing, 7 branch");
                let uninherited = line
                    .next()
                    .expect("at uninherited assignment in TimingPoint parsing, 7 branch")
                    .parse::<i64>()
                    .expect("at i64 parsing of uninherited in TimingPoint parsing, 7 branch")
                    == 1;
                let effects = line
                    .next()
                    .expect("at effects assignment in TimingPoint parsing, 7 branch")
                    .parse::<Effects>()
                    .expect("at Effects parsing of effects in TimingPoint parsing, 7 branch");
                Ok(Self {
                    time,
                    beat_length,
                    meter,
                    sample_set,
                    sample_index,
                    volume,
                    uninherited,
                    effects,
                })
            }
            _ => return Result::Err(format!("Invalid timing point: {}", s)),
        }
    }
}
