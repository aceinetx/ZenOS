#![no_main]
#![no_std]
pub mod globals;
pub mod init;
pub mod io;
pub mod lang;
pub mod mem;
pub mod text;
pub use crate::lang::ast;
extern crate alloc;
