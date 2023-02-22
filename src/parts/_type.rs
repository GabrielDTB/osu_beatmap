use super::errors::ParseError;
use anyhow::{bail, Context, Result};

/// The Type struct is a direct representation of the type field in the .osu file.
// TODO: Decide whether to only use this as an internal type. Reason: it sucks
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Type {
    object_type: ObjectType, // 0 circle, 1 slider, 3 spinner, 7 mania hold
    new_combo: bool,         // 2
    color_skip: u8,          // 4-6 -- Actually a 3 bit big-endian uint
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ObjectType {
    Circle,
    Slider,
    Spinner,
    ManiaHold,
}
