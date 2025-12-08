

use crate::models::user_entity::{Login, Register};
use crate::services;
use pasetors::keys::AsymmetricKeyPair;
use pasetors::version4::V4;
use sqlx::PgPool;
use tracing::debug;
use std::sync::Arc;
use poem_openapi::{OpenApi, payload::Json, Object, ApiResponse};

pub struct TestAPI{
    key: Arc<AsymmetricKeyPair::<V4>>,
    pool: PgPool, 
}


#[OpenApi]
impl TestAPI {
    pub fn new(key: AsymmetricKeyPair::<V4>, pool: PgPool) -> Self {
        Self { key: Arc::new(key), pool }
    }
    #[oai(path = "/login", method = "post")]
    async fn try_creds(&self, request: Json<Login>) -> LoginResponsePreview {
        debug!("Login: Received Login Request");
        let kp = &*self.key.clone();
        debug!("Login: Retrieved Key Pair");
        let login = request.0;
        debug!("Login: Retrieved Deserialized Login Info");
        debug!("Login: Trying authentication");
        let (auth_status, pub_token) = services::login_service::execute_login(login, kp.to_owned(), &self.pool.clone()).await;
        if auth_status {
            LoginResponsePreview::Ok(
                Json(
                    LoginResponse {
                        token: pub_token,
                    }
                )
            )
        }else {
            return LoginResponsePreview::NotFound;
        }
    }
    #[oai(path = "/register", method = "post")]
    async fn try_register(&self, request: Json<Register>) -> RegisterResponsePreview {
        debug!("Login: Received Login Request");
        let register = request.0;
        debug!("Login: Retrieved Deserialized Login Info");
        debug!("Login: Trying authentication");
        let auth_status = services::login_service::execute_register(register, &self.pool.clone()).await;
        if auth_status {
            RegisterResponsePreview::Created
        }else {
            return RegisterResponsePreview::NotAuthorized;
        }
    }
}

#[derive(serde::Serialize, Object)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(ApiResponse)]
pub enum LoginResponsePreview {
    #[oai(status = 200)]
    Ok(Json<LoginResponse>),
    #[oai(status = 400)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum RegisterResponsePreview {
    #[oai(status = 201)]
    Created,
    #[oai(status = 401)]
    NotAuthorized,
}