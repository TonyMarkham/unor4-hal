//! One boot of the R4 as the R3's S10 watch reports it.

use regex::Regex;

/// One `S10 BOOT <count> <intact> <delta> <gap>` line from the R3 watch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BootEvent {
    /// The boot counter the R4 read from its data flash log.
    pub count: i64,
    /// Whether every logged slot read back its value.
    pub intact: bool,
    /// The counter's change since the previous reply.
    pub delta: i64,
    /// Whether unanswered polls (R4 unpowered or restarting) came first.
    pub gap: bool,
}

impl BootEvent {
    /// Every boot event in `text`, in order.
    #[must_use]
    pub fn parse_all(text: &str) -> Vec<Self> {
        let line = Regex::new(r"(?m)^S10 BOOT (\d+) (\d+) (-?\d+) ([01])\r?$").unwrap();
        line.captures_iter(text)
            .map(|caps| Self {
                count: caps[1].parse().unwrap_or(-1),
                intact: &caps[2] == "1",
                delta: caps[3].parse().unwrap_or(0),
                gap: &caps[4] == "1",
            })
            .collect()
    }

    /// The baseline counter from the watch's `S10 BASE <count> <intact>`
    /// line, if any.
    #[must_use]
    pub fn baseline(text: &str) -> Option<(i64, bool)> {
        let line = Regex::new(r"(?m)^S10 BASE (\d+) (\d+)\r?$").unwrap();
        line.captures(text)
            .map(|caps| (caps[1].parse().unwrap_or(-1), &caps[2] == "1"))
    }

    /// One boot exactly: the counter advanced by one and the log is
    /// intact. Counting restarts at 1 when the log block fills, which
    /// also counts as one boot.
    #[must_use]
    pub fn is_single_boot(&self) -> bool {
        self.intact && (self.delta == 1 || self.count == 1)
    }
}
