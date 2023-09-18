use actix_web::{dev::Service, web, App, HttpServer};
use futures_util::future::FutureExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi <<wrap_fn>> from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi <<wrap_fn>> from response end!");
                    res
                })
            })
            .route(
                "/index.html",
                web::get().to(|| async { "Hello, middleware!" }),
            )
    })
    .bind("127.0.0.1:9999")?
    .run()
    .await
}
