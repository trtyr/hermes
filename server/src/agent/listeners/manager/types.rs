use std::time::{Duration, Instant};

use tokio::task::JoinHandle;

pub(super) struct ActiveListener {
    pub(super) fingerprint: String,
    pub(super) handle: JoinHandle<()>,
    pub(super) started_at: Instant,
    pub(super) restart_attempts: u32,
    pub(super) restart_not_before: Instant,
}

impl ActiveListener {
    pub(super) fn new(fingerprint: String, handle: JoinHandle<()>) -> Self {
        let now = Instant::now();
        Self {
            fingerprint,
            handle,
            started_at: now,
            restart_attempts: 0,
            restart_not_before: now,
        }
    }

    pub(super) fn note_restart(&mut self) {
        let now = Instant::now();
        let next_attempt = self.restart_attempts.saturating_add(1);
        self.restart_attempts = next_attempt;
        self.started_at = now;
        self.restart_not_before = now + restart_backoff(next_attempt);
    }

    pub(super) fn maybe_reset_restart_backoff(&mut self) {
        if self.restart_attempts == 0 {
            return;
        }
        if self.started_at.elapsed() >= LISTENER_RESTART_STABLE_WINDOW {
            self.restart_attempts = 0;
            self.restart_not_before = Instant::now();
        }
    }
}

const LISTENER_RESTART_STABLE_WINDOW: Duration = Duration::from_secs(30);
const LISTENER_RESTART_MAX_BACKOFF_SECS: u64 = 30;

fn restart_backoff(attempt: u32) -> Duration {
    let shift = attempt.saturating_sub(1).min(4);
    Duration::from_secs((1u64 << shift).min(LISTENER_RESTART_MAX_BACKOFF_SECS))
}

#[cfg(test)]
mod tests {
    use super::restart_backoff;

    #[test]
    fn restart_backoff_grows_and_caps() {
        assert_eq!(restart_backoff(0).as_secs(), 1);
        assert_eq!(restart_backoff(1).as_secs(), 1);
        assert_eq!(restart_backoff(2).as_secs(), 2);
        assert_eq!(restart_backoff(3).as_secs(), 4);
        assert_eq!(restart_backoff(4).as_secs(), 8);
        assert_eq!(restart_backoff(5).as_secs(), 16);
        assert_eq!(restart_backoff(6).as_secs(), 16);
    }
}
