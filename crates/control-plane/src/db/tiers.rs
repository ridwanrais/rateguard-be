use sqlx::{Pool, Postgres, Row};
use rateguard_core::models::tier::Tier;

pub async fn insert_tier(pool: &Pool<Postgres>, tier: &Tier) -> Result<(), sqlx::Error> {
    let overrides_json = tier.route_overrides.as_ref().map(|o| serde_json::to_value(o).unwrap());

    sqlx::query("INSERT INTO tiers (name, limit_val, window_seconds, route_overrides) VALUES ($1, $2, $3, $4)")
        .bind(&tier.name)
        .bind(tier.limit)
        .bind(tier.window_seconds)
        .bind(overrides_json)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_tiers(pool: &Pool<Postgres>) -> Result<Vec<Tier>, sqlx::Error> {
    let rows = sqlx::query("SELECT name, limit_val, window_seconds, route_overrides FROM tiers")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter().map(|row| {
        let overrides_json: Option<serde_json::Value> = row.get("route_overrides");
        let route_overrides = overrides_json.and_then(|v| serde_json::from_value(v).ok());

        Tier {
            name: row.get("name"),
            limit: row.get("limit_val"),
            window_seconds: row.get("window_seconds"),
            route_overrides,
        }
    }).collect())
}
