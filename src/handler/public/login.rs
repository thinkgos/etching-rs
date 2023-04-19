use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct LoginRequest {
    _username: String,
    _password: String,
}

pub(crate) async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    tracing::info!("{:?}", req);

    HttpResponse::Ok().finish()
}
