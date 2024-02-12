mod config;
mod db;
mod dtos;
mod error;
mod models;
mod scopes;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.db_url)
        .await?;

    let db_client = DBClient::new(pool);

    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    println!("Server is running on {}:{}", config.url, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(scopes::posts::posts_scope())
    })
    .bind((config.host_ip, config.port))?
    .run()
    .await?;

    Ok(())
}
