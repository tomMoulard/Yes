use actix_web::{web, App, HttpServer, Responder};

async fn index() -> impl Responder {
    "Welcome to the Bidding App!"
}

async fn bid() -> impl Responder {
    "Bid placed successfully!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/bid", web::post().to(bid))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
