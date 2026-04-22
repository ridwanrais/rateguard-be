use redis::Client;

pub struct RedisLimiter {
    client: Client,
}

impl RedisLimiter {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn check_limit(
        &self,
        api_key: &str,
        _limit: i64,
        window: i64,
    ) -> Result<(i64, u64), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
        let window_u64 = window as u64;
        let current_window = now / window_u64;
        let reset_time = (current_window + 1) * window_u64;
        
        let redis_key = format!("rate_limit:{}:{}", api_key, current_window);
        
        let (count, _): (i64, i64) = redis::pipe()
            .atomic()
            .incr(&redis_key, 1)
            .expire(&redis_key, window)
            .query_async(&mut conn)
            .await?;
            
        Ok((count, reset_time))
    }
}
