use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rateguard_core::models::api_key::ApiKey;
use rateguard_core::models::tier::Tier;

#[derive(Clone, Default, Debug)]
pub struct AppConfig {
    pub tiers: HashMap<String, Tier>,
    pub api_keys: HashMap<String, ApiKey>,
}

#[derive(Clone)]
pub struct ConfigStore {
    inner: Arc<RwLock<AppConfig>>,
}

impl ConfigStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(AppConfig::default())),
        }
    }

    pub fn update(&self, config: AppConfig) {
        let mut w = self.inner.write().expect("Failed to acquire write lock");
        *w = config;
    }

    pub fn get_api_key(&self, key: &str) -> Option<ApiKey> {
        self.inner.read().expect("Failed to acquire read lock").api_keys.get(key).cloned()
    }

    pub fn get_tier(&self, name: &str) -> Option<Tier> {
        self.inner.read().expect("Failed to acquire read lock").tiers.get(name).cloned()
    }
}
