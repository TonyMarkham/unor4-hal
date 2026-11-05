//! R3 validator part regressions: the help-line parse and which part runs
//! each check.

use crate::{checks::LINE_PATTERNS, serial::parse_part};

#[test]
fn given_each_part_help_line_when_parsed_then_the_part_results() {
    // given
    let lines = [
        "r3-validator part 1 of 3: see header\r\n",
        "r3-validator part 2 of 3: see header\r\n",
        "r3-validator part 3 of 3: see header\r\n",
    ];
    // when
    let parts: Vec<Option<u8>> = lines.iter().map(|line| parse_part(line)).collect();
    // then
    assert_eq!(parts, [Some(1), Some(2), Some(3)]);
}

#[test]
fn given_the_old_single_image_help_line_when_parsed_then_no_part_results() {
    // given
    let line = "r3-validator: see header\r\n";
    // when
    let part = parse_part(line);
    // then
    assert_eq!(part, None);
}

#[test]
fn given_a_part_outside_the_build_when_parsed_then_no_part_results() {
    // given: a part past the count, part 0, and the two-part build's line
    let lines = [
        "r3-validator part 4 of 3: see header",
        "r3-validator part 0 of 3: see header",
        "r3-validator part 1 of 2: see header",
    ];
    // when
    let parts: Vec<Option<u8>> = lines.iter().map(|line| parse_part(line)).collect();
    // then
    assert_eq!(parts, [None, None, None]);
}

#[test]
fn given_the_line_patterns_when_grouped_then_each_check_is_in_its_part() {
    // given: part 3 holds Slice 11's later checks and Slice 12, part 2 the
    // rest of Slice 11, part 1 everything else
    let part_3 = [
        "S11 dirlead",
        "S11 countstop",
        "S11 cancelrace",
        "S11 sleep",
        "S11 boot",
        "S12 wake",
    ];
    // when
    let misplaced: Vec<&str> = LINE_PATTERNS
        .iter()
        .filter(|(part, name, _)| {
            let want = if part_3.contains(name) {
                3
            } else if name.starts_with("S11") {
                2
            } else {
                1
            };
            *part != want
        })
        .map(|(_, name, _)| *name)
        .collect();
    // then
    assert!(misplaced.is_empty(), "misplaced: {misplaced:?}");
}
