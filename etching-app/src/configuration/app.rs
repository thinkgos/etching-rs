use serde::Deserialize;

/// 应用基本配置
#[derive(Debug, Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
}

impl App {
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
