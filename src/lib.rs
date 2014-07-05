// I don't understand why both are needed, but rust-http has both.
#![crate_type = "dylib"]
#![crate_type = "rlib"]

pub mod common;
pub mod request;
pub mod response;
