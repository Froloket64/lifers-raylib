use std::time::{Duration, Instant};

pub enum TimerState {
    Ongoing, // or whatever
    Finished,
    Paused,
}

/// Repeating timer
///
/// Uses [`std::time::Instant`], so it is monotonic.
pub struct RepeatingTimer {
    amount: Duration,
    time_left: Duration,
    last_checked: Instant,
    paused: bool,
}

impl RepeatingTimer {
    /// Creates a new timer.
    pub fn new(amount: Duration) -> Self {
        Self {
            amount,
            time_left: amount,
            last_checked: Instant::now(),
            paused: false,
        }
    }

    /// Updates the timer with the current time, returning whether the given
    /// amount has passed or not.
    pub fn update(&mut self) -> TimerState {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_checked);

        self.last_checked = now;

        if self.paused {
            TimerState::Paused
        } else {
            self.update_state(elapsed)
        }
    }

    /// Like [`update()`](Self::update), but uses [`Instant::checked_duration_since()`].
    #[allow(dead_code)]
    pub fn checked_update(&mut self) -> Option<TimerState> {
        let now = Instant::now();
        let elapsed = now.checked_duration_since(self.last_checked);

        self.last_checked = now;

        elapsed.map(|x| self.update_state(x))
    }

    fn update_state(&mut self, elapsed: Duration) -> TimerState {
        if elapsed >= self.time_left {
            self.time_left = if elapsed > self.amount {
                self.amount
            } else {
                // NOTE: `elapsed` is checked to be greater than or equal to
                // `self.time_left`
                #[allow(clippy::arithmetic_side_effects)]
                self.amount
                    .checked_sub(elapsed - self.time_left)
                    .unwrap_or(self.amount)
            };

            TimerState::Finished
        } else {
            // NOTE: `elapsed` is checked to be smaller than `self.time_left`
            #[allow(clippy::arithmetic_side_effects)]
            {
                self.time_left -= elapsed;
            }

            TimerState::Ongoing
        }
    }

    /// Pauses or unpauses the timer, depending on it's current state.
    ///
    /// When paused, [`update()`](Self::update()) and similar
    /// methods stop advancing the time and return [`TimerState::Paused`].
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    /// Returns the amount of time that each cycle takes.
    pub fn rate(&self) -> Duration {
        self.amount
    }
}
