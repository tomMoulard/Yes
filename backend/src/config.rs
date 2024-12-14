use confik::Configuration;
use serde::Deserialize;

#[derive(Debug, Default, Configuration)]
pub struct ServiceConfig {
    #[confik(default = "[::1]:8080")]
    pub server_addr: String,
    #[confik(from = DbConfig)]
    pub pg: deadpool_postgres::Config,
    #[confik(default = "fake_pg")]
    pub fake_pg: deadpool_postgres::Config,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
struct DbConfig(deadpool_postgres::Config);

impl From<DbConfig> for deadpool_postgres::Config {
    fn from(value: DbConfig) -> Self {
        value.0
    }
}

impl confik::Configuration for DbConfig {
    type Builder = Option<Self>;
}

fn fake_pg() -> deadpool_postgres::Config {
    deadpool_postgres::Config {
        user: Some("fake_user".to_string()),
        password: Some("fake_password".to_string()),
        dbname: Some("fake_db".to_string()),
        ..Default::default()
    }
}
