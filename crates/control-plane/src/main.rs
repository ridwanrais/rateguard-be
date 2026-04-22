use rateguard_control_plane::app;
use rateguard_control_plane::db::pool;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/rateguard".to_string());

    println!("Connecting to database...");
    let pool = pool::init_pool(&database_url).await.expect("Failed to connect to database");

    println!("Initializing DB schema...");
    init_db(&pool).await.expect("Failed to initialize DB");

    app::run_server(pool).await
}

async fn init_db(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            email VARCHAR(255) UNIQUE NOT NULL
        );"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tiers (
            name VARCHAR(50) PRIMARY KEY,
            limit_val BIGINT NOT NULL,
            window_seconds BIGINT NOT NULL
        );"
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS api_keys (
            id UUID PRIMARY KEY,
            key VARCHAR(255) UNIQUE NOT NULL,
            user_id UUID NOT NULL REFERENCES users(id),
            tier VARCHAR(50) NOT NULL REFERENCES tiers(name)
        );"
    )
    .execute(pool)
    .await?;

    // Create a mock user
    let mock_user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let _ = sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(mock_user_id)
        .bind("test@example.com")
        .execute(pool)
        .await;

    Ok(())
}
