//! Host-side unit tests for the validation CLI's pure logic. Compiled
//! only by `cargo test` (see the `#[cfg(test)]` gate in main.rs).

mod frames;
mod r3_part;
mod results;
mod retention;
mod slices;
mod timestamp;
mod verdict;
