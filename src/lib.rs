use std::num::ParseFloatError;
use std::str::{self, Utf8Error};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    StrToFloat(ParseFloatError),
    BytesToUtf8(Utf8Error),
    UnsupportedLetterCode(char)
}

pub fn ohms_value_to_float(value_string: &str) -> Result<f32, ParseError> {
    let (number, letter_code) = parse(&value_string)?;
    let multiplier = get_multiplier(letter_code)?;

    Ok(number * multiplier)
}

fn get_multiplier(letter_code: u8) -> Result<f32, ParseError> {
    match letter_code.to_ascii_uppercase() {
        b'L'        => Ok(1e-3),
        b'R' | b'.' => Ok(1e0),
        b'K'        => Ok(1e3),
        b'M'        => Ok(1e6),
        b'G'        => Ok(1e9),
        b'T'        => Ok(1e12),
        _           => Err(ParseError::UnsupportedLetterCode(letter_code.into()))
    }
}

fn parse(input: &str) -> Result<(f32, u8), ParseError>  {
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
        assert_eq!(parse("1"), Ok((1.0, b'R')));
        assert_eq!(parse("1.0"), Ok((1.0, b'R')));
        assert_eq!(parse("1."), Ok((1.0, b'R')));
        assert_eq!(parse("1.2"), Ok((1.2, b'R')));

        assert_eq!(parse("1.2R"), Ok((1.2, b'R')));
        assert_eq!(parse("1.R"), Ok((1.0, b'R')));
        assert_eq!(parse("1R"), Ok((1.0, b'R')));
        assert_eq!(parse("1R2"), Ok((1.2, b'R')));
        assert_eq!(parse("12R34"), Ok((12.34, b'R')));

        assert_eq!(parse("1L1"), Ok((1.1, b'L')));
        assert_eq!(parse("1R1"), Ok((1.1, b'R')));
        assert_eq!(parse("1K1"), Ok((1.1, b'K')));
        assert_eq!(parse("1M1"), Ok((1.1, b'M')));
        assert_eq!(parse("1G1"), Ok((1.1, b'G')));
        assert_eq!(parse("1T1"), Ok((1.1, b'T')));

        assert_eq!(parse("1.1L"), Ok((1.1, b'L')));
        assert_eq!(parse("1.1R"), Ok((1.1, b'R')));
        assert_eq!(parse("1.1K"), Ok((1.1, b'K')));
        assert_eq!(parse("1.1M"), Ok((1.1, b'M')));
        assert_eq!(parse("1.1G"), Ok((1.1, b'G')));
        assert_eq!(parse("1.1T"), Ok((1.1, b'T')));
    }

    #[test]
    fn test_get_multiplier() {
        assert_eq!(get_multiplier(b'L'), Ok(0.001));
        assert_eq!(get_multiplier(b'R'), Ok(1.0));
        assert_eq!(get_multiplier(b'K'), Ok(1_000.0));
        assert_eq!(get_multiplier(b'M'), Ok(1_000_000.0));
        assert_eq!(get_multiplier(b'G'), Ok(1_000_000_000.0));
        assert_eq!(get_multiplier(b'T'), Ok(1_000_000_000_000.0));

        assert_eq!(get_multiplier(b'l'), Ok(0.001));
        assert_eq!(get_multiplier(b'r'), Ok(1.0));
        assert_eq!(get_multiplier(b'k'), Ok(1_000.0));
        assert_eq!(get_multiplier(b'm'), Ok(1_000_000.0));
        assert_eq!(get_multiplier(b'g'), Ok(1_000_000_000.0));
        assert_eq!(get_multiplier(b't'), Ok(1_000_000_000_000.0));

        assert_eq!(get_multiplier(b'%'), Err(ParseError::UnsupportedLetterCode('%')));
    }
}
