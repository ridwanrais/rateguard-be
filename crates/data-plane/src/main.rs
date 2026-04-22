use rateguard_data_plane::cache::config_store::ConfigStore;
use rateguard_data_plane::client::control_plane::ControlPlaneClient;
use rateguard_data_plane::limiter::redis::RedisLimiter;
use rateguard_data_plane::polling::refresher;
use rateguard_data_plane::stats::collector::StatsCollector;
use rateguard_data_plane::AppState;
use rateguard_data_plane::app;
use rateguard_data_plane::config::DataPlaneConfig;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<DataPlaneConfig>().expect("Failed to parse configuration from environment");

    println!("Connecting to Redis at {}...", config.redis_url);
    let redis_client = redis::Client::open(config.redis_url.clone()).expect("Invalid Redis URL");
    
    let cache = ConfigStore::new();
    let stats = Arc::new(StatsCollector::new());
    let limiter = Arc::new(RedisLimiter::new(redis_client));
    
    // Initial fetch from control plane
    let cp_client = ControlPlaneClient::new(config.cp_url.clone());
    if let Ok(initial_config) = cp_client.fetch_config().await {
        cache.update(initial_config);
        println!("Initial configuration loaded.");
    } else {
        println!("Warning: Could not fetch initial config from control plane.");
    }

    // Start background refresher
    refresher::start_refresher(cp_client, cache.clone(), config.poll_interval_secs).await;

    let state = AppState {
        cache,
        stats,
        limiter,
    };

    app::run_server(state, config.port).await
}
