use sqlx::{Pool, Postgres};
use rateguard_core::models::api_key::ApiKey;
use crate::db::api_keys;
use uuid::Uuid;

pub async fn generate_api_key(pool: &Pool<Postgres>, user_id: Uuid, tier: String) -> Result<ApiKey, sqlx::Error> {
    let id = Uuid::new_v4();
    let key = format!("rg_{}", Uuid::new_v4().to_string().replace("-", ""));
    
    let api_key = ApiKey {
        id,
        key,
        user_id,
        tier,
    };

    api_keys::insert_api_key(pool, &api_key).await?;
    Ok(api_key)
}

pub async fn get_all_api_keys(pool: &Pool<Postgres>) -> Result<Vec<ApiKey>, sqlx::Error> {
    api_keys::list_api_keys(pool).await
}
