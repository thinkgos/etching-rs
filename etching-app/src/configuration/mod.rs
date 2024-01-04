mod app;
mod database;

pub use self::app::App;
pub use self::database::Database;

use std::env;

use serde::Deserialize;

use crates_utils::deploy::Deploy;

// 配置
#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub app: App,
    pub database: Database,
}

impl Configuration {
    pub fn load() -> Result<Self, anyhow::Error> {
        let work_dir = env::current_dir()?;
        let config_dir = work_dir.join("conf");

        let deploy: Deploy = env::var("APP_DEPLOY_MODE")
            .unwrap_or_else(|_a| "dev".into())
            .parse()?;

        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join("app")))
            .add_source(config::File::from(config_dir.join(deploy.to_string())))
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
}
