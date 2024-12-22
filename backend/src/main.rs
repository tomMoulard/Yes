use confik::Configuration as _;
use utoipa::OpenApi;

mod auth;
mod config;
mod db;
mod errors;
mod models;

use self::errors::MyError;

#[utoipa::path(
    post,
    path = "/register",
    request_body = self::models::User,
    responses((status = 200, description = "Register a new user", body = String))
)]
pub async fn register_user(
    user: actix_web::web::Json<self::models::User>,
    db_pool: actix_web::web::Data<deadpool_postgres::Pool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let user_info: self::models::User = user.into_inner();
    let client: deadpool_postgres::Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let token = db::register_user(&client, user_info).await?;
    Ok(actix_web::HttpResponse::Ok().json(token))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = self::models::User,
    responses((status = 200, description = "Login a user", body = String))
)]
pub async fn login_user(
    user: actix_web::web::Json<self::models::User>,
    db_pool: actix_web::web::Data<deadpool_postgres::Pool>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let user_info: self::models::User = user.into_inner();
    let client: deadpool_postgres::Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let token = db::login_user(&client, &user_info.email, &user_info.password).await?;
    Ok(actix_web::HttpResponse::Ok().json(token))
}

#[utoipa::path(
    post,
    path = "/refresh",
    security(("token" = [])),
    responses((status = 200, description = "Refresh JWT token", body = String))
)]
pub async fn refresh_token(
        req: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let token = req
        .cookie("token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
        });

    if token.is_none() {
        return Ok(actix_web::HttpResponse::Unauthorized().finish());
    }

    let new_token = auth::refresh_jwt(&token.unwrap()).ok_or(MyError::NotFound)?;
    Ok(actix_web::HttpResponse::Ok().json(new_token))
}

#[utoipa::path(
    post,
    path = "/purchase",
    request_body = self::models::PurchaseRequest,
    security(("token" = [])),
    responses(
        (status = 200, description = "Purchase points", body = String),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn purchase_points(
    amount: actix_web::web::Json<self::models::PurchaseRequest>,
    db_pool: actix_web::web::Data<deadpool_postgres::Pool>,
    req: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let token = req
        .cookie("token")
        .map(|c| c.value().to_string())
        .or_else(|| {
            req.headers()
                .get(actix_web::http::header::AUTHORIZATION)
                .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
        });

    if token.is_none() {
        return Ok(actix_web::HttpResponse::Unauthorized().finish());
    }

    let email = auth::extract_email_from_jwt(&token.unwrap()).ok_or(MyError::NotFound)?;
    let client: deadpool_postgres::Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let updated_points = db::purchase_points(&client, &email, amount.points).await?;
    Ok(actix_web::HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Points purchased successfully",
        "points": updated_points
    })))
}

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(register_user, login_user, refresh_token, purchase_points),
    components(schemas(self::models::User)),
    tags((name = "user", description = "self::models::User management endpoints")),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            utoipa::openapi::security::SecurityScheme::Http(
                utoipa::openapi::security::HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        )
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "generate-openapi" {
        println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
        return Ok(());
    }

    dotenvy::dotenv().ok();
    let config = crate::config::ServiceConfig::builder()
        .override_with(confik::EnvSource::new())
        .try_build()
        .unwrap();
    let pool = config.pg.create_pool(None, tokio_postgres::NoTls).unwrap();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .service(
                actix_web::web::resource("/register")
                    .route(actix_web::web::post().to(register_user)),
            )
            .service(
                actix_web::web::resource("/login").route(actix_web::web::post().to(login_user)),
            )
            .service(
                actix_web::web::resource("/refresh")
                    .route(actix_web::web::post().to(refresh_token)),
            )
            .service(
                actix_web::web::resource("/purchase")
                    .route(actix_web::web::post().to(purchase_points)),
            )
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/docs/{_:.*}")
                    .url("/docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
