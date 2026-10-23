//! `-0`, `-0.0`, `-0e0` and `-0.0e-5` are negative zero, as in serde_json and
//! as `f32` already decoded them: the sign survives every decoding path.
use serde::Deserialize;
use sonic_rs::{JsonNumberTrait, JsonValueTrait, Value};

const NEGATIVE_ZEROS: [&str; 7] = [
    "-0", "-0.0", "-0e0", "-0E+3", "-0.0e-5", "-0.000", "-0.00E7",
];

#[derive(Debug, Deserialize)]
struct Record {
    value: f64,
    any: Value,
}

#[test]
fn negative_zero_keeps_its_sign() {
    for text in NEGATIVE_ZEROS {
        let expected = serde_json::from_str::<f64>(text).unwrap();
        assert!(
            expected.is_sign_negative(),
            "serde_json reference for {text}"
        );

        let f: f64 = sonic_rs::from_str(text).unwrap();
        assert_eq!(f.to_bits(), (-0.0f64).to_bits(), "f64 from_str {text}");
        let f: f64 = sonic_rs::from_slice(text.as_bytes()).unwrap();
        assert_eq!(f.to_bits(), (-0.0f64).to_bits(), "f64 from_slice {text}");
        let f: f32 = sonic_rs::from_str(text).unwrap();
        assert_eq!(f.to_bits(), (-0.0f32).to_bits(), "f32 {text}");

        let value: Value = sonic_rs::from_str(text).unwrap();
        let f = value.as_f64().unwrap();
        assert_eq!(f.to_bits(), (-0.0f64).to_bits(), "Value {text}");
        assert_eq!(value.to_string(), "-0.0", "Value re-encoding {text}");
        assert_eq!(
            value.as_number().and_then(|n| n.as_f64()).map(f64::to_bits),
            Some((-0.0f64).to_bits()),
            "Number {text}"
        );

        let record: Record =
            sonic_rs::from_str(&format!(r#"{{"value":{text},"any":[{text}]}}"#)).unwrap();
        assert_eq!(record.value.to_bits(), (-0.0f64).to_bits(), "field {text}");
        assert_eq!(record.any.to_string(), "[-0.0]", "nested Value {text}");
    }
}

#[test]
fn positive_zero_is_unchanged() {
    for text in ["0", "0.0", "0e0", "0.0e-5"] {
        let f: f64 = sonic_rs::from_str(text).unwrap();
        assert_eq!(f.to_bits(), 0.0f64.to_bits(), "{text}");
    }
    let value: Value = sonic_rs::from_str("0").unwrap();
    assert_eq!(value.as_u64(), Some(0));
    assert_eq!(value.to_string(), "0");
}
