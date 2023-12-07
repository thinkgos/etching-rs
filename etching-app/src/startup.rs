use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use etching_handler::{configure_api, ApiDoc};
use etching_middleware::simple::SayHi;

use crate::configuration;
use crate::runtime::Runtime;

pub async fn run(c: &configuration::Setting) -> Result<(), anyhow::Error> {
    let rt = Runtime::new(c).await?;
    let rt = web::Data::new(rt);
    let db_pool = web::Data::new(rt.db_pool.clone());

    let bind_addr = c.app.addr();
    let openapi = ApiDoc::openapi();
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(rt.clone())
            .app_data(db_pool.clone())
            .wrap(TracingLogger::default())
            .wrap(SayHi)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .configure(configure_api)
    })
    .bind(bind_addr)?
    .run()
    .await?)
}
