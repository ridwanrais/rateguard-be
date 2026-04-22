use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RateGuardError {
    Internal(String),
    NotFound(String),
    Unauthorized,
    RateLimited {
        retry_in: String,
    },
}

impl std::fmt::Display for RateGuardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Internal(s) => write!(f, "Internal error: {}", s),
            Self::NotFound(s) => write!(f, "Not found: {}", s),
            Self::Unauthorized => write!(f, "Unauthorized"),
            Self::RateLimited { retry_in } => write!(f, "Rate limited, retry in {}", retry_in),
        }
    }
}
