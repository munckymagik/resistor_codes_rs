#[derive(Debug, PartialEq)]
pub struct ResistorCode(pub f32);

impl ResistorCode {
    pub fn new(value_string: &str) -> Result<Self, i32> {
        Ok(ResistorCode(value_string.parse().expect("failed")))
    }
}
