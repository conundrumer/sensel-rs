#[macro_use]
extern crate bitflags;

mod bindings;
mod result;
pub mod device;
pub mod frame;
pub mod contact;

pub use result::SenselError;

pub const MAX_DEVICES: usize = bindings::SENSEL_MAX_DEVICES as usize;

pub mod scan_mode {
    pub use bindings::SenselScanMode::SCAN_MODE_SYNC;
    pub use bindings::SenselScanMode::SCAN_MODE_ASYNC;
}

pub mod scan_detail {
    pub use bindings::SenselScanDetail::SCAN_DETAIL_HIGH;
    pub use bindings::SenselScanDetail::SCAN_DETAIL_MEDIUM;
    pub use bindings::SenselScanDetail::SCAN_DETAIL_LOW;
}
