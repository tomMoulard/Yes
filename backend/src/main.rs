use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use tokio_postgres::{NoTls, Client};
use tokio::signal;

const SECRET: &[u8] = b"your_secret_key";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn index() -> impl Responder {
    "Welcome to the Bidding App!"
}

pub async fn bid() -> impl Responder {
    "Bid placed successfully!"
}

pub async fn register(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let stmt = client.prepare("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)").await.unwrap();
    client.execute(&stmt, &[&user.username, &user.email, &hashed_password]).await.unwrap();
    let token = generate_jwt(&user.username);
    HttpResponse::Ok().json(token)
}

pub async fn login(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let stmt = client.prepare("SELECT password FROM users WHERE username = $1").await.unwrap();
    let rows = client.query(&stmt, &[&user.username]).await.unwrap();
    if rows.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }
    let stored_password: &str = rows[0].get(0);
    if verify(&user.password, stored_password).unwrap() {
        let token = generate_jwt(&user.username);
        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

pub fn generate_jwt(username: &str) -> String {
    let expiration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn verify_jwt(token: &str) -> bool {
    let validation = Validation::default();
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &validation).is_ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (client, connection) = tokio_postgres::connect("host=my-rds-proxy-endpoint user=foo password=bar dbname=mydb", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/", web::get().to(index))
            .route("/bid", web::post().to(bid))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run();

    let graceful = server.clone();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for ctrl_c signal");
        graceful.stop(true).await;
    });

    server.await
}
