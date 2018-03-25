extern crate resistor_codes;

use resistor_codes::float_value_to_ohms;

mod fixtures;

fn check(input: f32, expected: &str) {
    assert_eq!(float_value_to_ohms(input), expected);
}

#[test]
fn test_it_assigns_correct_code_for_range() {
    check(0.0, "0R");
    check(0.001, "1L");
    check(1.0, "1R");
    check(1_000.0, "1K");
    check(1_000_000.0, "1M");
    check(1_000_000_000.0, "1G");
    check(1_000_000_000_000.0, "1T");
}

#[test]
fn test_defaults_for_unsupported_values() {
    check(0.000_001, "0.000001R");
    check(1_000_000_000_000_000.0, "1000T");
}

#[test]
fn test_standard_e_series_resistor_values() {
    for &(float_value, expected_ohms_str) in fixtures::E_SERIES_SAMPLES {
        check(float_value, expected_ohms_str);
    }
}
