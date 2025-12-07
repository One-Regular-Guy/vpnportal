

use crate::models::user_entity::Login;
use crate::services;
use pasetors::keys::AsymmetricKeyPair;
use pasetors::version4::V4;
use tracing::{debug, info};
use std::sync::Arc;
use poem_openapi::{OpenApi, payload::Json, Object, ApiResponse};

pub struct TestAPI{
    key: Arc<AsymmetricKeyPair::<V4>>,
}


#[OpenApi]
impl TestAPI {
    pub fn new(key: AsymmetricKeyPair::<V4>) -> Self {
        Self { key: Arc::new(key) }
    }
    #[oai(path = "/login", method = "post")]
    async fn add_test(&self, request: Json<Login>) -> TestResponse {
        debug!("Login: Received Login Request");
        let kp = &*self.key.clone();
        debug!("Login: Retrieved Key Pair");
        let login = request.0;
        debug!("Login: Retrieved Deserialized Login Info");
        debug!("Login: Trying authentication");
        let (auth_status, pub_token) = services::user_service::execute_login(login, kp.to_owned());
        if auth_status {
            TestResponse::Ok(
                Json(
                    LoginResponse {
                        token: pub_token,
                    }
                )
            )
        }else {
            return TestResponse::NotFound;
        }
    }
}

#[derive(serde::Serialize, Object)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(ApiResponse)]
pub enum TestResponse {
    #[oai(status = 200)]
    Ok(Json<LoginResponse>),
    #[oai(status = 400)]
    NotFound,
}