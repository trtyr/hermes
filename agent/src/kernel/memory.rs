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

/// Server address with XOR-encrypted heap storage, zeroed on drop.
///
/// The XOR key is derived from the process PID at first use, so the address
/// is not recoverable from a raw memory scan after the process exits.
/// This is *obfuscation*, not cryptographic security — it defeats casual
/// string scanning (e.g. `strings` on the binary or a quick `/proc/mem` read)
/// but not a determined attacker with live process access.
pub struct SecureServerAddr {
    encrypted: Vec<u8>,
}

impl SecureServerAddr {
    /// Encrypt an address string with the runtime XOR key.
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

    /// Construct from a plain address string — equivalent to `new()`.
    ///
    /// Exists as a named alternative so call-sites make the intent clear:
    /// the value *will* be encrypted on the heap, the caller just happens
    /// to start with a plaintext literal (e.g. from the embedded config).
    pub fn from_plain(addr: &str) -> Self {
        Self::new(addr)
    }

    /// Decrypt and return the server address.
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
