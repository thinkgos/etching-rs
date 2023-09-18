use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::configuration;
use crate::middleware::simple::SayHi;
use crate::router;
use crate::runtime::Runtime;

pub async fn run(c: &configuration::Setting) -> Result<(), anyhow::Error> {
    let rt = Runtime::new(c).await?;
    let rt = web::Data::new(rt);

    let bind_addr = c.app.addr();
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(rt.clone())
            .wrap(TracingLogger::default())
            .wrap(SayHi)
            .configure(router::api)
    })
    .bind(bind_addr)?
    .run()
    .await?)
}
