use std::env;

use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use confik::{Configuration as _, EnvSource};
use deadpool_postgres::{Client, Pool};
use dotenvy::dotenv;
use env_logger::Env;
use serde_json::json;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::ServiceConfig;

mod auth;
mod config;
mod db;
mod errors;
mod models;

use self::{errors::MyError, models::User};

#[utoipa::path(
    post,
    path = "/register",
    request_body = User,
    responses((status = 200, description = "Register a new user", body = String))
)]
pub async fn register_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let token = db::register_user(&client, user_info).await?;
    Ok(HttpResponse::Ok().json(token))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = User,
    responses((status = 200, description = "Login a user", body = String))
)]
pub async fn login_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let token = db::login_user(&client, &user_info.email, &user_info.password).await?;
    Ok(HttpResponse::Ok().json(token))
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = String,
    responses((status = 200, description = "Refresh JWT token", body = String))
)]
pub async fn refresh_token(token: web::Json<String>) -> Result<HttpResponse, Error> {
    let new_token = auth::refresh_jwt(&token).ok_or(MyError::NotFound)?;
    Ok(HttpResponse::Ok().json(new_token))
}

#[utoipa::path(
    post,
    path = "/purchase",
    request_body = f64,
    responses(
        (status = 200, description = "Purchase points", body = String),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn purchase_points(
    amount: web::Json<f64>,
    db_pool: web::Data<Pool>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(MyError::NotFound)?
        .to_str()
        .map_err(|_| MyError::NotFound)?;

    if !auth::validate_jwt(token) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let email = auth::extract_email_from_jwt(token).ok_or(MyError::NotFound)?;
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let updated_points = db::purchase_points(&client, &email, *amount).await?;
    Ok(HttpResponse::Ok().json(json!({
        "success": true,
        "message": "Points purchased successfully",
        "points": updated_points
    })))
}

#[derive(OpenApi)]
#[openapi(
    paths(register_user, login_user, refresh_token, purchase_points),
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
            .service(web::resource("/register").route(web::post().to(register_user)))
            .service(web::resource("/login").route(web::post().to(login_user)))
            .service(web::resource("/refresh").route(web::post().to(refresh_token)))
            .service(web::resource("/purchase").route(web::post().to(purchase_points)))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiDoc::openapi()))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
