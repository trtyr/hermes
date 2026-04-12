//! Plugin Registry - service registration for microkernel

use std::collections::HashMap;
use std::sync::Arc;

/// Plugin trait - all services implement this
pub trait Plugin: Send + Sync {
    fn name(&self) -> &'static str;
}

/// Plugin registry - maps names to services
pub struct PluginRegistry {
    plugins: HashMap<String, Arc<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register<P: Plugin + 'static>(&mut self, name: &'static str, plugin: Arc<P>) {
        self.plugins.insert(name.to_string(), plugin);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn Plugin>> {
        self.plugins.get(name).cloned()
    }

    pub fn names(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
