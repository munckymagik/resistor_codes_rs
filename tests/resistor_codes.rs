extern crate resistor_codes;
#[macro_use] extern crate hamcrest;

use hamcrest::prelude::*;
use resistor_codes::ResistorCode;

#[test]
fn it_works() {
    assert_that!(ResistorCode::new("1").unwrap(), is(equal_to(ResistorCode(1.0))));
    assert_that!(ResistorCode::new("2").unwrap(), is(equal_to(ResistorCode(2.0))));
}
