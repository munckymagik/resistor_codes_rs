extern crate resistor_codes;
#[macro_use] extern crate hamcrest;

use hamcrest::prelude::*;
use resistor_codes::{ohms_value_to_float, ParseError};

#[test]
fn it_successfully_parses_integers() {
    assert_that!(ohms_value_to_float("1").unwrap(), is(equal_to(1.0)));
    assert_that!(ohms_value_to_float("1L").unwrap(), is(equal_to(0.001)));
    assert_that!(ohms_value_to_float("1R").unwrap(), is(equal_to(1.0)));
    assert_that!(ohms_value_to_float("1K").unwrap(), is(equal_to(1_000.0)));
    assert_that!(ohms_value_to_float("1M").unwrap(), is(equal_to(1_000_000.0)));
    assert_that!(ohms_value_to_float("1G").unwrap(), is(equal_to(1_000_000_000.0)));
    assert_that!(ohms_value_to_float("1T").unwrap(), is(equal_to(1_000_000_000_000.0)));
}

#[test]
fn it_successfully_parses_floats() {
    assert_that!(ohms_value_to_float("1.23456").unwrap(), is(equal_to(1.23456)));
    assert_that!(ohms_value_to_float("1.23456L").unwrap(), is(equal_to(0.00123456)));
    assert_that!(ohms_value_to_float("1.23456R").unwrap(), is(equal_to(1.23456)));
    assert_that!(ohms_value_to_float("1.23456K").unwrap(), is(equal_to(1_234.56)));
    assert_that!(ohms_value_to_float("1.23456M").unwrap(), is(equal_to(1_234_560.0)));
    assert_that!(ohms_value_to_float("1.23456G").unwrap(), is(equal_to(1_234_560_000.0)));
    assert_that!(ohms_value_to_float("1.23456T").unwrap(), is(equal_to(1_234_560_000_000.0)));
}

#[test]
fn it_supports_letter_and_digit_code_iec_60062() {
    assert_that!(ohms_value_to_float("1L23456").unwrap(), is(equal_to(0.00123456)));
    assert_that!(ohms_value_to_float("1R23456").unwrap(), is(equal_to(1.23456)));
    assert_that!(ohms_value_to_float("1K23456").unwrap(), is(equal_to(1_234.56)));
    assert_that!(ohms_value_to_float("1M23456").unwrap(), is(equal_to(1_234_560.0)));
    assert_that!(ohms_value_to_float("1G23456").unwrap(), is(equal_to(1_234_560_000.0)));
    assert_that!(ohms_value_to_float("1T23456").unwrap(), is(equal_to(1_234_560_000_000.0)));
}

#[test]
fn it_is_case_insensitive() {
    assert_that!(ohms_value_to_float("1k").unwrap(), is(equal_to(1000.0)));
    assert_that!(ohms_value_to_float("1.0m").unwrap(), is(equal_to(1_000_000.0)));
    assert_that!(ohms_value_to_float("1g2").unwrap(), is(equal_to(1_200_000_000.0)));
}

#[test]
fn it_handles_invalid_letter_codes() {
    assert_that!(ohms_value_to_float("1%").unwrap_err(),
                 is(equal_to(ParseError::UnsupportedLetterCode('%'))));

    assert_that!(ohms_value_to_float("1x2").unwrap_err(),
                 is(equal_to(ParseError::UnsupportedLetterCode('x'))));

    assert_that!(ohms_value_to_float("1,2").unwrap_err(),
                 is(equal_to(ParseError::UnsupportedLetterCode(','))));
}
