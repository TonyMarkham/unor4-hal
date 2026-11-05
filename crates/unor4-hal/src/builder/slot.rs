//! Slot names: markers named after the builder method that fills each
//! slot, lowercase on purpose, so the compiler's "not set" message shows the
//! method to call.
#![allow(non_camel_case_types)]

macro_rules! slots {
    ( $( $name:ident: $doc:literal; )* ) => {
        $(
            #[doc = $doc]
            #[derive(Clone, Copy, Debug)]
            pub struct $name;
        )*
    };
}

slots! {
    with_pin: "`with_pin(…)` of a pin setup builder.";
    with_timing: "`PulPinBuilder::with_timing`.";
    with_setup: "`DirPinBuilder::with_setup`.";
    with_forward: "`DirPinBuilder::with_forward`.";
    with_enabled: "`EnaPinBuilder::with_enabled`.";
    with_lead: "`EnaPinBuilder::with_lead`.";
    with_contact: "`SwitchPinBuilder::with_contact`.";
    with_pull: "`SwitchPinBuilder::with_pull`.";
    with_timer: "`with_timer` of `CounterBuilder` or `WakeTimerBuilder`.";
    with_timers: "`TickBuilder::with_timers`.";
    with_pul: "`StepGeneratorBuilder::with_pul`.";
    with_dir: "`StepGeneratorBuilder::with_dir`.";
    with_counter: "`StepGeneratorBuilder::with_counter`.";
    with_tick: "`StepGeneratorBuilder::with_tick`.";
    with_peripherals: "`with_peripherals(…)` of a run-time builder.";
    with_edge: "`SwitchPinBuilder::with_edge`.";
    with_input: "`WakeTimerBuilder::with_input`.";
}
