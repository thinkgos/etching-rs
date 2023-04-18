use std::io;

use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use etching::logic::AppState;
use etching::router;
use etching::telemetry;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("etching", "info", io::stdout));

    tracing::info!("hello etching!!!");

    let app_state = web::Data::new(AppState {
        app_name: "你好".to_owned(),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(TracingLogger::default())
            .configure(router::api)
    })
    .bind("127.0.0.1:9527")?
    .run()
    .await?;
    Ok(())
}
