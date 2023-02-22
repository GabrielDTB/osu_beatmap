use anyhow::{Context, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Break {
    start_time: i64,
    end_time: i64,
}
impl std::str::FromStr for Break {
    type Err = anyhow::Error;
    /// Takes in a string that should be a comma-separated pair of numbers and returns a Break object.
    ///
    /// In a .osu file, break lines are located in the [Events] section and begin with "2," or "Break,".
    /// The break line should be shucked of its identifier then parsed.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair = s
            .split_once(',')
            .context(format!("Failed to split on comma in Break parsing of {s}"))?;
        let start_time = pair.0.parse::<i64>().context(format!(
            "Failed to parse first value as i64 in Break parsing of {s}"
        ))?;
        let end_time = pair.1.parse::<i64>().context(format!(
            "Failed to parse second value as i64 in Break parsing of {s}"
        ))?;
        Ok(Self {
            start_time,
            end_time,
        })
    }
}
