
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{user_entity::User, user_repository::Repository};


pub struct UserSqlxPGRepository;

impl UserSqlxPGRepository {
    pub fn new() -> Self{
        UserSqlxPGRepository
    } 
}
#[async_trait::async_trait]
impl Repository for UserSqlxPGRepository {
    
    async fn get_user_by_id(&self, pool: &PgPool, user_id: Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM "users" WHERE id = $1"#, user_id)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }
    async fn get_user_by_name(&self, pool: &PgPool, user_name: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM "users" WHERE name = $1"#, user_name)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }
    async fn get_user_by_email(&self, pool: &PgPool, user_email: String) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM "users" WHERE email = $1"#, user_email)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }
    async fn create_user(&self, pool: &PgPool, user: User) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(
        r#"INSERT INTO "users" (id, name, email, password) VALUES ($1, $2, $3, $4)"#,
        user.id(),
        user.name(),
        user.email(),
        user.password()
    )
    .execute(pool)
    .await?;
    Ok(())
    }

    async fn delete_user_by_name(&self, pool: &PgPool, user_name: String) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(r#"DELETE FROM "users" WHERE name = $1"#,
            user_name,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}