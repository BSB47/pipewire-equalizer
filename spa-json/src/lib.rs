pub mod error;

mod serde;
pub use self::serde::ser::{
    to_string, to_string_pretty, to_vec, to_vec_pretty, to_writer, to_writer_pretty,
};

pub type Map<K, V> = std::collections::HashMap<K, V>;

pub enum Value {
    Int(i32),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

use tri;
