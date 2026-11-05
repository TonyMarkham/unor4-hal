//! Host side of the validation pair (`r3-validator` + `r4_validator`). It
//! neither builds nor uploads: flash both boards first (Validation.md).
//!
//! The R3 validator builds as three parts, each uploaded for its own
//! session (the checks no longer fit the R3's 32 KB in one image).
//! `validate all` runs the loaded part's session:
//! - **part 1:** S1-S8, the S10 flash self-test and the S4 token, then S10
//!   retention across three resets, then S9 and the knock (unless
//!   `--skip-usb`), then the reset bits;
//! - **part 2:** the first S11 checks (step, count, tick, queue, scenario,
//!   reverse, wake, skip, elc), then the reset bits;
//! - **part 3:** the later S11 checks (dirlead, countstop, cancelrace,
//!   sleep, boot), then the reset bits.
//!
//! The other parts' checks are recorded as NOT RUN. Exit code 0 only if
//! every executed check passed; results go to
//! `target/validation/<timestamp>.txt`.

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
    checks::{identify, reset, retention, slices, usb},
    cli::{Cli, Command},
    discover::{PID_BRIDGE, PID_R3, VID_ARDUINO, find_port},
    report::Report,
    serial::{Monitor, R3, R3_PARTS},
};

use clap::Parser as _;
use std::{process::ExitCode, time::Duration};

/// The R3 validator part with the boot-counter watch (`k`) and the USB
/// take-over (`u`): S10 retention and S9 run only on it.
const HOST_CHECKS_PART: u8 = 1;

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
            let part = r3_part(&mut r3)?;
            slices::run(&mut r3, &mut monitor, &bridge, names, part, &mut report);
        }
        Command::S4 => {
            let (mut r3, mut monitor, bridge) = open_pair(cli)?;
            let part = r3_part(&mut r3)?;
            slices::run(
                &mut r3,
                &mut monitor,
                &bridge,
                &["S4".to_owned()],
                part,
                &mut report,
            );
        }
        Command::Usb => {
            bridge_port(cli)?; // S9 needs the bench at rest: bridge port present
            let mut r3 = open_r3(cli)?;
            need_host_checks_part(&mut r3, "S9")?;
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
        Command::S10 {
            resets,
            watch,
            min_boots,
        } => {
            let mut r3 = open_r3(cli)?;
            need_host_checks_part(&mut r3, "S10 retention")?;
            match watch {
                Some(seconds) => retention::watch(&mut r3, *seconds, *min_boots, &mut report),
                None => retention::resets(&mut r3, *resets, &mut report),
            }
        }
    }
    report
        .finish()
        .map_err(|e| format!("writing artifacts failed: {e}"))?;
    Ok(report.failed() == 0)
}

/// The loaded part's session: its slice checks, the host checks on part 1
/// (S10 retention, S9 + knock), the reset bits on every part. The other parts'
/// checks are recorded as not run.
fn run_all(cli: &Cli, report: &mut Report, skip_usb: bool) -> Result<(), String> {
    let (mut r3, mut monitor, bridge) = open_pair(cli)?;
    let part = r3_part(&mut r3)?;
    println!(
        "R3={} (validator part {part} of {R3_PARTS}) R4 bridge={bridge}",
        r3.name()
    );
    slices::run(
        &mut r3,
        &mut monitor,
        &bridge,
        &["all".to_owned()],
        part,
        report,
    );
    drop(monitor);
    if part == HOST_CHECKS_PART {
        retention::resets(&mut r3, 3, report);
        if !skip_usb {
            usb::run(&mut r3, report);
        }
    } else {
        slices::not_run("S10 retention", HOST_CHECKS_PART, report);
        if !skip_usb {
            slices::not_run("S9", HOST_CHECKS_PART, report);
            slices::not_run("Knock", HOST_CHECKS_PART, report);
        }
    }
    reset::run(&mut r3, report);
    Ok(())
}

/// The R3 validator part that answers on `r3`, or an error naming the fix.
fn r3_part(r3: &mut R3) -> Result<u8, String> {
    r3.part().ok_or_else(|| {
        format!(
            "the R3 on {} names no validator part; upload part 1, 2 or 3 of \
             arduino/validation/r3-validator (Validation.md)",
            r3.name()
        )
    })
}

/// Refuses `check` unless the R3 runs the part with the host-check commands.
fn need_host_checks_part(r3: &mut R3, check: &str) -> Result<(), String> {
    let part = r3_part(r3)?;
    if part == HOST_CHECKS_PART {
        Ok(())
    } else {
        Err(format!(
            "{check} runs on R3 validator part {HOST_CHECKS_PART}; part {part} is loaded \
             (Validation.md)"
        ))
    }
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
