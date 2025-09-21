use alloc::string::*;
use alloc::vec::*;
use core::fmt::Display;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
    FunctionRef(u64, u64),
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
                let _ = write!(f, "[");

                let len = array.len();
                for i in 0..len {
                    let _ = write!(f, "{}", array[i]);
                    if i != len - 1 {
                        let _ = write!(f, ", ");
                    }
                }

                let _ = write!(f, "]");
                Ok(())
            }
            Value::FunctionRef(addr, args_count) => {
                return write!(f, "[function at {} with {} arguments]", addr, args_count);
            }
            Value::Null() => {
                return write!(f, "Null");
            }
        }
    }
}
