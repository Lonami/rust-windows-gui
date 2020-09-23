#![cfg(windows)]
pub mod messagebox;

pub use std::io::Error;

pub type Result<T> = std::io::Result<T>;
