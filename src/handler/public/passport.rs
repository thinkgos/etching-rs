use actix_web::post;
use actix_web::{web, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{prelude::SysUser, sys_user::Column as SysUserColumn};

use crate::error::Error;
use crate::runtime::Runtime;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
struct LoginRequest {
    /// 用户名
    username: String,
    /// 用户密码
    password: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct LoginResponse {
    token: String,
    expires_at: i64,
}

/// 用户登陆
#[utoipa::path(
    request_body = inline(LoginRequest),
    responses(
        (status = 200, body = inline(LoginResponse), description = "login successfully")
    ),
)]
#[post("/v1/public/login")]
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
