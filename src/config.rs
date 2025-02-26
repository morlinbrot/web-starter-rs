use std::{env, sync::OnceLock};

use crate::{Error, Result};

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();

    CONFIG.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            panic!("FATAL - Failed to load config from environment - cause: {err:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub APP_HOST: std::net::IpAddr,
    pub APP_PORT: u16,
    pub DATABASE_URL: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        #[cfg(debug_assertions)]
        dotenvy::dotenv().expect("Failed to load .env file");

        let db_host = get_env("DB_HOST")?;
        let db_port = get_env("DB_PORT").or(Ok("5432".to_string()))?;
        let db_pw = get_env("POSTGRES_PASSWORD")?;
        let db_user = get_env("POSTGRES_USER")?;
        let db_name = get_env("POSTGRES_DB")?;

        Ok(Config {
            APP_HOST: get_env("SERVICE_APP_HOST")?
                .parse()
                .map_err(|_err| Error::ConfigFailedToParseIpAddr("SERVICE_APP_HOST"))?,
            APP_PORT: get_env("SERVICE_APP_PORT")?
                .parse()
                .map_err(|_err| Error::ConfigFailedToParseInt("APP_PORT"))?,
            DATABASE_URL: format!("postgres://{db_user}:{db_pw}@{db_host}:{db_port}/{db_name}"),
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_err| Error::ConfigMissingEnv(name))
}
