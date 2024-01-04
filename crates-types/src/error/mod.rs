use std::collections::HashMap;

use actix_web::http::{header::ContentType, StatusCode};
use actix_web::{error, HttpResponse};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
struct Response {
    code: usize,
    message: String,
    detail: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("未经授权,请先登陆")]
    Unauthorized,
    #[error("没有权限,拒绝防问")]
    PermissionForbidden,
    #[error("请求参数错误")]
    ValidationError,
    #[error("业务处理失败, 请稍后再试")]
    DbError(#[from] DbErr),
    #[error("业务处理失败, 请稍后再试")]
    AnyhowError(#[from] anyhow::Error),
    #[error("用户名或密码错误")]
    UserOrPassword,
    #[error("用户名或密码错误次数超过限制")]
    UserOrPasswordOverTime,
    #[error("用户不存在")]
    UserNotExists,
    #[error("用户已存在")]
    UserAlreadyExists,
    #[error("用户未设置密码, 不能用密码登录")]
    UserNoPasswd,
    #[error("用户密码已存在, 请直接登录")]
    UserPasswdExist,
    #[error("旧密码不能与新密码相同")]
    SamePasswd,
    #[error("密码错误")]
    PasswordIncorrect,
    #[error("密码已失效")]
    PasswdExpired,
    #[error("密码错误次数超过限制")]
    PasswordOverQuota,
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ValidationError => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::PermissionForbidden => StatusCode::FORBIDDEN,
            Error::AnyhowError(_) => StatusCode::BAD_GATEWAY,
            _ => StatusCode::from_u16(599).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(Response {
                code: 999999999,
                message: self.to_string(),
                detail: self.to_string(),
                metadata: None,
            })
    }
}
