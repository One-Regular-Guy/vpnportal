use std::env;

use pasetors::{keys::{AsymmetricKeyPair, Generate}, version4::V4};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
mod models;
mod handlers;
use handlers::endpoints::MainAPI;
use sqlx::{Pool, postgres::PgPoolOptions};
use tracing_subscriber::EnvFilter;

use crate::models::cachain_entity::CaChain;
mod services;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("hades=debug")) // or "debug", "trace", etc.
        // .with_env_filter("debug")                     // global debug
        .init();
    let pool: Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/hades").await.unwrap();
    let ca_cert_str = match env::var("CA_CERT") {
        Ok(ca_cert) => ca_cert,
        Err(_) => panic!("CA_CERT dont provided"), 
    };
    let ca_key_str = match env::var("CA_KEY") {
        Ok(ca_key) => ca_key,
        Err(_) => panic!("CA_KEY dont provided"), 
    };
    let ca_cert: Vec<u8> = ca_cert_str.as_bytes().to_vec();
    let ca_key: Vec<u8> = ca_key_str.as_bytes().to_vec();
    let ca_chain = CaChain::new(ca_cert, ca_key);
    let kp = AsymmetricKeyPair::<V4>::generate().unwrap();
    let endpoints: MainAPI = MainAPI::new(kp, pool, ca_chain);
    let api_service = OpenApiService::new(endpoints, "Hades", "1.0").server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await.unwrap();
}