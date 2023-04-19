use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("请求参数错误")]
    ValidationError,
    #[error("未经授权,请先登陆")]
    Unauthorized,
    #[error("没有权限,拒绝防问")]
    PermissionForbidden,
    #[error("Validation error on field")]
    AnyhowError(#[from] anyhow::Error),
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ValidationError => StatusCode::BAD_REQUEST,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::PermissionForbidden => StatusCode::FORBIDDEN,
            Error::AnyhowError(_) => StatusCode::BAD_GATEWAY,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
