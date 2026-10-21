//! The validation checks, one module per bench interaction.

mod boot_event;
pub mod identify;
pub mod reset;
pub mod retention;
pub mod slices;
pub mod usb;

#[cfg(test)]
pub(crate) use crate::checks::{boot_event::BootEvent, slices::requested};
