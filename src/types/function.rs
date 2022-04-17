use crate::{Decimal, Function, FunctionName, Result, Value};

impl Value {
    pub fn apply(self) -> Result {
        if let Value::Function(func) = self {
            func.apply()
        } else {
            Ok(self)
        }
    }
}

impl Function {
    pub fn apply(self) -> Result {
        // match self.name.as_ref() {
        //     "add" => add(self.args),
        //     "subtract" => subtract(self.args),
        //     "product" => multiply(self.args),
        //     "divide" => divide(self.args),
        //     _ => Err(()),
        // }
        use FunctionName::*;

        #[allow(unreachable_patterns)]
        match self.name {
            Add => add(self.args),
            Subtract => subtract(self.args),
            Multiply => multiply(self.args),
            Divide => divide(self.args),
            _ => Err(()),
        }
    }
}

fn add(args: Vec<Value>) -> Result {
    let value = args
        .into_iter()
        .try_fold(Decimal::ZERO, |acc, val| Ok(acc + val.into_decimal()?))?;

    Ok(Value::Decimal(value))
}

fn subtract(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        return Ok(Value::Decimal(Decimal::ZERO));
    }

    let mut arguments = args.into_iter();
    let first_arg = arguments.next().unwrap().into_decimal()?;
    let value = arguments.try_fold(first_arg, |acc, val| Ok(acc - val.into_decimal()?))?;

    Ok(Value::Decimal(value.into()))
}

fn divide(args: Vec<Value>) -> Result {
    if args.len() == 0 {
        return Ok(Value::Decimal(Decimal::ONE));
    }

    let mut arguments = args.into_iter();
    let first_arg = arguments.next().unwrap().into_decimal()?;
    let value = arguments.try_fold(first_arg, |acc, val| Ok(acc / val.into_decimal()?))?;

    Ok(Value::Decimal(value.into()))
}

fn multiply(args: Vec<Value>) -> Result {
    let value = args
        .into_iter()
        .try_fold(Decimal::ONE, |acc, val| Ok(acc * val.into_decimal()?))?;

    Ok(Value::Decimal(value.into()))
}

#[test]
fn apply_add() {
    use crate::Decimal;

    let value = Value::Function(Function::new(
        FunctionName::Add,
        vec![
            Value::Function(Function::new(
                FunctionName::Add,
                vec![Value::Number(12.12), Value::Number(123.0)],
            )),
            Value::Number(5.0),
        ],
    ));

    let expected = Value::Decimal(Decimal::from_f64(140.12).unwrap());
    let result = value.apply().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn apply_multiply() {
    use crate::Decimal;

    let value = Value::Function(Function::new(
        FunctionName::Multiply,
        vec![
            Value::Function(Function::new(
                FunctionName::Multiply,
                vec![Value::Number(12.12), Value::Number(123.0)],
            )),
            Value::Number(5.0),
        ],
    ));

    let expected = Value::Decimal(Decimal::from_f64(7453.8).unwrap());
    let result = value.apply().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn apply_subtract() {
    use crate::Decimal;

    let value = Value::Function(Function::new(
        FunctionName::Subtract,
        vec![
            Value::Function(Function::new(
                FunctionName::Subtract,
                vec![Value::Number(8400.0), Value::Number(400.0)],
            )),
            Value::Number(50.0),
            Value::Number(20.0),
            Value::Number(10.0),
        ],
    ));

    let expected = Value::Decimal(Decimal::from_f64(7920.0).unwrap());
    let result = value.apply().unwrap();

    assert_eq!(expected, result);
}

#[test]
fn apply_divide() {
    use crate::Decimal;

    let value = Value::Function(Function::new(
        FunctionName::Divide,
        vec![
            Value::Function(Function::new(
                FunctionName::Divide,
                vec![Value::Number(8400.0), Value::Number(4.0)],
            )),
            Value::Number(50.0),
            Value::Number(2.0),
            Value::Number(0.1),
        ],
    ));

    let expected = Value::Decimal(Decimal::from_f64(210.0).unwrap());
    let result = value.apply().unwrap();

    assert_eq!(expected, result);
}
