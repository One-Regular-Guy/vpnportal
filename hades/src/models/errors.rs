
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("database error ")]
    DatabaseError(#[from] sqlx::Error),
    #[error("error when generating token")]
    TokenError(#[from] pasetors::errors::Error),
    #[error("Error when Ldapping")]
    LdapError(#[from] anyhow::Error),
    #[error("unknown error")]
    Unknown,
}