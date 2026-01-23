use sqlx::PgPool;

use crate::{models::{cachain_entity::CaChain, client_entity::{Client, RegisterLdap}, client_repository::Repository, errors::ApiError}, services::client_repository};


pub async fn execute_register_not_ldap(pool: &PgPool, incomming_client: Client) -> Result<(),ApiError> {
    let repository = client_repository::ClientSqlxPGRepository::new();
    repository.create_client(pool, incomming_client).await.map_err(|e| ApiError::DatabaseError(e))
}
pub async fn execute_register_ldap(pool: &PgPool, incomming_client_from_ldap: RegisterLdap) -> Result<(),ApiError> {
    let repository = client_repository::ClientSqlxPGRepository::new();
    let converted_client = Client::new_from_register(incomming_client_from_ldap);
    repository.create_client(pool, converted_client).await.map_err(|e| ApiError::DatabaseError(e))
}