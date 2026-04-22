CREATE TABLE api_keys (
    id UUID PRIMARY KEY,
    key VARCHAR(255) UNIQUE NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    tier VARCHAR(50) NOT NULL REFERENCES tiers(name)
);
