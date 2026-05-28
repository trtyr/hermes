//! Plugin trait - service identity interface

/// Plugin trait - all services implement this.
///
/// Used by the kernel to identify services; the agent runtime uses concrete
/// types directly rather than a registry, keeping the binary minimal.
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
}
