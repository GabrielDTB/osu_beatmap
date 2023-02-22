use anyhow::{Context, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    filename: String,
    xoffset: i64,
    yoffset: i64,
}
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
