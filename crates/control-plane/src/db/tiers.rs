use sqlx::{Pool, Postgres, Row};
use rateguard_core::models::tier::Tier;

pub async fn insert_tier(pool: &Pool<Postgres>, tier: &Tier) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO tiers (name, limit_val, window_seconds) VALUES ($1, $2, $3)")
        .bind(&tier.name)
        .bind(tier.limit)
        .bind(tier.window_seconds)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_tiers(pool: &Pool<Postgres>) -> Result<Vec<Tier>, sqlx::Error> {
    let rows = sqlx::query("SELECT name, limit_val, window_seconds FROM tiers")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(|row| Tier {
        name: row.get("name"),
        limit: row.get("limit_val"),
        window_seconds: row.get("window_seconds"),
    }).collect())
}
