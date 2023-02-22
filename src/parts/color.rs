use anyhow::{Context, Result};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
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

impl Color {
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}
