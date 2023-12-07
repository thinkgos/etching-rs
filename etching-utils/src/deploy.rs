use anyhow::anyhow;

/// 应用程序运行时环境
pub enum Deploy {
    Local, // 本地
    Dev,   // 开发
    Uat,   // 预发布
    Prod,  // 生产
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
