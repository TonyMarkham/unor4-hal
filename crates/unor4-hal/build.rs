//! Ships an empty `device.x` for cortex-m-rt's `device` feature.
//!
//! Firmware links with cortex-m-rt built with `features = ["device"]`,
//! which makes `link.x` end with `INCLUDE device.x` — the hook through
//! which PACs normally supply weak default handler aliases. This HAL
//! needs none: its interrupt table (`src/intc/dispatch.rs`) defines the
//! 32 `IELn` trampolines itself, so the file only has to exist for the
//! `INCLUDE` to resolve. The `rustc-link-search` emitted here puts it on
//! the linker's search path.

use std::{env, fs, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::write(
        out.join("device.x"),
        "/* unor4-hal: __INTERRUPTS is provided by src/intc/dispatch.rs;\n\
           no weak handler aliases are needed. */\n",
    )
    .unwrap();
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
}
