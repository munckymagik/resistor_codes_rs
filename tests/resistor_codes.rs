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
    assert_that!(ohms_value_to_float("1k").unwrap(), is(equal_to(1000.0)));
    assert_that!(ohms_value_to_float("2K").unwrap(), is(equal_to(2000.0)));
}
