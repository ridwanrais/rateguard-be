use crate::client::control_plane::ControlPlaneClient;
use crate::cache::config_store::ConfigStore;
use std::time::Duration;
use rand::Rng;

pub async fn start_refresher(client: ControlPlaneClient, store: ConfigStore) {
    tokio::spawn(async move {
        loop {
            match client.fetch_config().await {
                Ok(config) => {
                    store.update(config);
                    println!("Successfully updated configuration from control plane");
                }
                Err(e) => {
                    println!("Failed to update configuration: {}. Retrying later.", e);
                }
            }

            let jitter = rand::thread_rng().gen_range(0..5);
            tokio::time::sleep(Duration::from_secs(30 + jitter)).await;
        }
    });
}
