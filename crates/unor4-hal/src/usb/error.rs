//! CDC errors retain the underlying USB error instead of retrying forever.

use core::fmt;
use usb_device::UsbError;

/// An error returned by the CDC class or USB bus adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Error(pub UsbError);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "USB error: {:?}", self.0)
    }
}

/// embedded-io 0.7 requires its errors to be `core::error::Error`.
impl core::error::Error for Error {}

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl From<UsbError> for Error {
    fn from(error: UsbError) -> Self {
        Self(error)
    }
}
