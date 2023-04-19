use std::io;

use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use etching::router;
use etching::telemetry;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("etching", "info", io::stdout));

    tracing::info!("hello etching!!!");

    HttpServer::new(move || {
        App::new()
            // .app_data(app_state.clone())
            .wrap(TracingLogger::default())
            .configure(router::api)
    })
    .bind("127.0.0.1:9527")?
    .run()
    .await?;

    Ok(())
}
