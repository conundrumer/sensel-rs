#[macro_use]
extern crate bitflags;

mod bindings;
mod result;
pub mod device;
pub mod frame;
pub mod contact;

pub use result::SenselError;
