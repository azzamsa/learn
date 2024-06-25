const ENV_DATABASE_URL: &str = "DATABASE_URL";

pub struct Config {
    pub database: Database,
}

/// Database contains the data necessary to connect to a database
pub struct Database {
    pub url: String,
}

impl Config {
    pub fn load() -> Result<Self, crate::Error> {
        dotenvy::dotenv().ok();
        let database = {
            let url =
                std::env::var(ENV_DATABASE_URL).map_err(|_| env_not_found(ENV_DATABASE_URL))?;

            Database { url }
        };
        let config = Self { database };
        Ok(config)
    }
}

fn env_not_found(var: &str) -> crate::Error {
    crate::Error::NotFound(format!("config: {var} env var not found"))
}
