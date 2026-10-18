//! Tests for the clock module's pure logic.

use crate::clock::SCKDIVCR_48MHZ;

#[test]
fn given_48mhz_sckdivcr_when_decoded_then_bclk_bits_mirror_pclkb() {
    // given: the divider value programmed at startup (SCKDIVCR = 0x1001_0100)
    let sckdivcr = SCKDIVCR_48MHZ;

    // when: the BCLK [18:16] and PCKB [10:8] divisor fields are decoded
    let bclk = (sckdivcr >> 16) & 0b111;
    let pckb = (sckdivcr >> 8) & 0b111;

    // then: BCLK mirrors PCLKB (the RA4M1 requirement) and both are /2
    assert_eq!(bclk, pckb);
    assert_eq!(pckb, 0b001);
}
