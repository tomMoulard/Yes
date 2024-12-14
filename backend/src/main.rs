use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use env_logger::Env;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::ServiceConfig;
use crate::openapi::ApiDoc;

mod config;
mod db;
mod errors;
mod models;
mod openapi;

use self::{errors::MyError, models::User};

pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let users = db::get_users(&client).await?;
    Ok(HttpResponse::Ok().json(users))
}

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = db::add_user(&client, user_info).await?;
    Ok(HttpResponse::Ok().json(new_user))
}

#[actix_web::main]
async fn main() -> std::.io::Result<()> {
    dotenv().ok();
    let docs_mode = std::env::var("DOCS_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
    let config = ServiceConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::default())
            .service(
                web::resource("/users")
                    .route(web::post().to(add_user))
                    .route(web::get().to(get_users)),
            )
            .service(SwaggerUi::new("/api-docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

        if !docs_mode {
            let pool = config.pg.create_pool(None, NoTls).unwrap();
            app.app_data(web::Data::new(pool.clone()))
        } else {
            app
        }
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}

// TODO:
// - JWT
// - salt passwords in DB
// - login/register
