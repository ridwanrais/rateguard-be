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
RateGuard includes a comprehensive native Rust integration test suite that verifies end-to-end rate limiting scenarios, including global limits and route-specific overrides.

### Prerequisites
1. Ensure **Postgres** and **Redis** are running (via `docker-compose up -d`).
2. Start the **Control Plane**: `cargo run --bin rateguard-control-plane`.
3. Start the **Data Plane**: `cargo run --bin rateguard-data-plane`.

### Running Tests
Run the following command from the root directory:
```bash
cargo test -p rateguard-integration-tests -- --nocapture
```
*Note: The test suite automatically cleans up the database before running, so you don't need to manually wipe your tables.*
