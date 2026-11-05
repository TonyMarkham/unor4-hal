//! Golden pin-table cross-check: the map compiled into
//! the crate must equal the transcription of the Arduino core's board
//! tables. This is the test that would have caught the existing HAL's
//! docs.rs trap automatically.
//!
//! Core-only by design: no `std`, no `alloc` — `Vec` never appears. The
//! file therefore analyzes and compiles identically under every cfg the
//! toolchain can throw at it (host `cargo test`, rust-analyzer's
//! no_std view of a `no_std` crate, firmware builds), while the tests
//! themselves still run on the host via `cargo test`.

use crate::gpio::pin_map::for_each_header;

/// The checked-in transcription, living next to the crate manifest:
/// `pins.csv`, transcribed from `variants/MINIMA/variant.cpp` and
/// `variants/UNOWIFIR4/variant.cpp` (cross-check data, never copied
/// code).
const GOLDEN: &str = include_str!("pins.csv");

/// One parsed row: `(board, pin name, port letter, pin index)`.
type Row<'a> = (&'a str, &'a str, char, u8);

/// Parses one data line: `wifi,d2,P104` → `("wifi", "d2", '1', 4)`.
/// Comments, blank lines and the column header decode to `None`.
fn parse_row(line: &str) -> Option<Row<'_>> {
    let mut cells = line.split(',');
    let board = cells.next()?;
    let name = cells.next()?;
    let port_pin = cells.next()?;
    let bytes = port_pin.as_bytes();
    if bytes.len() != 4 || bytes[0] != b'P' {
        return None;
    }
    Some((board, name, bytes[1] as char, port_pin[2..].parse().ok()?))
}

/// Iterates one board's data rows, in file order.
fn rows_of<'a>(board: &str, lines: impl Iterator<Item = &'a str>) -> impl Iterator<Item = Row<'a>> {
    lines
        .filter_map(parse_row)
        .filter(move |row| row.0 == board)
}

#[test]
fn given_golden_table_when_parsed_then_both_boards_list_the_same_20_header_pins() {
    // given: the transcription checked in next to the crate
    // when: the data rows of both boards are paired positionally
    let mut wifi = rows_of("wifi", GOLDEN.lines());
    let mut minima = rows_of("minima", GOLDEN.lines());
    let mut count = 0;
    let mut aligned = true;
    loop {
        match (wifi.next(), minima.next()) {
            (Some(w), Some(m)) => {
                count += 1;
                if w.1 != m.1 {
                    aligned = false; // same position, different pin name
                }
            }
            (None, None) => break,
            _ => aligned = false, // one section is longer than the other
        }
    }

    // then: 20 header pins on both boards, same names in the same order
    assert_eq!(count, 20, "both boards must list 20 header pins");
    assert!(aligned, "board sections diverge — see pins.csv");
}

/// The board the crate is compiled for.
#[cfg(feature = "wifi")]
const BOARD: &str = "wifi";
/// The board the crate is compiled for.
#[cfg(feature = "minima")]
const BOARD: &str = "minima";

#[test]
fn given_compiled_map_when_compared_to_golden_table_then_all_20_pins_match() {
    // given: the map compiled into the crate (feature `wifi` or `minima`)
    // and the transcribed table
    let mut count = 0;

    // when: every compiled header is looked up in the transcription
    for_each_header(|name, port, index| {
        count += 1;
        let row = rows_of(BOARD, GOLDEN.lines())
            .find(|row| row.1 == name)
            .unwrap_or_else(|| panic!("{BOARD} row missing for {name}"));

        // then: port letter and pin index agree
        assert_eq!(row.2, port, "{name}: port letter");
        assert_eq!(row.3, index, "{name}: pin index");
    });

    // then: all 20 headers were compiled
    assert_eq!(count, 20, "the {BOARD} map must compile 20 header pins");
}
