use std::num::ParseFloatError;
use std::str;

#[derive(Debug, PartialEq)]
pub struct ResistorCode(pub f32);

impl ResistorCode {
    pub fn new(value_string: &str) -> Result<Self, i32> {
        Ok(ResistorCode(value_string.parse().expect("failed")))
    }
}

pub fn ohms_value_to_float(value_string: &str) -> Result<f32, ParseFloatError> {
    let (number, letter_code) = parse(&value_string);
    let multiplier = get_multiplier(letter_code);

    Ok(number * multiplier)
}

fn get_multiplier(letter_code: u8) -> f32 {
    match letter_code.to_ascii_uppercase() {
        b'L'        => 1e-3,
        b'R' | b'.' => 1e0,
        b'K'        => 1e3,
        b'M'        => 1e6,
        b'G'        => 1e9,
        b'T'        => 1e12,
        _           => panic!("Unsupported letter code")
    }
}

fn parse(input: &str) -> (f32, u8) {
    fn read_digits_into_vec(buffer: &mut Vec<u8>, s: &[u8], i: &mut usize) {
        while *i < s.len() && b'0' <= s[*i] && s[*i] <= b'9' {
            buffer.push(s[*i]);
            *i += 1;
        }
    };

    let mut number: Vec<u8> = Vec::with_capacity(input.len());

    let s = input.as_bytes();
    let mut letter = b'R';
    let mut i = 0;

    read_digits_into_vec(&mut number, s, &mut i);

    if i < s.len() {
        if b'.' == s[i] {
            number.push(s[i]);
        } else {
            number.push(b'.');
            letter = s[i];
        }
        i += 1;

        read_digits_into_vec(&mut number, s, &mut i);

        if i < s.len() {
            letter = s[i];
        }
    }

    let value_string = str::from_utf8(&number).unwrap();
    (value_string.parse::<f32>().unwrap(), letter)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::panic;

    #[test]
    fn test_letter_notation_parser2() {
        assert_eq!(parse("1"), (1.0, b'R'));
        assert_eq!(parse("1.0"), (1.0, b'R'));
        assert_eq!(parse("1."), (1.0, b'R'));
        assert_eq!(parse("1.2"), (1.2, b'R'));

        assert_eq!(parse("1.2R"), (1.2, b'R'));
        assert_eq!(parse("1.R"), (1.0, b'R'));
        assert_eq!(parse("1R"), (1.0, b'R'));
        assert_eq!(parse("1R2"), (1.2, b'R'));
        assert_eq!(parse("12R34"), (12.34, b'R'));

        assert_eq!(parse("1L1"), (1.1, b'L'));
        assert_eq!(parse("1R1"), (1.1, b'R'));
        assert_eq!(parse("1K1"), (1.1, b'K'));
        assert_eq!(parse("1M1"), (1.1, b'M'));
        assert_eq!(parse("1G1"), (1.1, b'G'));
        assert_eq!(parse("1T1"), (1.1, b'T'));

        assert_eq!(parse("1.1L"), (1.1, b'L'));
        assert_eq!(parse("1.1R"), (1.1, b'R'));
        assert_eq!(parse("1.1K"), (1.1, b'K'));
        assert_eq!(parse("1.1M"), (1.1, b'M'));
        assert_eq!(parse("1.1G"), (1.1, b'G'));
        assert_eq!(parse("1.1T"), (1.1, b'T'));
    }

    #[test]
    fn test_get_multiplier() {
        assert_eq!(get_multiplier(b'L'), 0.001);
        assert_eq!(get_multiplier(b'R'), 1.0);
        assert_eq!(get_multiplier(b'K'), 1_000.0);
        assert_eq!(get_multiplier(b'M'), 1_000_000.0);
        assert_eq!(get_multiplier(b'G'), 1_000_000_000.0);
        assert_eq!(get_multiplier(b'T'), 1_000_000_000_000.0);

        assert_eq!(get_multiplier(b'l'), 0.001);
        assert_eq!(get_multiplier(b'r'), 1.0);
        assert_eq!(get_multiplier(b'k'), 1_000.0);
        assert_eq!(get_multiplier(b'm'), 1_000_000.0);
        assert_eq!(get_multiplier(b'g'), 1_000_000_000.0);
        assert_eq!(get_multiplier(b't'), 1_000_000_000_000.0);

        let result = panic::catch_unwind(|| {
            get_multiplier(b'%');
        });
        assert_eq!(result.unwrap_err().downcast_ref::<&str>().unwrap(),
                   &"Unsupported letter code");
    }
}
