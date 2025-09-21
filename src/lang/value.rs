use alloc::string::*;
use alloc::vec::*;
use core::fmt::Display;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    FunctionRef(u64),
    Null(),
}

impl Display for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::Number(num) => {
                return write!(f, "{}", num);
            }
            Value::String(string) => {
                return write!(f, "{}", string);
            }
            Value::Boolean(boolean) => {
                return write!(f, "{}", boolean);
            }
            Value::Array(array) => {
                return write!(f, "[array]");
            }
            Value::FunctionRef(addr) => {
                return write!(f, "[reference to a function]");
            }
            Value::Null() => {
                return write!(f, "Null");
            }
        }
    }
}
