extern crate resistor_codes;

use resistor_codes::ResistanceValue;

#[test]
fn test_try_from_str() {
    assert_eq!(
        ResistanceValue::try_from_str("1.23456L"),
        Ok(ResistanceValue::Coded(1.23456, b'L'))
    );

    assert!(ResistanceValue::try_from_str("abc").is_err());
}

#[test]
fn test_from_str() {
    use std::str::FromStr;

    assert_eq!(
        ResistanceValue::from_str("1.23456L"),
        Ok(ResistanceValue::Coded(1.23456, b'L'))
    );

    assert_eq!(
        "1.23456L".parse(),
        Ok(ResistanceValue::Coded(1.23456, b'L'))
    );

    assert!(ResistanceValue::from_str("abc").is_err());
    assert!("abc".parse::<ResistanceValue>().is_err());
}

#[test]
fn test_from_float() {
    assert_eq!(ResistanceValue::from(1.2345), ResistanceValue::Coded(1.2345, b'R'));
    let rv: ResistanceValue = 1.2345.into();
    assert_eq!(rv, ResistanceValue::Coded(1.2345, b'R'));
}
