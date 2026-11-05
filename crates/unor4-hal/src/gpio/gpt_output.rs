//! Which of a GPT channel's two outputs a pin carries.

/// One of a GPT channel's two outputs (manual §22.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GptOutput {
    /// `GTIOCnA`: compare register `GTCCRA`.
    A,
    /// `GTIOCnB`: compare register `GTCCRB`.
    B,
}
