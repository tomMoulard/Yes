use actix_web::web;
use actix_web::{test, App};
use crate::index;
use crate::bid;

#[actix_rt::test]
async fn test_index() {
    let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_bid() {
    let mut app = test::init_service(App::new().route("/bid", web::post().to(bid))).await;
    let req = test::TestRequest::post().uri("/bid").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}
