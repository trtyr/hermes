//! Heartbeat Service

use crate::kernel::Plugin;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub struct HeartbeatService {
    interval_secs: u64,
    jitter: u32,
    next_due: Instant,
    sequence: u64,
}

impl HeartbeatService {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            interval_secs: 15,
            jitter: 0,
            next_due: now + Duration::from_secs(15),
            sequence: 0,
        }
    }

    pub fn should_send(&self) -> bool {
        Instant::now() >= self.next_due
    }

    pub fn sent(&mut self) {
        self.schedule_from(Instant::now());
    }

    pub fn update(&mut self, secs: u64, jitter: u32) {
        self.interval_secs = secs.max(1);
        self.jitter = jitter;
        self.schedule_from(Instant::now());
    }

    pub fn interval_secs(&self) -> u64 {
        self.interval_secs
    }

    pub fn jitter(&self) -> u32 {
        self.jitter
    }

    pub fn wait_duration(&self) -> Duration {
        self.next_due.saturating_duration_since(Instant::now())
    }

    fn schedule_from(&mut self, now: Instant) {
        let base_ms = self.interval_secs.saturating_mul(1000);
        let max_jitter_ms = base_ms.saturating_mul(self.jitter as u64) / 100;
        let jitter_ms = if max_jitter_ms == 0 {
            0
        } else {
            self.sequence = self.sequence.wrapping_add(1);
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos() as u64)
                .unwrap_or(0)
                ^ ((std::process::id() as u64) << 16)
                ^ self.sequence;
            seed % (max_jitter_ms + 1)
        };
        self.next_due = now + Duration::from_millis(base_ms.saturating_add(jitter_ms));
    }
}

impl Plugin for HeartbeatService {
    fn name(&self) -> &'static str {
        "heartbeat"
    }
}

impl Default for HeartbeatService {
    fn default() -> Self {
        Self::new()
    }
}
