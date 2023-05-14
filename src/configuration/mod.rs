use std::env;

use anyhow::anyhow;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub app: App,
    pub database: Database,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub dialect: String,
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub require_ssl: bool,
}

impl App {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Database {
    fn args(&self) -> String {
        if self.require_ssl {
            "?ssl-mode=required".to_owned()
        } else {
            "?ssl-mode=disabled".to_owned()
        }
    }

    pub fn url_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "{}://{}:{}@{}:{}{}",
            self.dialect,
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.args(),
        ))
    }
    pub fn url(&self) -> Secret<String> {
        Secret::new(format!(
            "{}://{}:{}@{}:{}/{}{}",
            self.dialect,
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_name,
            self.args(),
        ))
    }
}

pub fn get_configuration() -> Result<Setting, anyhow::Error> {
    let work_dir = env::current_dir()?;
    let config_dir = work_dir.join("conf");

    let deploy: Deploy = env::var("APP_DEPLOY_MODE")
        .unwrap_or_else(|_| "dev".into())
        .try_into()?;

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base")))
        .add_source(config::File::from(config_dir.join(deploy.as_str())))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("."),
        )
        .build()?
        .try_deserialize()?;

    tracing::info!("{:?}", settings);
    Ok(settings)
}

/// The possible runtime environment for our application.
pub enum Deploy {
    Local,
    Dev,
    Uat,
    Prod,
}

impl Deploy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Deploy::Local => "local",
            Deploy::Dev => "dev",
            Deploy::Uat => "uat",
            Deploy::Prod => "prod",
        }
    }
}

impl TryFrom<String> for Deploy {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "dev" => Ok(Self::Dev),
            "uat" => Ok(Self::Uat),
            "prod" => Ok(Self::Prod),
            other => Err(anyhow!(
                "{} is not a supported environment. Use either `local`, `dev`, `uat`, `prod`.",
                other
            )),
        }
    }
}
