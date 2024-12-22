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
            points: 0,
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
            points: 0,
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
        let token = generate_jwt(&User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            points: 0,
        });
        let new_token = refresh_jwt(&token).unwrap();
        assert!(validate_jwt(&new_token));
    }

    #[test]
    fn test_generate_jwt() {
        let user = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            points: 0,
        };
        let token = generate_jwt(&user);
        assert!(validate_jwt(&token));
    }

    #[test]
    fn test_validate_jwt() {
        let user = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            points: 0,
        };
        let token = generate_jwt(&user);
        assert!(validate_jwt(&token));
    }

    #[test]
    fn test_refresh_jwt() {
        let user = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            points: 0,
        };
        let token = generate_jwt(&user);
        let new_token = refresh_jwt(&token).unwrap();
        assert!(validate_jwt(&new_token));
    }

    #[test]
    fn test_service_config_default() {
        use crate::config::ServiceConfig;
        let config = ServiceConfig::default();
        assert_eq!(config.server_addr, "[::1]:8080");
    }

    #[test]
    fn test_service_config_from_env() {
        use crate::config::ServiceConfig;
        std::env::set_var("SERVER_ADDR", "127.0.0.1:8080");
        let config = ServiceConfig::builder()
            .override_with(confik::EnvSource::new())
            .try_build()
            .unwrap();
        assert_eq!(config.server_addr, "127.0.0.1:8080");
    }

    #[actix_rt::test]
    async fn test_register_user_db() {
        use crate::db::register_user;
        let mut mock_client = MockClient::new();
        let user_info = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            points: 0,
        };

        mock_client
            .expect_query()
            .with(predicate::eq("INSERT INTO bidding.users(email, password, username) VALUES ($1, $2, $3) RETURNING *"))
            .returning(|_, _| Ok(vec![]));

        let result = register_user(&mock_client, user_info).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_login_user_db() {
        use crate::db::login_user;
        let mut mock_client = MockClient::new();
        let user_info = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "password".to_string(),
            points: 0,
        };

        mock_client
            .expect_query()
            .with(predicate::eq("SELECT * FROM bidding.users WHERE email = $1"))
            .returning(|_, _| Ok(vec![user_info.clone()]));

        let result = login_user(&mock_client, &user_info.email, &user_info.password).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_create_table() {
        let mut mock_client = MockClient::new();

        mock_client
            .expect_query()
            .with(predicate::eq("CREATE TABLE test_table (id SERIAL PRIMARY KEY, name VARCHAR(100) NOT NULL)"))
            .returning(|_, _| Ok(vec![]));

        let result = mock_client.query("CREATE TABLE test_table (id SERIAL PRIMARY KEY, name VARCHAR(100) NOT NULL)", &[]).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_insert_into_table() {
        let mut mock_client = MockClient::new();

        mock_client
            .expect_query()
            .with(predicate::eq("INSERT INTO test_table (name) VALUES ($1)"))
            .returning(|_, _| Ok(vec![]));

        let result = mock_client.query("INSERT INTO test_table (name) VALUES ($1)", &[&"test_name"]).await;
        assert!(result.is_ok());
    }

    #[actix_rt::test]
    async fn test_select_from_table() {
        let mut mock_client = MockClient::new();

        mock_client
            .expect_query()
            .with(predicate::eq("SELECT * FROM test_table WHERE name = $1"))
            .returning(|_, _| Ok(vec![]));

        let result = mock_client.query("SELECT * FROM test_table WHERE name = $1", &[&"test_name"]).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_jwt_table() {
        let test_cases = vec![
            ("test@example.com", "testuser", "password", 0),
            ("another@example.com", "anotheruser", "anotherpassword", 0),
        ];

        for (email, username, password, points) in test_cases {
            let user = User {
                email: email.to_string(),
                username: username.to_string(),
                password: password.to_string(),
                points: points,
            };
            let token = generate_jwt(&user);
            assert!(validate_jwt(&token));
        }
    }

    #[test]
    fn test_validate_jwt_table() {
        let test_cases = vec![
            ("test@example.com", "testuser", "password", 0),
            ("another@example.com", "anotheruser", "anotherpassword", 0),
        ];

        for (email, username, password, points) in test_cases {
            let user = User {
                email: email.to_string(),
                username: username.to_string(),
                password: password.to_string(),
                points: points,
            };
            let token = generate_jwt(&user);
            assert!(validate_jwt(&token));
        }
    }

    #[test]
    fn test_refresh_jwt_table() {
        let test_cases = vec![
            ("test@example.com", "testuser", "password", 0),
            ("another@example.com", "anotheruser", "anotherpassword", 0),
        ];

        for (email, username, password, points) in test_cases {
            let user = User {
                email: email.to_string(),
                username: username.to_string(),
                password: password.to_string(),
                points: points,
            };
            let token = generate_jwt(&user);
            let new_token = refresh_jwt(&token).unwrap();
            assert!(validate_jwt(&new_token));
        }
    }

    #[test]
    fn test_service_config_default_table() {
        let test_cases = vec![
            ("[::1]:8080"),
            ("127.0.0.1:8080"),
        ];

        for server_addr in test_cases {
            let config = ServiceConfig {
                server_addr: server_addr.to_string(),
                ..Default::default()
            };
            assert_eq!(config.server_addr, server_addr);
        }
    }

    #[test]
    fn test_service_config_from_env_table() {
        let test_cases = vec![
            ("SERVER_ADDR", "127.0.0.1:8080"),
            ("SERVER_ADDR", "[::1]:8080"),
        ];

        for (env_var, server_addr) in test_cases {
            std::env::set_var(env_var, server_addr);
            let config = ServiceConfig::builder()
                .override_with(confik::EnvSource::new())
                .try_build()
                .unwrap();
            assert_eq!(config.server_addr, server_addr);
        }
    }
}
