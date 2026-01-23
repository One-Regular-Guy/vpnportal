use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("database error ")]
    DatabaseError(#[from] sqlx::Error),
    #[error("error when generating token")]
    TokenError(#[from] pasetors::errors::Error),
    #[error("unknown error")]
    Unknown,
}