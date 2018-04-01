use std::num::ParseFloatError;
use std::str::{self, FromStr, Utf8Error};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    StrToFloat(ParseFloatError),
    BytesToUtf8(Utf8Error),
    UnsupportedLetterCode(char),
}

pub fn ohms_value_to_float(value_string: &str) -> Result<f32, ParseError> {
    let (number, letter_code) = parse(&value_string)?;
    let multiplier = get_multiplier(letter_code)?;

    Ok(number * multiplier)
}

pub fn float_value_to_ohms(value: f32) -> String {
    let resistance_value = ResistanceValue::from_float(value);

    match resistance_value {
        ResistanceValue::Ohms(ohms) => ohms.to_string() + "R",
        ResistanceValue::Coded(ohms, code) => format!("{}{}", ohms, char::from(code)),
    }
}

#[derive(Debug, PartialEq)]
pub enum ResistanceValue {
    Ohms(f32),
    Coded(f32, u8),
}

impl ResistanceValue {
    fn from_float(value: f32) -> ResistanceValue {
        if value == 0.0 {
            return ResistanceValue::Ohms(value);
        }

        static DENOMINATORS_CODES: [(f32, u8); 6] = [
            (1e12, b'T'),
            (1e9, b'G'),
            (1e6, b'M'),
            (1e3, b'K'),
            (1e0, b'R'),
            (1e-3, b'L'),
        ];

        for &(denominator, code) in &DENOMINATORS_CODES {
            let result = value / denominator;
            if result >= 1.0 {
                return ResistanceValue::Coded(result, code);
            }
        }

        ResistanceValue::Ohms(value)
    }

    pub fn try_from_str(value_string: &str) -> Result<Self, ParseError> {
        let (number, letter_code) = parse(&value_string)?;
        Ok(ResistanceValue::Coded(number, letter_code))
    }
}

impl FromStr for ResistanceValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

impl From<f32> for ResistanceValue {
    fn from(f: f32) -> Self {
        Self::from_float(f)
    }
}

fn get_multiplier(letter_code: u8) -> Result<f32, ParseError> {
    match letter_code.to_ascii_uppercase() {
        b'L' => Ok(1e-3),
        b'R' => Ok(1e0),
        b'K' => Ok(1e3),
        b'M' => Ok(1e6),
        b'G' => Ok(1e9),
        b'T' => Ok(1e12),
        _ => Err(ParseError::UnsupportedLetterCode(letter_code.into())),
    }
}

fn parse(input: &str) -> Result<(f32, u8), ParseError> {
    fn read_digits_into_vec(buffer: &mut Vec<u8>, s: &[u8], i: &mut usize) {
        while *i < s.len() && b'0' <= s[*i] && s[*i] <= b'9' {
            buffer.push(s[*i]);
            *i += 1;
        }
    };

    let mut buffer: Vec<u8> = Vec::with_capacity(input.len());

    let s = input.as_bytes();
    let mut letter = b'R';
    let mut i = 0;

    read_digits_into_vec(&mut buffer, s, &mut i);

    if i < s.len() {
        if b'.' == s[i] {
            buffer.push(s[i]);
        } else {
            buffer.push(b'.');
            letter = s[i];
        }
        i += 1;

        read_digits_into_vec(&mut buffer, s, &mut i);

        if i < s.len() {
            letter = s[i];
        }
    }

    let number_str = str::from_utf8(&buffer).map_err(ParseError::BytesToUtf8)?;
    let float_value = number_str.parse::<f32>().map_err(ParseError::StrToFloat)?;

    Ok((float_value, letter))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        fn check(input: &str, expected: (f32, u8)) {
            assert_eq!(parse(input), Ok(expected));
        }

        check("1", (1.0, b'R'));
        check("1.0", (1.0, b'R'));
        check("1.", (1.0, b'R'));
        check("1.2", (1.2, b'R'));

        check("1.2R", (1.2, b'R'));
        check("1.R", (1.0, b'R'));
        check("1R", (1.0, b'R'));
        check("1R2", (1.2, b'R'));
        check("12R34", (12.34, b'R'));

        check("1L1", (1.1, b'L'));
        check("1R1", (1.1, b'R'));
        check("1K1", (1.1, b'K'));
        check("1M1", (1.1, b'M'));
        check("1G1", (1.1, b'G'));
        check("1T1", (1.1, b'T'));

        check("1.1L", (1.1, b'L'));
        check("1.1R", (1.1, b'R'));
        check("1.1K", (1.1, b'K'));
        check("1.1M", (1.1, b'M'));
        check("1.1G", (1.1, b'G'));
        check("1.1T", (1.1, b'T'));
    }

    #[test]
    fn test_get_multiplier() {
        fn check(input: u8, expected: f32) {
            assert_eq!(get_multiplier(input), Ok(expected));
        }

        check(b'L', 0.001);
        check(b'R', 1.0);
        check(b'K', 1_000.0);
        check(b'M', 1_000_000.0);
        check(b'G', 1_000_000_000.0);
        check(b'T', 1_000_000_000_000.0);

        check(b'l', 0.001);
        check(b'r', 1.0);
        check(b'k', 1_000.0);
        check(b'm', 1_000_000.0);
        check(b'g', 1_000_000_000.0);
        check(b't', 1_000_000_000_000.0);

        assert_eq!(
            get_multiplier(b'%'),
            Err(ParseError::UnsupportedLetterCode('%'))
        );
    }
}
