use sea_orm::{Database, DatabaseConnection};
use secrecy::ExposeSecret;

use crate::configuration;

pub struct Runtime {
    pub db_pool: DatabaseConnection,
}

impl Runtime {
    pub async fn new(c: &configuration::Setting) -> Result<Self, anyhow::Error> {
        let db_pool = Database::connect(c.database.url().expose_secret()).await?;

        Ok(Self { db_pool })
    }
}
