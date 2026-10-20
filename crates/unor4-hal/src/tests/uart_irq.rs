//! Regression for the foreground/ISR TDR ownership race.

use crate::uart::can_prime_tx;

#[test]
fn given_active_txi_and_empty_tdr_when_write_queues_then_priming_is_forbidden() {
    // given: TIE is on and TDRE is set; TXI may already be pending.
    let scr = 0xf0;
    // when: the foreground writer considers taking over TDR.
    let prime = can_prime_tx(scr, true);
    // then: the ISR alone owns TDR; priming here could lose a byte.
    assert!(!prime);
}

#[test]
fn given_idle_txi_and_empty_tdr_when_write_queues_then_priming_is_allowed() {
    // given: TE/RE/RIE on, TIE off, and an empty data register.
    let scr = 0x70;
    // when: a new queued write needs to start the transmitter.
    let prime = can_prime_tx(scr, true);
    // then: foreground owns the idle transmitter and can prime it.
    assert!(prime);
}

#[test]
fn given_idle_txi_and_full_tdr_when_write_queues_then_priming_is_forbidden() {
    // given: TIE off but TDR still contains an unsent byte.
    let scr = 0x70;
    // when: the foreground writer checks whether it may prime.
    let prime = can_prime_tx(scr, false);
    // then: never overwrite the data register while it is full.
    assert!(!prime);
}
