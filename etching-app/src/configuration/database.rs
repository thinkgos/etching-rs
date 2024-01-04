use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

/// 数据库配置
#[derive(Debug, Deserialize)]
pub struct Database {
    dialect: String,
    username: String,
    password: Secret<String>,
    port: u16,
    host: String,
    db_name: String,
    require_ssl: bool,
}

impl Database {
    /// url含db_name
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
    /// url不含db_name
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
    /// 参数部份, 包含`?`.
    fn args(&self) -> String {
        let ssl_mode = if self.require_ssl {
            "required"
        } else {
            "disabled"
        };
        format!("?ssl-mode={}", ssl_mode)
    }
}
