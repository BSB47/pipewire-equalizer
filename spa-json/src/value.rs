use core::fmt;

use crate::{Error, Map, number::Number};

use self::ser::Serializer;

pub mod de;
pub mod ser;

#[derive(Clone, PartialEq, Hash)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "\"{s}\""),
            Value::Array(a) => write!(f, "{a:?}"),
            Value::Object(o) => write!(f, "Object({:?})", o),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Number::from_f32(value).map_or(Value::Null, Value::Number)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Number::from_f64(value).map_or(Value::Null, Value::Number)
    }
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: serde::Serialize,
{
    value.serialize(Serializer)
}
