extern crate resistor_codes;
#[macro_use] extern crate hamcrest;

use hamcrest::prelude::*;
use resistor_codes::{ResistorCode, ohms_value_to_float};

#[test]
fn it_works() {
    assert_that!(ResistorCode::new("1").unwrap(), is(equal_to(ResistorCode(1.0))));
    assert_that!(ResistorCode::new("2").unwrap(), is(equal_to(ResistorCode(2.0))));
}

#[test]
fn it_converts_from_ohms() {
    assert_that!(ohms_value_to_float("1").unwrap(), is(equal_to(1.0)));
    assert_that!(ohms_value_to_float("2").unwrap(), is(equal_to(2.0)));
}

#[test]
fn it_converts_from_kilohms() {
    assert_that!(ohms_value_to_float("2K").unwrap(), is(equal_to(2000.0)));
}

#[test]
fn it_converts_from_megohms() {
    assert_that!(ohms_value_to_float("2M").unwrap(), is(equal_to(2_000_000.0)));
}

#[test]
fn it_converts_from_gigaohms() {
    assert_that!(ohms_value_to_float("2G").unwrap(), is(equal_to(2_000_000_000.0)));
}

#[test]
fn it_is_case_insensitive() {
    assert_that!(ohms_value_to_float("1k").unwrap(), is(equal_to(1000.0)));
    assert_that!(ohms_value_to_float("1m").unwrap(), is(equal_to(1_000_000.0)));
    assert_that!(ohms_value_to_float("1g").unwrap(), is(equal_to(1_000_000_000.0)));
}

#[test]
fn it_successfully_parses_fractions() {
    assert_that!(ohms_value_to_float("1.23456").unwrap(), is(equal_to(1.23456)));
    assert_that!(ohms_value_to_float("1.23456K").unwrap(), is(equal_to(1_234.56)));
}

#[test]
fn it_support_letter_and_digit_code_iec_60062() {
    assert_that!(ohms_value_to_float("1R23456").unwrap(), is(equal_to(1.23456)));
    assert_that!(ohms_value_to_float("1K23456").unwrap(), is(equal_to(1_234.56)));
    assert_that!(ohms_value_to_float("1M23456").unwrap(), is(equal_to(1_234_560.0)));
    assert_that!(ohms_value_to_float("1G23456").unwrap(), is(equal_to(1_234_560_000.0)));
    assert_that!(ohms_value_to_float("1T23456").unwrap(), is(equal_to(1_234_560_000_000.0)));
}
