use crate::{Error, number::Number};

use self::ser::Serializer;

pub mod ser;

pub type Map<K, V> = std::collections::HashMap<K, V>;

pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Number(Number::from(value))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(Number::from(value))
    }
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: serde::Serialize,
{
    value.serialize(Serializer)
}
