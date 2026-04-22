CREATE TABLE tiers (
    name VARCHAR(50) PRIMARY KEY,
    limit_val BIGINT NOT NULL,
    window_seconds BIGINT NOT NULL
);
