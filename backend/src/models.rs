use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use utoipa::ToSchema;

#[derive(Deserialize, PostgresMapper, Serialize, ToSchema)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
    pub points: i64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct RefreshRequest {
    pub token: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct PurchaseRequest {
    pub points: i64,
}
