//! The validation checks, one module per bench interaction.

pub mod identify;
pub mod reset;
pub mod slices;
pub mod usb;

#[cfg(test)]
pub(crate) use crate::checks::slices::requested;
