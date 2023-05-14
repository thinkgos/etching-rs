use actix_web::{web, App, HttpServer};
use sea_orm::Database;
use secrecy::ExposeSecret;
use tracing_actix_web::TracingLogger;

use crate::configuration;
use crate::router;

pub async fn run(c: &configuration::Setting) -> Result<(), anyhow::Error> {
    let pool = Database::connect(c.database.url().expose_secret()).await?;

    let db_pool = web::Data::new(pool);

    let bind_addr = c.app.addr();
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .wrap(TracingLogger::default())
            .configure(router::api)
    })
    .bind(bind_addr)?
    .run()
    .await?)
}
