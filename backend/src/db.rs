use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{auth::generate_jwt, errors::MyError, models::User};

pub async fn register_user(
    client: &deadpool_postgres::Client,
    user_info: User,
) -> Result<String, MyError> {
    let _stmt = include_str!("sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let hashed_password = bcrypt::hash(&user_info.password, bcrypt::DEFAULT_COST).unwrap();

    let user = client
        .query(
            &stmt,
            &[&user_info.email, &hashed_password, &user_info.username],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    Ok(generate_jwt(&user))
}

pub async fn login_user(
    client: &deadpool_postgres::Client,
    email: &str,
    password: &str,
) -> Result<String, MyError> {
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
        .ok_or(MyError::Unauthorized)?;

    if bcrypt::verify(password, &result.password).unwrap() {
        Ok(generate_jwt(&result))
    } else {
        Err(MyError::NotFound)
    }
}

pub async fn purchase_points(
    client: &deadpool_postgres::Client,
    email: &str,
    amount: i64,
) -> Result<i64, MyError> {
    let stmt = include_str!("sql/get_user_by_email.sql");
    let stmt = stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();

    let mut user = client
        .query(&stmt, &[&email])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    user.points += amount * 100;

    let update_stmt = "UPDATE bidding.users SET points = $1 WHERE email = $2";
    let update_stmt = client.prepare(update_stmt).await.unwrap();

    client
        .execute(&update_stmt, &[&user.points, &user.email])
        .await?;

    Ok(user.points)
}
