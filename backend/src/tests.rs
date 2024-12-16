#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use mockall::predicate::*;
    use mockall::*;
    use deadpool_postgres::Client;
    use crate::db::{register_user, login_user};
    use crate::auth::{generate_jwt, validate_jwt, refresh_jwt};
    use crate::models::User;
    use crate::errors::MyError;

    mock! {
        pub Client {}
        #[async_trait::async_trait]
        impl deadpool_postgres::Client for Client {
            async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error>;
        }
    }

    #[actix_rt::test]
    async fn test_register_user() {
        let mut mock_client = MockClient::new();
        let user_info = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
        };

        mock_client
            .expect_query()
            .with(predicate::eq("INSERT INTO users(email, password, username) VALUES ($1, $2, $3) RETURNING *"))
            .returning(|_, _| Ok(vec![]));

        let result = register_user(&mock_client, user_info).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_login_user() {
        let mut mock_client = MockClient::new();
        let user_info = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
        };

        mock_client
            .expect_query()
            .with(predicate::eq("SELECT * FROM users WHERE email = $1"))
            .returning(|_, _| Ok(vec![user_info.clone()]));

        let result = login_user(&mock_client, &user_info.email, &user_info.password).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_refresh_token() {
        let token = generate_jwt("test@example.com");
        let new_token = refresh_jwt(&token).unwrap();
        assert!(validate_jwt(&new_token));
    }
}
