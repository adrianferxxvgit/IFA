pub mod creativesdev;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Error de red: {0}")]
    NetworkError(String),
    #[error("Error de autenticación")]
    AuthError,
    #[error("Rate limit excedido")]
    RateLimitExceeded,
    #[error("Error de parseo: {0}")]
    ParseError(String),
}
