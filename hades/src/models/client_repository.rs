use sqlx::PgPool;

use crate::models::client_entity::Client;

#[async_trait::async_trait]
pub trait Repository {
    async fn get_client_by_cn(&self, pool: &PgPool, client_cn: String) -> Result<Client, sqlx::Error> ;
    async fn get_client_by_email(&self, pool: &PgPool, client_email: String) -> Result<Client, sqlx::Error> ;
    async fn get_client_by_sn(&self, pool: &PgPool, client_sn: String) -> Result<Client, sqlx::Error> ;
    async fn get_client_by_uid(&self, pool: &PgPool, client_uid: String) -> Result<Client, sqlx::Error> ;
    async fn create_client(&self, pool: &PgPool, user: Client) -> Result<(), sqlx::Error> ;
    async fn delete_client_by_cn(&self, pool: &PgPool, client_cn: String) -> Result<(), sqlx::Error> ;
    async fn deactivate_client_by_cn(&self, pool: &PgPool, client_cn: String, is_active: bool) -> Result<(), sqlx::Error> ;
    async fn update_client_cert_by_cn(&self, pool: &PgPool, client_cn: String, client_cert: String, client_actual_download_id: String) -> Result<(), sqlx::Error> ;
    async fn update_client_totp_secret_by_cn(&self, pool: &PgPool, client_cn: String, client_totp_secret: String, client_actual_download_id: String) -> Result<(), sqlx::Error> ;
}