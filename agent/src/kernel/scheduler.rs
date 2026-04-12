//! Scheduler - minimal task scheduling

use std::time::Duration;

/// Kernel struct - provides basic timing
pub struct Kernel;

impl Kernel {
    pub fn new() -> Self {
        Self
    }

    pub fn sleep(&self, dur: Duration) {
        std::thread::sleep(dur);
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}
