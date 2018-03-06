use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub struct ResistorCode(pub f32);

impl ResistorCode {
    pub fn new(value_string: &str) -> Result<Self, i32> {
        Ok(ResistorCode(value_string.parse().expect("failed")))
    }
}

pub fn ohms_value_to_float(value_string: &str) -> Result<f32, ParseFloatError> {
    let lowercase: String = value_string.to_lowercase();

    let (value_string, multiplier) = if lowercase.ends_with("k") {
        (&value_string[..(value_string.len() - 1)], 1000.0)
    } else if lowercase.ends_with("m") {
        (&value_string[..(value_string.len() - 1)], 1_000_000.0)
    } else if lowercase.ends_with("g") {
        (&value_string[..(value_string.len() - 1)], 1_000_000_000.0)
    } else {
        (value_string, 1.0)
    };

    value_string.parse::<f32>().map(|n| n * multiplier)
}
