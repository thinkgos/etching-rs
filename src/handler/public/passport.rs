use actix_web::post;
use actix_web::{web, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{prelude::SysUser, sys_user::Column as SysUserColumn};

use crate::error::Error;
use crate::runtime::Runtime;

/// login 请求
#[derive(Debug, Deserialize, utoipa::ToSchema)]
struct LoginRequest {
    /// 用户名
    username: String,
    /// 用户密码
    password: String,
}

/// login 回复
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    /// token value
    token: String,
    /// token expires in seconds
    expires_at: i64,
}

/// 用户登陆
///
/// 用户账号登陆并获取token和token过期时间
#[utoipa::path(
    security(()),
    context_path = "/v1",
    request_body = inline(LoginRequest),
    responses(
        (status = StatusCode::OK, body = inline(LoginResponse), description = "login successfully")
    ),
)]
#[post("/public/login")]
pub async fn login(
    rt: web::Data<Runtime>,
    req: web::Json<LoginRequest>,
) -> Result<impl Responder, Error> {
    let user = SysUser::find()
        .filter(SysUserColumn::Name.eq(&req.username))
        .one(&rt.db_pool)
        .await?
        .ok_or(Error::UserOrPassword)?;

    bcrypt::verify(&req.password, &user.passwd)
        .map_err(|_| Error::UserOrPassword)?
        .then_some(())
        .ok_or(Error::UserOrPassword)?;
    Ok(web::Json(LoginResponse {
        token: "token".to_owned(),
        expires_at: 10000,
    }))
}
