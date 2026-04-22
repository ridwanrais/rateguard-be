# RateGuard

## Database Migrations
For this MVP, database migrations are handled automatically upon startup by the `Control Plane`. When you run `cargo run --bin rateguard-control-plane`, the application automatically connects to Postgres, creates any missing tables, and applies schema updates (such as adding the `route_overrides` column). No manual migration tool is required.

## API Documentation (Swagger)
The Control Plane serves a fully interactive Swagger UI dashboard detailing all endpoints, parameters, and models.
Once the Control Plane is running, visit: **[http://localhost:3001/swagger-ui/](http://localhost:3001/swagger-ui/)**


A developer-first rate limiting service with modular architecture.

## Architecture

- **crates/core**: Shared models and configurations.
- **crates/control-plane**: Management API (Actix + SQLx + Postgres).
- **crates/data-plane**: High-performance request path (Actix + Redis).

## Setup & Running

1. **Start Infrastructure**
   ```shell
   docker-compose -f infra/docker-compose.yml up -d
   ```

2. **Start the Control Plane**
   ```shell
   cargo run --bin rateguard-control-plane
   ```
   > Port: 3001. Automatically seeds a mock user.

3. **Start the Data Plane**
   ```shell
   cargo run --bin rateguard-data-plane
   ```
   > Port: 3000. Polls control plane every 30s.

## Testing

You can use the provided test script:
```shell
chmod +x scripts/test_requests.sh
./scripts/test_requests.sh
```

Or manually:
1. Create Tier: `POST :3001/tiers`
2. Create Key: `POST :3001/api-keys`
3. Request: `GET :3000/` with `X-API-Key` header.
