use sqlx::{Pool, Postgres};
use rateguard_core::models::tier::Tier;
use crate::db::tiers;

pub async fn create_tier(pool: &Pool<Postgres>, tier: Tier) -> Result<(), sqlx::Error> {
    // Business logic/validation could go here
    tiers::insert_tier(pool, &tier).await
}

pub async fn get_all_tiers(pool: &Pool<Postgres>) -> Result<Vec<Tier>, sqlx::Error> {
    tiers::list_tiers(pool).await
}
