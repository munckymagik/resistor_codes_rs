extern crate resistor_codes;

use resistor_codes::{ohms_value_to_float, ParseError};

fn check(input: &str, expected: f32) {
    assert_eq!(ohms_value_to_float(input), Ok(expected));
}

fn check_err(input: &str, expected: ParseError) {
    assert_eq!(ohms_value_to_float(input), Err(expected));
}

#[test]
fn it_successfully_parses_integers() {
    check("1", 1.0);
    check("1L", 0.001);
    check("1R", 1.0);
    check("1K", 1_000.0);
    check("1M", 1_000_000.0);
    check("1G", 1_000_000_000.0);
    check("1T", 1_000_000_000_000.0);
}

#[test]
fn it_successfully_parses_floats() {
    check("1.23456", 1.23456);
    check("1.23456L", 0.00123456);
    check("1.23456R", 1.23456);
    check("1.23456K", 1_234.56);
    check("1.23456M", 1_234_560.0);
    check("1.23456G", 1_234_560_000.0);
    check("1.23456T", 1_234_560_000_000.0);
}

#[test]
fn it_supports_letter_and_digit_code_iec_60062() {
    check("1L23456", 0.00123456);
    check("1R23456", 1.23456);
    check("1K23456", 1_234.56);
    check("1M23456", 1_234_560.0);
    check("1G23456", 1_234_560_000.0);
    check("1T23456", 1_234_560_000_000.0);
}

#[test]
fn it_is_case_insensitive() {
    check("1k", 1000.0);
    check("1.0m", 1_000_000.0);
    check("1g2", 1_200_000_000.0);
}

#[test]
fn it_handles_invalid_letter_codes() {
    check_err("1%", ParseError::UnsupportedLetterCode('%'));
    check_err("1x2", ParseError::UnsupportedLetterCode('x'));
    check_err("1,2", ParseError::UnsupportedLetterCode(','));
}
