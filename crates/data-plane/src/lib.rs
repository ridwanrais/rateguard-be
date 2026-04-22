use std::sync::Arc;
pub mod app;
pub mod cache;
pub mod client;
pub mod limiter;
pub mod middleware;
pub mod polling;
pub mod stats;
pub mod types;

#[derive(Clone)]
pub struct AppState {
    pub cache: cache::config_store::ConfigStore,
    pub stats: Arc<stats::collector::StatsCollector>,
    pub limiter: Arc<limiter::redis::RedisLimiter>,
}
