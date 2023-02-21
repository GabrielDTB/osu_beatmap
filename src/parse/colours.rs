use rayon::prelude::*;
use std::str::FromStr;

use crate::beatmap::customization::color::Color;

pub fn parse(data: Vec<&str>) -> Result<Vec<Returns>, String> {
    // Parse colors
    let mut lines = data
        .par_iter()
        .map(|s| match s.split(':').next_back() {
            Some(s) => {
                return match Color::from_str(s.trim()) {
                    Ok(c) => Ok(c),
                    Err(e) => Err(format!("invalid line \"{}\" in [Colours] parsing", s)),
                }
            }
            None => Err(format!("invalid line \"{}\" in [Colours] parsing", s)),
        })
        .collect::<Vec<Result<Color, String>>>();
    // Check for errors
    if lines.par_iter().any(|l| l.is_err()) {
        return Err(lines
            .par_iter()
            .filter(|l| l.is_err())
            .map(|l| l.unwrap_err())
            .collect::<Vec<String>>()
            .join(", "));
    } else {
        // Unwrap and return colors
        Ok(lines
            .par_iter()
            .map(|c| Returns::Color(c.unwrap()))
            .collect::<Vec<Returns>>())
    }
}

pub enum Returns {
    Color(Color),
}
