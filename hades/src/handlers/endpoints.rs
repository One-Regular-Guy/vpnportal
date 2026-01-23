

use crate::models::cachain_entity::CaChain;
use crate::models::errors::ApiError;
use crate::models::user_entity::{Login, Register};
use crate::services;
use pasetors::keys::AsymmetricKeyPair;
use pasetors::version4::V4;
use sqlx::PgPool;
use tracing::debug;
use std::sync::Arc;
use poem_openapi::{OpenApi, payload::Json, Object, ApiResponse};

pub struct MainAPI{
    key: Arc<AsymmetricKeyPair::<V4>>,
    pool: PgPool, 
    ca: Arc<CaChain>,
}


#[OpenApi]
impl MainAPI {
    pub fn new(key: AsymmetricKeyPair::<V4>, pool: PgPool, ca: CaChain) -> Self {
        Self { key: Arc::new(key), pool, ca: Arc::new(ca) }
    }
    #[oai(path = "/login", method = "post")]
    async fn try_creds(&self, request: Json<Login>) -> LoginResponsePreview {
        debug!("Login: Received Login Request");
        let kp = &*self.key.clone();
        debug!("Login: Retrieved Key Pair");
        let login = request.0;
        debug!("Login: Retrieved Deserialized Login Info");
        debug!("Login: Trying authentication");
        match services::login_service::execute_login(login, kp.to_owned(), &self.pool.clone()).await {
            Ok((auth_status, pub_token)) => {
                if auth_status {
                    LoginResponsePreview::Ok(
                        Json(
                            LoginResponse {
                                token: pub_token,
                            }
                        )
                    )
                }else {
                    return LoginResponsePreview::NotAuthorized;
                }
            },
            Err(e) => match e {
                _ => LoginResponsePreview::InternalServerError
            }
        }
    }
    #[oai(path = "/register", method = "post")]
    async fn try_register(&self, request: Json<Register>) -> RegisterResponsePreview {
        debug!("Register: Received Register Request");
        let register = request.0;
        debug!("Register: Retrieved Deserialized Login Info");
        debug!("Register: Trying authentication");
        match services::login_service::execute_register(register, &self.pool.clone()).await{
            Ok(()) => RegisterResponsePreview::Created,
            Err(e) => match e {
                ApiError::DatabaseError(_) => RegisterResponsePreview::InternalServerError,
                _ => RegisterResponsePreview::NotAuthorized,
            }
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
    #[oai(status = 500)]
    InternalServerError,
    #[oai(status = 401)]
    NotAuthorized,
}

#[derive(ApiResponse)]
pub enum RegisterResponsePreview {
    #[oai(status = 201)]
    Created,
    #[oai(status = 401)]
    NotAuthorized,
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(serde::Serialize, Object)]
pub struct CertifyResponse {
    pub token: String,
}
#[derive(ApiResponse)]
pub enum CertifyResponsePreview {
    #[oai(status = 200)]
    Ok(Json<LoginResponse>),
    #[oai(status = 201)]
    Created,
    #[oai(status = 401)]
    NotAuthorized,
    #[oai(status = 500)]
    InternalServerError,
}

