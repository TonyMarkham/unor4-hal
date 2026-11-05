//! Switch input facts: contact levels and IRQ edges (manual §13.2.1), the
//! filter clock codes, and the IRQ → ICU event map (Table 13.4 / 18.3:
//! `PORT_IRQn` = 001h–010h, no IRQ13).

use crate::{
    gpio::{
        SwitchFilter,
        contact::{Contact, Nc, No},
    },
    intc::Event,
};
use embedded_hal::digital::PinState;

/// The pin table's source, scanned for its `irq n` facts.
const PIN_TABLE: &str = include_str!("../gpio/pin_map.rs");

#[test]
fn given_contact_types_when_read_then_no_is_active_low_on_a_falling_edge_and_nc_active_high_on_a_rising_edge()
 {
    // given / when
    let no = (No::ACTIVE, No::IRQMD);
    let nc = (Nc::ACTIVE, Nc::IRQMD);

    // then
    assert_eq!(no, (PinState::Low, 0b00));
    assert_eq!(nc, (PinState::High, 0b01));
}

#[test]
fn given_filter_clocks_when_encoded_then_fclksel_matches_the_manual() {
    // given
    let filters = [
        SwitchFilter::Pclkb,
        SwitchFilter::PclkbDiv8,
        SwitchFilter::PclkbDiv32,
        SwitchFilter::PclkbDiv64,
    ];

    // when
    let codes = filters.map(SwitchFilter::fclksel);

    // then
    assert_eq!(codes, [0b00, 0b01, 0b10, 0b11]);
}

#[test]
fn given_irq_numbers_when_mapped_then_codes_follow_port_irq_and_13_is_absent() {
    // given / when / then
    for irq in 0..=12u8 {
        assert_eq!(Event::gpio_irq(irq).map(|e| e as u8), Some(irq + 1));
    }
    assert_eq!(Event::gpio_irq(13), None);
    assert_eq!(Event::gpio_irq(14).map(|e| e as u8), Some(0x0F));
    assert_eq!(Event::gpio_irq(15).map(|e| e as u8), Some(0x10));
    assert_eq!(Event::gpio_irq(16), None);
}

#[test]
fn given_the_pin_table_when_its_irq_facts_are_mapped_then_every_one_has_an_icu_event() {
    // given
    let facts = PIN_TABLE.split("irq ").skip(1).filter_map(|rest| {
        let digits = rest.split(|c: char| !c.is_ascii_digit()).next()?;
        digits.parse::<u8>().ok()
    });

    // when
    let mut count = 0;
    for irq in facts {
        count += 1;

        // then
        assert!(Event::gpio_irq(irq).is_some(), "irq {irq} has no ICU event");
    }
    assert!(count >= 15, "only {count} irq facts found: the scan broke");
}

#[test]
fn given_every_header_when_board_pull_downs_are_read_then_only_wifi_d13_has_one() {
    // given
    let mut loaded = [("", 0u32); 20];
    let mut count = 0;

    // when
    crate::gpio::pin_map::for_each_header(|name, port, pin| {
        let ohms = crate::gpio::pin_map::board_pull_down_ohms(port, pin);
        if ohms != 0 {
            loaded[count] = (name, ohms);
            count += 1;
        }
    });

    // then
    #[cfg(feature = "wifi")]
    assert_eq!(&loaded[..count], &[("d13", 100_000)]);
    #[cfg(feature = "minima")]
    assert_eq!(count, 0, "Minima board loads are unverified: none recorded");
}
