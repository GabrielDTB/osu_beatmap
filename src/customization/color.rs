use rayon::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl std::str::FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse colors
        let mut colors = s
            .split(',')
            .collect::<Vec<&str>>()
            .into_par_iter()
            .map(|s| match s.parse::<u8>() {
                Ok(u) => Ok(u),
                Err(_) => Err(format!("at u8 parsing of {} in Color parsing", s)),
            })
            .collect::<Vec<Result<u8, String>>>();
        // Check for errors
        if colors.iter().any(|c| c.is_err()) {
            return Err(colors
                .into_iter()
                .filter(|c| c.is_err())
                .map(|c| c.unwrap_err())
                .collect::<Vec<String>>()
                .join(", "));
        }
        // Unwrap colors
        let (red, green, blue) = match colors.len() {
            3 => (
                colors.remove(0).unwrap(),
                colors.remove(0).unwrap(),
                colors.remove(0).unwrap(),
            ),
            _ => return Err(format!("expected 3 colors, got {}", colors.len())),
        };
        Ok(Self { red, green, blue })
    }
}

impl Color {
    pub fn tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}
