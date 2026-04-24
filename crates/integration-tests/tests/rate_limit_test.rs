use rateguard_core::models::tier::{Tier, RouteLimit};
use rateguard_core::models::api_key::ApiKey;
use reqwest::StatusCode;
use std::time::Duration;
use sqlx::{Pool, Postgres};
use tracing::{info, warn, debug};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

const CP_URL: &str = "http://localhost:3001";
const DP_URL: &str = "http://localhost:3000";

fn setup_tracing() {
    let _ = tracing_subscriber::registry()
        .with(fmt::layer().with_test_writer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .try_init();
}

async fn setup_db() -> Pool<Postgres> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<Postgres>::connect(&db_url).await.expect("Failed to connect to DB");
    
    info!("Cleaning up database tables...");
    sqlx::query("DELETE FROM api_keys").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM tiers").execute(&pool).await.unwrap();
    info!("Database cleanup complete.");
    
    pool
}

async fn ensure_services_running() {
    let client = reqwest::Client::new();
    
    debug!("Checking Control Plane at {}...", CP_URL);
    let cp_check = client.get(format!("{}/tiers", CP_URL)).send().await;
    if cp_check.is_err() {
        panic!("FATAL: Control Plane is not running at {}. Please start it before running tests.", CP_URL);
    }
    
    debug!("Checking Data Plane at {}...", DP_URL);
    let dp_check = client.get(DP_URL).send().await;
    if dp_check.is_err() {
        panic!("FATAL: Data Plane is not running at {}. Please start it before running tests.", DP_URL);
    }
}

#[tokio::test]
async fn test_rate_limiting_scenarios() {
    setup_tracing();
    info!("Starting integration test: test_rate_limiting_scenarios");

    ensure_services_running().await;
    let _pool = setup_db().await;
    let client = reqwest::Client::new();

    // 1. Create a tier with overrides
    let tier_name = "integration_test_tier";
    let tier = Tier {
        name: tier_name.to_string(),
        limit: 5,
        window_seconds: 60,
        route_overrides: Some(vec![
            RouteLimit {
                path: "/expensive".to_string(),
                limit: 2,
                window_seconds: 60,
            }
        ]),
    };

    info!("Creating tier: {} (Limit: 5, Override /expensive: 2)", tier_name);
    let res = client.post(format!("{}/tiers", CP_URL))
        .json(&tier)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    // 2. Create API Key
    info!("Creating API key for tier: {}", tier_name);
    let api_key_req = serde_json::json!({
        "user_id": "00000000-0000-0000-0000-000000000001",
        "tier": tier_name
    });
    let res = client.post(format!("{}/api-keys", CP_URL))
        .json(&api_key_req)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let api_key_obj: ApiKey = res.json().await.unwrap();
    let api_key = api_key_obj.key;
    info!("API key created: {}...", &api_key[..8]);

    // 3. Wait for Data Plane to sync (poll interval is 30s)
    info!("Waiting 35s for Data Plane synchronization...");
    tokio::time::sleep(Duration::from_secs(35)).await;

    // 4. Test Global Limit (5 req)
    info!("Testing global limits (should allow 5, then block)...");
    for i in 1..=6 {
        let res = client.get(DP_URL)
            .header("x-api-key", &api_key)
            .send()
            .await
            .unwrap();
        
        let status = res.status();
        info!("Global Request {}: Status {}", i, status);
        
        if i <= 5 {
            assert_eq!(status, StatusCode::OK, "Request {} should have been OK", i);
        } else {
            assert_eq!(status, StatusCode::TOO_MANY_REQUESTS, "Request 6 should have been limited");
        }
    }

    // 5. Test Route Override (2 req)
    info!("Testing route override limits for /expensive (should allow 2, then block)...");
    for i in 1..=3 {
        let res = client.get(format!("{}/expensive", DP_URL))
            .header("x-api-key", &api_key)
            .send()
            .await
            .unwrap();
        
        let status = res.status();
        info!("Override Request {}: Status {}", i, status);
        
        if i <= 2 {
            assert_eq!(status, StatusCode::OK, "Expensive Request {} should have been OK", i);
        } else {
            assert_eq!(status, StatusCode::TOO_MANY_REQUESTS, "Expensive Request 3 should have been limited");
        }
    }
    
    info!("Integration tests passed successfully!");
}
