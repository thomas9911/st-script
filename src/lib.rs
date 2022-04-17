pub mod types;
use schemars::schema_for;
pub use types::*;

pub fn from_json(input: &str) -> std::result::Result<UnresolvedValue, serde_json::Error> {
    serde_json::from_str::<UnresolvedValue>(input)
}

pub fn json_schema() -> String {
    let schema = schema_for!(UnresolvedValue);
    serde_json::to_string_pretty(&schema).unwrap()
}

#[test]
fn serde_roundtrip_from_json() {
    let text = r#"{
  "type": "function",
  "value": {
    "name": "add",
    "args": [
      {
        "type": "variable",
        "value": "test"
      },
      {
        "type": "static",
        "value": {
          "type": "number",
          "value": 1.2
        }
      }
    ]
  }
}"#;

    let data = from_json(text).unwrap();

    let result = serde_json::to_string_pretty(&data).unwrap();
    println!("{}", result);

    assert_eq!(text, result)
}

#[test]
fn serde_roundtrip_from_rust() {
    let data = UnresolvedValue::Function(UnresolvedFunction::new(
        FunctionName::Add,
        vec![
            UnresolvedValue::Variable("test".to_string()),
            UnresolvedValue::Static(Value::Number(1.2)),
        ],
    ));
    let text = serde_json::to_string_pretty(&data).unwrap();

    let result = from_json(&text).unwrap();

    assert_eq!(data, result)
}
