use serde::Deserialize;

fn default_port() -> u16 { 3001 }

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    #[serde(default = "default_port")]
    pub port: u16,
}
