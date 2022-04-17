use derive_more::{Add, Deref, DerefMut, Div, From, Mul, Sub};
use rust_decimal::prelude::FromPrimitive;
use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// wrapper around rust_decimal::Decimal that implement JsonSchema
#[derive(
    Debug, PartialEq, Clone, Serialize, Deserialize, Add, Sub, Mul, Div, Deref, DerefMut, From,
)]
#[mul(forward)]
#[div(forward)]
pub struct Decimal(rust_decimal::Decimal);

impl Decimal {
    pub const ZERO: Decimal = Decimal(rust_decimal::Decimal::ZERO);
    pub const ONE: Decimal = Decimal(rust_decimal::Decimal::ONE);

    pub fn new(value: i64, offset: u32) -> Decimal {
        rust_decimal::Decimal::new(value, offset).into()
    }

    pub fn from_f64(input: f64) -> Option<Decimal> {
        rust_decimal::Decimal::from_f64(input).map(Decimal)
    }
}

impl JsonSchema for Decimal {
    fn is_referenceable() -> bool {
        false
    }

    fn schema_name() -> String {
        "Decimal".to_owned()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            // decimal works for number and string
            instance_type: Some(vec![InstanceType::Number, InstanceType::String].into()),
            ..Default::default()
        }
        .into()
    }
}

// impl Deref for Decimal {
//     type Target = rust_decimal::Decimal;

//     fn deref(&self) -> &rust_decimal::Decimal {
//         &self.0
//     }
// }

// impl DerefMut for Decimal {
//     fn deref_mut(&mut self) -> &mut rust_decimal::Decimal {
//         &mut self.0
//     }
// }

// impl Add for Decimal {
//     type Output = Decimal;

//     fn add(self, other: Self) -> Decimal {
//         Decimal(self.0 + other.0)
//     }
// }

// impl Sub for Decimal {
//     type Output = Decimal;

//     fn sub(self, other: Self) -> Decimal {
//         Decimal(self.0 - other.0)
//     }
// }

// impl std::ops::Mul for Decimal {
//     type Output = Decimal;

//     fn mul(self, other: Self) -> Decimal {
//         Decimal(self.0 * other.0)
//     }
// }

// impl std::ops::Div for Decimal {
//     type Output = Decimal;

//     fn div(self, other: Self) -> Decimal {
//         Decimal(self.0 / other.0)
//     }
// }

// impl From<rust_decimal::Decimal> for Decimal {
//     fn from(decimal: rust_decimal::Decimal) -> Decimal {
//         Decimal(decimal)
//     }
// }

// impl From<Decimal> for rust_decimal::Decimal {
//     fn from(decimal: Decimal) -> rust_decimal::Decimal {
//         decimal.0
//     }
// }
