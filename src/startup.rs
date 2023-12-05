use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::configuration;
use crate::middleware::simple::SayHi;
use crate::router;
use crate::router::api::ApiDoc;
use crate::runtime::Runtime;

pub async fn run(c: &configuration::Setting) -> Result<(), anyhow::Error> {
    let rt = Runtime::new(c).await?;
    let rt = web::Data::new(rt);
    let bind_addr = c.app.addr();
    let openapi = ApiDoc::openapi();

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(rt.clone())
            .wrap(TracingLogger::default())
            .wrap(SayHi)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .configure(router::api)
    })
    .bind(bind_addr)?
    .run()
    .await?)
}
