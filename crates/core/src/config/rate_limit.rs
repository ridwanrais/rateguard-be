use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RateLimit {
    pub max_requests: u64,
    pub window_seconds: u64,
}
