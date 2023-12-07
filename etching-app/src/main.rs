use std::io;

use etching_app::configuration;
use etching_app::startup;
use etching_app::telemetry;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("etching", "info", io::stdout));

    let c = configuration::get_configuration()?;
    startup::run(&c).await?;
    Ok(())
}
