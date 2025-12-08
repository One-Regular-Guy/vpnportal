use pasetors::{keys::{AsymmetricKeyPair, Generate}, version4::V4};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
mod models;
mod handlers;
use handlers::endpoints::TestAPI;
use sqlx::{Pool, postgres::PgPoolOptions};
use tracing_subscriber::EnvFilter;
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
    let kp = AsymmetricKeyPair::<V4>::generate().unwrap();
    let endpoints: TestAPI = TestAPI::new(kp, pool);
    let api_service = OpenApiService::new(endpoints, "Hades", "1.0").server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await.unwrap();
}