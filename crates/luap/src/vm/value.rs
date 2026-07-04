use std::{fmt::Display, sync::Arc};

use crate::{bytecode::Constant, vm::NativeFuncValue};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Value {
    #[default]
    Nil,
    Boolean(bool),
    String(Arc<str>),
    Number(f64),
    NativeFunc(NativeFuncValue),
}

impl From<Constant> for Value {
    fn from(constant: Constant) -> Self {
        match constant {
            Constant::String(s) => Value::String(s),
            Constant::Number(n) => Value::Number(n.into_inner()),
            Constant::Integer(i) => Value::Number(i as f64),
            Constant::Boolean(b) => Value::Boolean(b),
            Constant::Nil => Value::Nil,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::NativeFunc(nf) => write!(f, "{}", nf.name()),
        }
    }
}
