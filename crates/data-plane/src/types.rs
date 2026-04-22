// Core types used across the data plane
pub type RateLimitResult = Result<(i64, u64), Box<dyn std::error::Error + Send + Sync>>;
pub type ConfigFetchResult = Result<crate::cache::config_store::AppConfig, Box<dyn std::error::Error + Send + Sync>>;
