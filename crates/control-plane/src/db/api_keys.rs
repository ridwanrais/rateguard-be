use sqlx::{Pool, Postgres, Row};
use rateguard_core::models::api_key::ApiKey;


pub async fn insert_api_key(pool: &Pool<Postgres>, api_key: &ApiKey) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO api_keys (id, key, user_id, tier) VALUES ($1, $2, $3, $4)")
        .bind(api_key.id)
        .bind(&api_key.key)
        .bind(api_key.user_id)
        .bind(&api_key.tier)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_api_keys(pool: &Pool<Postgres>) -> Result<Vec<ApiKey>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, key, user_id, tier FROM api_keys")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(|row| ApiKey {
        id: row.get("id"),
        key: row.get("key"),
        user_id: row.get("user_id"),
        tier: row.get("tier"),
    }).collect())
}
