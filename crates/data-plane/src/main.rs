
use rateguard_data_plane::cache::config_store::ConfigStore;
use rateguard_data_plane::client::control_plane::ControlPlaneClient;
use rateguard_data_plane::limiter::redis::RedisLimiter;
use rateguard_data_plane::polling::refresher;
use rateguard_data_plane::stats::collector::StatsCollector;
use rateguard_data_plane::AppState;
use rateguard_data_plane::app;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let cp_url = std::env::var("CP_URL").unwrap_or_else(|_| "http://localhost:3001".to_string());

    println!("Connecting to Redis at {}...", redis_url);
    let redis_client = redis::Client::open(redis_url).unwrap();
    
    let cache = ConfigStore::new();
    let stats = Arc::new(StatsCollector::new());
    let limiter = Arc::new(RedisLimiter::new(redis_client));
    
    // Initial fetch from control plane
    let cp_client = ControlPlaneClient::new(cp_url);
    if let Ok(config) = cp_client.fetch_config().await {
        cache.update(config);
        println!("Initial configuration loaded.");
    } else {
        println!("Warning: Could not fetch initial config from control plane.");
    }

    // Start background refresher
    refresher::start_refresher(cp_client, cache.clone()).await;

    let state = AppState {
        cache,
        stats,
        limiter,
    };

    app::run_server(state).await
}
