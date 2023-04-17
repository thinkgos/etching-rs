use std::io;

use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

use etching::handler::misc::healthy;
use etching::handler::passport::login;
use etching::handler::passport::logout;
use etching::logic::AppState;
use etching::telemetry;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("etching", "info", io::stdout));

    tracing::info!("hello etching!!!");

    let app_state = web::Data::new(AppState {
        app_name: "你好".to_owned(),
    });
    HttpServer::new(move || {
        App::new().app_data(app_state.clone()).service(
            web::scope("/api")
                .wrap(TracingLogger::default())
                .route("/v1/public/login", web::get().to(login))
                .route("/v1/logout", web::get().to(logout))
                .route("/v1/public/healthy", web::get().to(healthy))
                .route("/v1/public/hello", web::get().to(healthy)),
        )
    })
    .bind("127.0.0.1:9527")?
    .run()
    .await?;
    Ok(())
}
