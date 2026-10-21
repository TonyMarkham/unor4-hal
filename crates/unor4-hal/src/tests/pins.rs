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

use crate::gpio::PORT_PINS;

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

#[cfg(feature = "wifi")]
#[test]
fn given_compiled_wifi_map_when_compared_to_golden_table_then_all_20_pins_match() {
    // given: the WiFi map compiled into the crate (feature `wifi`) and
    // the transcribed table
    // when: every compiled pin is looked up in the transcription
    // then: port letter and pin index agree — every one of the 20
    for (name, port, index) in PORT_PINS {
        let row = rows_of("wifi", GOLDEN.lines())
            .find(|row| row.1 == name)
            .unwrap_or_else(|| panic!("wifi row missing for {name}"));
        assert_eq!(row.2, port, "{name}: port letter");
        assert_eq!(row.3, index, "{name}: pin index");
    }
}

#[cfg(feature = "minima")]
#[test]
fn given_compiled_minima_map_when_compared_to_golden_table_then_all_20_pins_match() {
    // given: the Minima map compiled into the crate (feature `minima`)
    // and the transcribed table
    // when: every compiled pin is looked up in the transcription
    // then: port letter and pin index agree — every one of the 20
    for (name, port, index) in PORT_PINS {
        let row = rows_of("minima", GOLDEN.lines())
            .find(|row| row.1 == name)
            .unwrap_or_else(|| panic!("minima row missing for {name}"));
        assert_eq!(row.2, port, "{name}: port letter");
        assert_eq!(row.3, index, "{name}: pin index");
    }
}
