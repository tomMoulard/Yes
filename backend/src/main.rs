use std::env;

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

mod config;
mod db;
mod errors;
mod models;

use self::{errors::MyError, models::User};

#[utoipa::path(
    get,
    path = "/users/v1",
    responses((status = 200, description = "List all users", body = [User]))
)]
pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let users = db::get_users(&client).await?;
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
    post,
    path = "/users/v1",
    request_body = User,
    responses((status = 200, description = "Add a new user", body = User))
)]
pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = db::add_user(&client, user_info).await?;
    Ok(HttpResponse::Ok().json(new_user))
}

#[derive(OpenApi)]
#[openapi(
    paths(get_users, add_user),
    components(schemas(User)),
    tags((name = "user", description = "User management endpoints"))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "generate-openapi" {
        println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
        return Ok(());
    }

    dotenv().ok();
    let config = ServiceConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .service(
                web::resource("/users/v1")
                    .route(web::post().to(add_user))
                    .route(web::get().to(get_users)),
            )
            .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiDoc::openapi()))
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
// - OpenAPI
