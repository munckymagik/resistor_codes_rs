extern crate resistor_codes;

use resistor_codes::float_value_to_ohms;

mod fixtures;

#[test]
fn test_it_assigns_correct_code_for_range() {
    assert_eq!(float_value_to_ohms(0.0), "0R");
    assert_eq!(float_value_to_ohms(0.001), "1L");
    assert_eq!(float_value_to_ohms(1.0), "1R");
    assert_eq!(float_value_to_ohms(1_000.0), "1K");
    assert_eq!(float_value_to_ohms(1_000_000.0), "1M");
    assert_eq!(float_value_to_ohms(1_000_000_000.0), "1G");
    assert_eq!(float_value_to_ohms(1_000_000_000_000.0), "1T");
}

#[test]
fn test_defaults_for_unsupported_values() {
    assert_eq!(float_value_to_ohms(0.000_001), "0.000001R");
    assert_eq!(float_value_to_ohms(1_000_000_000_000_000.0), "1000T");
}

#[test]
fn test_standard_e_series_resistor_values() {
    for &(value, expected) in fixtures::E_SERIES_SAMPLES {
        assert_eq!(float_value_to_ohms(value), expected);
    }
}
