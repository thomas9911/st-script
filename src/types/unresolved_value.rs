use std::rc::Rc;

use crate::{Error, Function, UnresolvedFunction, UnresolvedValue, Value, Variable, Variables};

#[cfg(test)]
use crate::FunctionName;

impl UnresolvedValue {
    pub fn resolve(self, variables: &Variables) -> Result<Value, Error> {
        use UnresolvedValue::*;

        match self {
            Variable(var) => {
                if let Some(value) = variables.get(&var).cloned() {
                    Ok(Rc::try_unwrap(value).unwrap_or_else(|rc| (*rc).clone()))
                } else {
                    Err(())
                }
            }
            Function(function) => Ok(Value::Function(function.resolve(variables)?)),
            Static(value) => Ok(value),
        }
    }

    pub fn variables(&self) -> Vec<Variable> {
        use UnresolvedValue::*;

        let mut variables = Vec::new();

        match self {
            Variable(var) => {
                let idx = variables.binary_search(var).unwrap_or_else(|x| x);
                variables.insert(idx, var.clone());
            }
            Function(function) => variables.extend(function.variables()),
            Static(_) => (),
        }

        variables
    }
}

impl UnresolvedFunction {
    pub fn resolve(self, variables: &Variables) -> Result<Function, Error> {
        let args: Result<_, _> = self
            .args
            .into_iter()
            .map(|arg| arg.resolve(variables))
            .collect();

        Ok(Function::new(self.name, args?))
    }

    pub fn variables(&self) -> Vec<Variable> {
        let mut variables = Vec::new();

        for arg in self.args.iter() {
            variables.extend(arg.variables())
        }

        variables.sort();
        variables.dedup();

        variables
    }
}

#[test]
fn resolve_value() {
    let expected = Value::Function(Function::new(
        FunctionName::Add,
        vec![Value::Number(12.12), Value::Number(5.0)],
    ));
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
                "value": 5.0
              }
            }
          ]
        }
      }"#;

    let data = crate::from_json(text).unwrap();

    let mut variables = std::collections::HashMap::new();
    variables.insert("test".to_string(), Value::Number(12.12).into());

    let resolved = data.resolve(&variables).unwrap();

    assert_eq!(resolved, expected);
}

#[test]
fn resolve_nested_value() {
    let expected = Value::Function(Function::new(
        FunctionName::Add,
        vec![
            Value::Function(Function::new(
                FunctionName::Add,
                vec![Value::Number(12.12), Value::Number(123.0)],
            )),
            Value::Number(5.0),
        ],
    ));
    let text = r#"{
        "type": "function",
        "value": {
          "name": "add",
          "args": [
            {
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
                        "value": 123.0
                      }
                  }
                ]
              }
            },
            {
              "type": "static",
              "value": {
                "type": "number",
                "value": 5.0
              }
            }
          ]
        }
      }"#;

    let data = crate::from_json(text).unwrap();

    let mut variables = std::collections::HashMap::new();
    variables.insert("test".to_string(), Value::Number(12.12).into());

    let resolved = data.resolve(&variables).unwrap();

    assert_eq!(resolved, expected);
}

#[test]
fn variable_lists_variables() {
    let text = r#"{
        "type": "function",
        "value": {
          "name": "multiply",
          "args": [
            {
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
                        "value": 123.0
                      }
                  }
                ]
              }
            },
            {
                "type": "variable",
                "value": "test"
            },
            {
                "type": "static",
                "value": {
                  "type": "number",
                  "value": 5.0
                }
            },
            {
                "type": "variable",
                "value": "another"
            },
            {
                "type": "function",
                "value": {
                    "name": "add",
                    "args": [
                    {
                        "type": "variable",
                        "value": "one"
                    },
                    {
                        "type": "variable",
                        "value": "two"
                    }
                  ]
                }
              }
          ]
        }
      }"#;

    let data = crate::from_json(text).unwrap();
    let expected = vec![
        String::from("another"),
        String::from("one"),
        String::from("test"),
        String::from("two"),
    ];
    assert_eq!(expected, data.variables());
}

#[test]
fn serde_decimal() {
    use crate::Decimal;
    use crate::UnresolvedValue;

    let text_string = r#"
    {
        "type": "static",
        "value": {
            "type": "decimal",
            "value": "123.212332"
        }
    }
    "#;

    let text_number = r#"
    {
        "type": "static",
        "value": {
            "type": "decimal",
            "value": 123.212332
        }
    }
    "#;

    let expected = UnresolvedValue::Static(Value::Decimal(Decimal::new(123212332, 6)));

    let data = crate::from_json(text_string).unwrap();
    assert_eq!(expected, data);
    let data = crate::from_json(text_number).unwrap();
    assert_eq!(expected, data);
}

#[test]
fn serde_datetime() {
    use crate::UnresolvedValue;
    use chrono::{TimeZone, Utc};

    let text = r#"
    {
        "type": "static",
        "value": {
            "type": "datetime",
            "value": "2022-01-01T12:00:00Z"
        }
    }
    "#;

    let expected = UnresolvedValue::Static(Value::Datetime(Utc.ymd(2022, 1, 1).and_hms(12, 0, 0)));

    let data = crate::from_json(text).unwrap();
    assert_eq!(expected, data);
}

#[test]
fn serde_date() {
    use crate::UnresolvedValue;
    use chrono::NaiveDate;

    let text = r#"
    {
        "type": "static",
        "value": {
            "type": "date",
            "value": "2022-01-01"
        }
    }
    "#;

    let expected = UnresolvedValue::Static(Value::Date(NaiveDate::from_ymd(2022, 1, 1)));

    let data = crate::from_json(text).unwrap();
    assert_eq!(expected, data);
}

#[test]
fn serde_time() {
    use crate::UnresolvedValue;
    use chrono::NaiveTime;

    let text = r#"
    {
        "type": "static",
        "value": {
            "type": "time",
            "value": "23:59:59"
        }
    }
    "#;

    let expected = UnresolvedValue::Static(Value::Time(NaiveTime::from_hms(23, 59, 59)));

    let data = crate::from_json(text).unwrap();
    assert_eq!(expected, data);
}
