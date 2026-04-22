use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RouteLimit {
    pub path: String,
    pub limit: i64,
    pub window_seconds: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Tier {
    pub name: String,
    pub limit: i64,
    pub window_seconds: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_overrides: Option<Vec<RouteLimit>>,
}
