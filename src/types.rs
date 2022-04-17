use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::rc::Rc;

pub mod decimal;
pub mod function;
pub mod unresolved_value;
pub mod value;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum UnresolvedValue {
    Variable(Variable),
    Function(UnresolvedFunction),
    Static(Value),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
// #[serde(untagged)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "snake_case")]
pub enum Value {
    Function(Function),
    Object(Object),
    Array(Array),
    Text(Text),
    Number(Number),
    Decimal(Decimal),
    Boolean(Boolean),
    Datetime(DateTime),
    Date(Date),
    Time(Time),
    Null,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UnresolvedFunction {
    name: FunctionName,
    args: Vec<UnresolvedValue>,
}

impl UnresolvedFunction {
    pub fn new(name: FunctionName, args: Vec<UnresolvedValue>) -> UnresolvedFunction {
        UnresolvedFunction { name: name, args }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Function {
    name: FunctionName,
    args: Vec<Value>,
}

impl Function {
    pub fn new(name: FunctionName, args: Vec<Value>) -> Function {
        Function { name: name, args }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum FunctionName {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl std::str::FromStr for FunctionName {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

// wrappers around types so we can change them easily if we want to replace them for structs/enums
pub type Variable = String;
pub type Variables = HashMap<String, Rc<Value>>;

pub type Object = HashMap<String, Value>;
pub type Array = Vec<Value>;
pub type Text = String;
pub type Number = f64;
pub type Boolean = bool;
// pub type Decimal = rust_decimal::Decimal;
pub type Decimal = decimal::Decimal;

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Date = chrono::NaiveDate;
pub type Time = chrono::NaiveTime;

pub type Error = ();
pub type Result = std::result::Result<Value, Error>;
