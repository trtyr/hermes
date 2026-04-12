//! Memory - secure heap allocation

use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

static KEY: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());
static KEY_INIT: AtomicUsize = AtomicUsize::new(0);

fn ensure_key_init() {
    if KEY_INIT.load(Ordering::SeqCst) == 0 {
        let pid = std::process::id() as u64;
        let mut key = Box::new([0u8; 32]);

        for i in 0..32 {
            key[i] = ((pid >> (i % 8)) ^ (i as u64) ^ 0x5A) as u8;
        }

        KEY.store(Box::into_raw(key) as *mut u8, Ordering::SeqCst);
        KEY_INIT.store(1, Ordering::SeqCst);
    }
}

fn key() -> &'static [u8; 32] {
    ensure_key_init();
    unsafe { &*(KEY.load(Ordering::SeqCst) as *const [u8; 32]) }
}

/// Server address with encrypted storage
pub struct SecureServerAddr {
    encrypted: Vec<u8>,
}

impl SecureServerAddr {
    pub fn new(addr: &str) -> Self {
        ensure_key_init();
        let k = key();
        let bytes = addr.as_bytes();
        let mut enc = vec![0u8; bytes.len()];

        for (i, &b) in bytes.iter().enumerate() {
            enc[i] = b ^ k[i % 32];
        }

        Self { encrypted: enc }
    }

    pub fn get(&self) -> String {
        let k = key();
        let mut dec = Vec::with_capacity(self.encrypted.len());

        for (i, &b) in self.encrypted.iter().enumerate() {
            dec.push(b ^ k[i % 32]);
        }

        String::from_utf8_lossy(&dec).to_string()
    }
}

impl Drop for SecureServerAddr {
    fn drop(&mut self) {
        for b in &mut self.encrypted {
            *b = 0;
        }
    }
}
