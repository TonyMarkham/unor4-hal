//! Finding the bench's serial ports by USB VID/PID.

mod finder;
mod usb_ids;

// ---------------------------------------------------------------------------------------------- //

pub use crate::discover::{
    finder::{find_port, wait_port},
    usb_ids::{PID_BRIDGE, PID_NATIVE, PID_R3, VID_ARDUINO},
};
