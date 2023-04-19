use actix_web::{HttpResponse, Responder};

pub(crate) async fn logout() -> impl Responder {
    HttpResponse::Ok().finish()
}
