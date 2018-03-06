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
    let uppercase: String = value_string.to_uppercase();
    let (integral, letter_code, fractional) = parse(&uppercase);
    let multiplier = get_multiplier(letter_code);
    let value_string = join_parts(integral, fractional);

    value_string.parse::<f32>().map(|n| n * multiplier)
}

fn parse(raw_input: &str) -> (&str, u8, &str) {
    let (integral, tail) = eat_digits(raw_input.as_bytes());

    match tail.first() {
        None => (raw_input, b'.', ""),
        Some(&non_number) => {
            let letter_code = match non_number {
                 b'.' | b'L' | b'R' | b'K' | b'M' | b'G' | b'T' => non_number,
                 _ => panic!("Not handling this one yet")
            };

            let (fractional, _) = eat_digits(&tail[1..]);

            (
                str::from_utf8(integral).unwrap(),
                letter_code,
                str::from_utf8(fractional).unwrap()
            )
        }
    }
}

fn eat_digits(s: &[u8]) -> (&[u8], &[u8]) {
    let mut i = 0;
    while i < s.len() && b'0' <= s[i] && s[i] <= b'9' {
        i += 1;
    }
    (&s[..i], &s[i..])
}

fn get_multiplier(letter_code: u8) -> f32 {
    match letter_code {
        b'L'        => 1e-3,
        b'R' | b'.' => 1e0,
        b'K'        => 1e3,
        b'M'        => 1e6,
        b'G'        => 1e9,
        b'T'        => 1e12,
        _           => panic!("Unsupported letter code")
    }
}

fn join_parts(integral: &str, fractional: &str) -> String {
    integral.to_owned() + "." + fractional
}

#[cfg(test)]
mod test {
    use super::*;
    use std::panic;

    #[test]
    fn test_letter_notation_parser() {
        assert_eq!(parse("1"), ("1", b'.', ""));
        assert_eq!(parse("1.0"), ("1", b'.', "0"));

        assert_eq!(parse("1R"), ("1", b'R', ""));
        assert_eq!(parse("1R0"), ("1", b'R', "0"));

        assert_eq!(parse("1R2"), ("1", b'R', "2"));
        assert_eq!(parse("12R34"), ("12", b'R', "34"));

        assert_eq!(parse("1L1"), ("1", b'L', "1"));
        assert_eq!(parse("1R1"), ("1", b'R', "1"));
        assert_eq!(parse("1K1"), ("1", b'K', "1"));
        assert_eq!(parse("1M1"), ("1", b'M', "1"));
        assert_eq!(parse("1G1"), ("1", b'G', "1"));
        assert_eq!(parse("1T1"), ("1", b'T', "1"));

        let result = panic::catch_unwind(|| {
            parse("1$1");
        });
        assert_eq!(result.unwrap_err().downcast_ref::<&str>().unwrap(),
                   &"Not handling this one yet");
    }

    #[test]
    fn test_get_multiplier() {
        assert_eq!(get_multiplier(b'L'), 0.001);
        assert_eq!(get_multiplier(b'R'), 1.0);
        assert_eq!(get_multiplier(b'K'), 1_000.0);
        assert_eq!(get_multiplier(b'M'), 1_000_000.0);
        assert_eq!(get_multiplier(b'G'), 1_000_000_000.0);
        assert_eq!(get_multiplier(b'T'), 1_000_000_000_000.0);

        let result = panic::catch_unwind(|| {
            get_multiplier(b'%');
        });
        assert_eq!(result.unwrap_err().downcast_ref::<&str>().unwrap(),
                   &"Unsupported letter code");
    }

    #[test]
    fn test_join_integral_and_fractional() {
        assert_eq!(join_parts("1", "2"), "1.2");
        assert_eq!(join_parts("12", "34"), "12.34");
    }
}
