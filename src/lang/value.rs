use alloc::string::*;
use alloc::vec::*;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    FunctionRef(u64),
}
