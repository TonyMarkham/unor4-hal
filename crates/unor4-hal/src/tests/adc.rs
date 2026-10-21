//! Host tests for the ADC module — the golden channel table (a
//! hardware fact from manual §19 Tables 19.5/19.6 + the datasheet pin
//! tables, cross-checked against the Arduino core's `pinmux.inc`) and
//! the scan-time calculator
//! (manual §35.3.6 Table 35.10's arithmetic).

use crate::adc::{Channel, Resolution, ScanTimeParams, calc_scan_time};
use crate::gpio::{Analog, Pin};

const PCLKB_24MHZ: u32 = 24_000_000;
const ADCLK_48MHZ: u32 = 48_000_000;

#[test]
fn given_header_analog_pins_when_channel_read_then_matches_the_manual_map() {
    // given: the six header analog pins, in analog state
    // when: their AN channel constants are read
    // then: every entry equals the verified pin-function map — A0's
    // channel is AN009 (P014 also carries DA0), A1–A3 are AN000–AN002,
    // and A4/A5 are the PORT1 high-numbered channels AN021/AN022.
    assert_eq!(<Pin<'0', 14, Analog> as Channel>::AN, 9);
    assert_eq!(<Pin<'0', 0, Analog> as Channel>::AN, 0);
    assert_eq!(<Pin<'0', 1, Analog> as Channel>::AN, 1);
    assert_eq!(<Pin<'0', 2, Analog> as Channel>::AN, 2);
    assert_eq!(<Pin<'1', 1, Analog> as Channel>::AN, 21);
    assert_eq!(<Pin<'1', 0, Analog> as Channel>::AN, 22);
}

#[test]
fn given_resolution_when_mask_asked_then_right_justified_width() {
    // given: both supported resolutions
    // when: their masks/maxima are taken
    // then: 12-bit fills bits 11:0 (4095), 14-bit bits 13:0 (16383) —
    // the §35.2.1 right-justified data formats.
    assert_eq!(Resolution::Bits12.mask(), 0x0FFF);
    assert_eq!(Resolution::Bits12.max(), 4_095);
    assert_eq!(Resolution::Bits14.mask(), 0x3FFF);
    assert_eq!(Resolution::Bits14.max(), 16_383);
    assert_eq!(Resolution::Bits12.bits(), 12);
    assert_eq!(Resolution::Bits14.bits(), 14);
}

#[test]
fn given_reset_sst_12bit_highspeed_when_scan_time_computed_then_1206ns() {
    // given: this crate's clock tree (PCLKB 24 MHz, ADCLK 48 MHz), the
    // ADSSTRn reset value 0Dh (13 states), 12-bit high-speed — the
    // driver's shipped configuration
    // when: one conversion time is computed
    // then: t_D = 166 ns, t_SPL = 281 ns, t_SAM = 656 ns, t_ED = 103 ns
    // (integer ns, per-term floor) — total 1206 ns, matching the
    // research doc's ~1.21 µs.
    let t = calc_scan_time(ScanTimeParams {
        pclkb_hz: PCLKB_24MHZ,
        adclk_hz: ADCLK_48MHZ,
        sst_states: 13,
        resolution: Resolution::Bits12,
        low_current: false,
    });
    assert_eq!(t.start_delay_ns, 166);
    assert_eq!(t.sampling_ns, 281);
    assert_eq!(t.sar_ns, 656);
    assert_eq!(t.end_delay_ns, 103);
    assert_eq!(t.total_ns, 1206);
}

#[test]
fn given_14bit_highspeed_when_scan_time_computed_then_1331ns() {
    // given: the same clocks and SST, at full 14-bit resolution
    // when: one conversion time is computed
    // then: only t_SAM changes (37.5 ADCLK = 781 ns) — 1331 ns total.
    let t = calc_scan_time(ScanTimeParams {
        pclkb_hz: PCLKB_24MHZ,
        adclk_hz: ADCLK_48MHZ,
        sst_states: 13,
        resolution: Resolution::Bits14,
        low_current: false,
    });
    assert_eq!(t.sar_ns, 781);
    assert_eq!(t.total_ns, 1331);
}

#[test]
fn given_14bit_lowcurrent_when_scan_time_computed_then_1518ns() {
    // given: the same clocks and SST, 14-bit low-current (`ADHSC = 1`)
    // when: one conversion time is computed
    // then: t_SAM = 46.5 ADCLK = 968 ns — 1518 ns total; still far
    // below any realistic sampling cadence.
    let t = calc_scan_time(ScanTimeParams {
        pclkb_hz: PCLKB_24MHZ,
        adclk_hz: ADCLK_48MHZ,
        sst_states: 13,
        resolution: Resolution::Bits14,
        low_current: true,
    });
    assert_eq!(t.sar_ns, 968);
    assert_eq!(t.total_ns, 1518);
}

#[test]
fn given_sst_28_states_when_scan_time_computed_then_sampling_doubles_ish() {
    // given: SST = 28h (40 states) — the datasheet's higher-impedance
    // operating point (source impedance up to ~1.3 kΩ at 48 MHz,
    // Table 2.43)
    // when: the sampling term is computed
    // then: t_SPL = 40.5 ADCLK = 843 ns (vs 281 ns at 13 states) — the
    // knob the manual names for high-impedance sources (§35.3.6).
    let t = calc_scan_time(ScanTimeParams {
        pclkb_hz: PCLKB_24MHZ,
        adclk_hz: ADCLK_48MHZ,
        sst_states: 40,
        resolution: Resolution::Bits14,
        low_current: false,
    });
    assert_eq!(t.sampling_ns, 843);
    assert_eq!(t.total_ns, 1893);
}
