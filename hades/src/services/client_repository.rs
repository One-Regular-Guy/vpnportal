
use sqlx::PgPool;

use crate::models::{client_entity::Client, client_repository::Repository};


pub struct ClientSqlxPGRepository;

impl ClientSqlxPGRepository {
    pub fn new() -> Self{
        ClientSqlxPGRepository
    } 
}
#[async_trait::async_trait]
impl Repository for ClientSqlxPGRepository {
    
    async fn get_client_by_cn(&self, pool: &PgPool, client_cn: String) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(Client, r#"SELECT * FROM "clients" WHERE cn = $1"#, client_cn)
            .fetch_one(pool)
            .await?;
        Ok(client)
    }
    async fn get_client_by_email(&self, pool: &PgPool, client_email: String) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(Client, r#"SELECT * FROM "clients" WHERE mail = $1"#, client_email)
            .fetch_one(pool)
            .await?;
        Ok(client)
    }
    async fn get_client_by_sn(&self, pool: &PgPool, client_sn: String) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(Client, r#"SELECT * FROM "clients" WHERE sn = $1"#, client_sn)
            .fetch_one(pool)
            .await?;
        Ok(client)
    }
    async fn get_client_by_uid(&self, pool: &PgPool, client_uid: String) -> Result<Client, sqlx::Error> {
        let client = sqlx::query_as!(Client, r#"SELECT * FROM "clients" WHERE uid = $1"#, client_uid)
            .fetch_one(pool)
            .await?;
        Ok(client)
    }
    async fn create_client(&self, pool: &PgPool, client: Client) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(
            r#"INSERT INTO "clients" (cn, ou, dc1, dc2, mail, sn, uid, password, is_ldap, is_active) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
            client.cn(),
            client.ou(),
            client.dc1(),
            client.dc2(),
            client.mail(),
            client.sn(),
            client.uid(),
            client.password(),
            client.is_ldap(),
            client.is_active()
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    async fn delete_client_by_cn(&self, pool: &PgPool, client_cn: String) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(r#"DELETE FROM "clients" WHERE cn = $1"#,
            client_cn,
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    async fn deactivate_client_by_cn(&self, pool: &PgPool, client_cn: String, is_active: bool) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(r#"UPDATE "clients" SET is_active = $1 WHERE cn = $2"#, 
            is_active,
            client_cn
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    async fn update_client_cert_by_cn(&self, pool: &PgPool, client_cn: String, client_cert: String, client_actual_download_id: String) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(r#"UPDATE "clients" SET certificate = $1, actual_download_id = $2 WHERE cn = $3"#, 
            client_cert,
            client_actual_download_id,
            client_cn
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    async fn update_client_totp_secret_by_cn(&self, pool: &PgPool, client_cn: String, client_totp_secret: String, client_actual_download_id: String) -> Result<(), sqlx::Error> {
        let _rows_affected = sqlx::query!(r#"UPDATE "clients" SET totp_secret = $1, actual_download_id = $2 WHERE cn = $3"#, 
            client_totp_secret,
            client_actual_download_id,
            client_cn
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}