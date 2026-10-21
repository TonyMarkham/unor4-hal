//! Host-side unit tests for the validation CLI's pure logic. Compiled
//! only by `cargo test` (see the `#[cfg(test)]` gate in main.rs).

mod frames;
mod results;
mod retention;
mod slices;
mod timestamp;
mod verdict;
