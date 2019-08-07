#![feature(async_await)]

pub mod errors;
mod mail;
pub mod v3;

pub use mail::{Destination, Mail};
