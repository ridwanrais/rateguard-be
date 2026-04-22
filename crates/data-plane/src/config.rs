use serde::Deserialize;

fn default_port() -> u16 { 3000 }
fn default_cp_url() -> String { "http://localhost:3001".to_string() }
fn default_poll_interval() -> u64 { 30 }

#[derive(Debug, Clone, Deserialize)]
pub struct DataPlaneConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    
    pub redis_url: String,
    
    #[serde(default = "default_cp_url")]
    pub cp_url: String,

    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
}
