//! Host tests for the ELC tables. The register paths (module stop, `ELSRn`,
//! `ELSEGRn`) are target code — they are proven by Slice 11 step 1's
//! hardware run (Validation.md, `S11 elc`), not here.

use crate::{
    elc::{ALL_ELC_EVENTS, ALL_GPT_INPUTS, ElcEvent, GptInput},
    gpio::GptOutput,
};

#[test]
fn given_elc_events_when_codes_read_then_they_match_table_18_3() {
    // given: the events this crate routes
    // when: their ELS codes are read off
    // then: each equals the manual's Table 18.3 entry
    assert_eq!(ElcEvent::SoftwareEvent0.code(), 0x53); // ELC_SWEVT0
    assert_eq!(ElcEvent::SoftwareEvent1.code(), 0x54); // ELC_SWEVT1
    assert_eq!(ElcEvent::Gpt0CompareB.code(), 0x58); // GPT0_CCMPB
    assert_eq!(ElcEvent::Gpt4Overflow.code(), 0x7D); // GPT4_OVF
}

#[test]
fn given_every_gpt_channel_when_events_looked_up_then_they_follow_table_18_3_spacing() {
    // given: channels 0–7
    for channel in 0..8u8 {
        // when: compare A/B and overflow are looked up
        let a = ElcEvent::gpt_compare(channel, GptOutput::A).expect("channel ≤ 7");
        let b = ElcEvent::gpt_compare(channel, GptOutput::B).expect("channel ≤ 7");
        let ovf = ElcEvent::gpt_overflow(channel).expect("channel ≤ 7");

        // then: 057h + 8n, 058h + 8n, 05Dh + 8n (Table 18.3)
        assert_eq!(a.code(), 0x57 + 8 * channel, "GPT{channel}_CCMPA");
        assert_eq!(b.code(), 0x58 + 8 * channel, "GPT{channel}_CCMPB");
        assert_eq!(ovf.code(), 0x5D + 8 * channel, "GPT{channel}_OVF");
    }
    assert_eq!(ElcEvent::gpt_overflow(8), None);
}

#[test]
fn given_elc_events_when_enumerated_then_codes_are_unique_and_selectable() {
    // given: every listed event
    let mut codes: Vec<u8> = ALL_ELC_EVENTS.iter().map(|event| event.code()).collect();
    // when: duplicates are removed
    codes.sort_unstable();
    let count = codes.len();
    codes.dedup();
    // then: none collide, and each is inside ELS's allowed range
    // 00000001b–10110110b (§18.2.3)
    assert_eq!(codes.len(), count);
    assert!(codes.iter().all(|&code| (0x01..=0xB6).contains(&code)));
}

#[test]
fn given_gpt_inputs_when_indexed_then_they_are_elsr0_to_elsr7_in_order() {
    // given: the eight GPT inputs A–H
    // when: their ELSR indices are read
    let indices: Vec<u8> = ALL_GPT_INPUTS.iter().map(|input| input.index()).collect();
    // then: A..H map to ELSR0..ELSR7 with no gap (Table 18.2)
    assert_eq!(indices, (0..8).collect::<Vec<u8>>());
    assert_eq!(GptInput::A.index(), 0);
    assert_eq!(GptInput::H.index(), 7);
}
