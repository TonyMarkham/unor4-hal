//! Host side of the validation pair (`r3-validator` + `r4_validator`). It
//! neither builds nor uploads: flash both boards first (README, Validation).
//!
//! `validate all` runs the bench's full sequence: S1-S8
//! and the S4 token, then S9 and the knock (unless `--skip-usb`), then the
//! reset bits. Exit code 0 only if every executed check passed; results go
//! to `target/validation/<timestamp>.txt`.

#![deny(missing_docs)]

mod checks;
mod cli;
mod discover;
mod report;
mod serial;

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------------------------- //

use crate::{
    checks::{identify, reset, slices, usb},
    cli::{Cli, Command},
    discover::{PID_BRIDGE, PID_R3, VID_ARDUINO, find_port},
    report::Report,
    serial::{Monitor, R3},
};

use clap::Parser as _;
use std::{process::ExitCode, time::Duration};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match dispatch(&cli) {
        Ok(passed) => ExitCode::from(u8::from(!passed)),
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(1)
        }
    }
}

/// Runs the requested command and reports whether every check passed.
fn dispatch(cli: &Cli) -> Result<bool, String> {
    let mut report = Report::new();
    match &cli.command {
        Command::All { skip_usb } => run_all(cli, &mut report, *skip_usb)?,
        Command::Identify => {
            let mut r3 = open_r3(cli)?;
            identify::run(&mut r3, &mut report);
        }
        Command::Slices { names } => {
            let (mut r3, mut monitor, bridge) = open_pair(cli)?;
            slices::run(&mut r3, &mut monitor, &bridge, names, &mut report);
        }
        Command::S4 => {
            let (mut r3, mut monitor, bridge) = open_pair(cli)?;
            slices::run(
                &mut r3,
                &mut monitor,
                &bridge,
                &["S4".to_owned()],
                &mut report,
            );
        }
        Command::Usb => {
            bridge_port(cli)?; // S9 needs the bench at rest: bridge port present
            let mut r3 = open_r3(cli)?;
            usb::run(&mut r3, &mut report);
        }
        Command::Reset { single, double } => {
            if *single == *double {
                return Err("pass exactly one of --single or --double".to_owned());
            }
            let mut r3 = open_r3(cli)?;
            if *single {
                reset::single(&mut r3, &mut report);
            } else {
                reset::double(&mut r3, &mut report);
            }
        }
    }
    report
        .finish()
        .map_err(|e| format!("writing artifacts failed: {e}"))?;
    Ok(report.failed() == 0)
}

/// Everything: S1-S8 + S4, S9 + knock, reset bits.
fn run_all(cli: &Cli, report: &mut Report, skip_usb: bool) -> Result<(), String> {
    let (mut r3, mut monitor, bridge) = open_pair(cli)?;
    println!("R3={} R4 bridge={bridge}", r3.name());
    slices::run(&mut r3, &mut monitor, &bridge, &["all".to_owned()], report);
    drop(monitor);
    if !skip_usb {
        usb::run(&mut r3, report);
    }
    reset::run(&mut r3, report);
    Ok(())
}

/// Opens the R3 validator port (9600 8N1), waits out the restart an open
/// may cause, and drains whatever the restart printed.
fn open_r3(cli: &Cli) -> Result<R3, String> {
    let name = match &cli.r3_port {
        Some(port) => port.clone(),
        None => find_port(PID_R3).ok_or_else(|| {
            format!("R3 ({VID_ARDUINO:04x}:{PID_R3:04x}) not found; pass --r3-port")
        })?,
    };
    let mut r3 = R3::open(&name).map_err(|e| format!("opening {name} failed: {e}"))?;
    std::thread::sleep(Duration::from_secs(2)); // the R3 may restart when its port opens
    let _ = r3.drain();
    Ok(r3)
}

/// The bridge port's name: the CLI override or VID/PID discovery.
fn bridge_port(cli: &Cli) -> Result<String, String> {
    match &cli.bridge_port {
        Some(port) => Ok(port.clone()),
        None => find_port(PID_BRIDGE).ok_or_else(|| {
            "R4 bridge port (2341:1002) not found. If the R4 holds native USB, \
             open that port at 1200 baud first (the knock)."
                .to_owned()
        }),
    }
}

/// Opens the R3 port and the bridge monitor, as the full run needs.
fn open_pair(cli: &Cli) -> Result<(R3, Monitor, String), String> {
    let bridge = bridge_port(cli)?;
    let mut monitor = Monitor::open(&bridge)
        .map_err(|e| format!("opening bridge monitor {bridge} failed: {e}"))?;
    let r3 = open_r3(cli)?;
    monitor.drain();
    Ok((r3, monitor, bridge))
}
