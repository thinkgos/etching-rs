use actix_web::post;
use actix_web::{web, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use entity::{prelude::SysUser, sys_user::Column as SysUserColumn};

use crate::error::Error;
use crate::runtime::Runtime;

#[derive(Debug, Deserialize)]
pub(crate) struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct LoginResponse {
    token: String,
    expires_at: i64,
}

#[post("/public/login")]
pub(crate) async fn login(
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
