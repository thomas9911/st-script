use crate::{Decimal, Error, Value};

impl Value {
    pub fn into_decimal(self) -> Result<Decimal, Error> {
        use Value::*;

        match self.apply()? {
            Number(number) => crate::Decimal::from_f64(number).ok_or(()),
            Decimal(decimal) => Ok(decimal),
            _ => Err(()),
        }
    }
}
