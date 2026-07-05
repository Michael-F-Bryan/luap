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
            Constant::Boolean(b) => Value::Boolean(b),
            Constant::Nil => Value::Nil,
        }
    }
}

impl From<NativeFuncValue> for Value {
    fn from(func: NativeFuncValue) -> Self {
        Value::NativeFunc(func)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s.into())
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.into())
    }
}

impl From<Arc<str>> for Value {
    fn from(s: Arc<str>) -> Self {
        Value::String(s)
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
