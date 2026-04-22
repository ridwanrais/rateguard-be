use serde::{Serialize, Deserialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiKey {
    pub id: Uuid,
    pub key: String,
    pub user_id: Uuid,
    pub tier: String,
}
