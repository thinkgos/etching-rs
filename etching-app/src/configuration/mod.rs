use std::env;

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

use etching_utils::deploy::Deploy;

// 配置
#[derive(Debug, Deserialize)]
pub struct Setting {
    pub app: App,
    pub database: Database,
}

// 应用基本配置
#[derive(Debug, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
}

/// 数据库配置
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
    // 参数部份, 包含`?`.
    fn args(&self) -> String {
        if self.require_ssl {
            "?ssl-mode=required".to_owned()
        } else {
            "?ssl-mode=disabled".to_owned()
        }
    }
    // url不含db_name
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
    // url含db_name
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
        .add_source(config::File::from(config_dir.join("app")))
        .add_source(config::File::from(config_dir.join(deploy.as_str())))
        .add_source(
            // 环境变量: APP_XX.YY, 例如端口: APP_APP.PORT=9999
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("."),
        )
        .build()?
        .try_deserialize()?;

    tracing::info!("{:?}", settings);
    Ok(settings)
}
