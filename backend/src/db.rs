use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{errors::MyError, models::User, auth::generate_jwt};

pub async fn register_user(client: &Client, user_info: User) -> Result<String, MyError> {
    let _stmt = include_str!("sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let hashed_password = hash(&user_info.password, DEFAULT_COST).unwrap();

    client
        .query(&stmt, &[&user_info.email, &hashed_password, &user_info.username])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    let token = generate_jwt(&user_info);
    Ok(token)
}

pub async fn login_user(client: &Client, email: &str, password: &str) -> Result<String, MyError> {
    let stmt = include_str!("sql/get_user_by_email.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&email])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    if verify(password, &result.password).unwrap() {
        let token = generate_jwt(&result.email);
        Ok(token)
    } else {
        Err(MyError::NotFound)
    }
}
