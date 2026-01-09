use sqlx::PgPool;
use uuid::Uuid;

use crate::models::user_entity::User;

#[async_trait::async_trait]
pub trait Repository {
    async fn get_user_by_id(&self, pool: &PgPool, user_id: Uuid) -> Result<User, sqlx::Error> ;
    async fn get_user_by_name(&self, pool: &PgPool, user_name: String) -> Result<User, sqlx::Error> ;
    async fn get_user_by_email(&self, pool: &PgPool, user_email: String) -> Result<User, sqlx::Error> ;
    async fn create_user(&self, pool: &PgPool, user: User) -> Result<(), sqlx::Error> ;
    async fn delete_user_by_name(&self, pool: &PgPool, user_name: String) -> Result<(), sqlx::Error> ;
}