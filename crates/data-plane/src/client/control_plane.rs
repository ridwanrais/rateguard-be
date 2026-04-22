use rateguard_core::models::api_key::ApiKey;
use rateguard_core::models::tier::Tier;
use serde::Deserialize;
use std::collections::HashMap;
use crate::cache::config_store::AppConfig;

#[derive(Deserialize)]
struct InternalConfigResponse {
    pub tiers: Vec<Tier>,
    pub api_keys: Vec<ApiKey>,
}

pub struct ControlPlaneClient {
    base_url: String,
    client: reqwest::Client,
}

impl ControlPlaneClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_config(&self) -> Result<AppConfig, Box<dyn std::error::Error + Send + Sync>> {
        let resp: InternalConfigResponse = self.client
            .get(format!("{}/internal/config", self.base_url))
            .send()
            .await?
            .json()
            .await?;

        let mut tiers = HashMap::new();
        for t in resp.tiers {
            tiers.insert(t.name.clone(), t);
        }

        let mut api_keys = HashMap::new();
        for k in resp.api_keys {
            api_keys.insert(k.key.clone(), k);
        }

        Ok(AppConfig { tiers, api_keys })
    }
}
