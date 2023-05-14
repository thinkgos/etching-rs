use std::io;

use etching::configuration;
use etching::startup;
use etching::telemetry;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_subscriber(telemetry::get_subscriber("etching", "info", io::stdout));

    let c = configuration::get_configuration()?;
    startup::run(&c).await?;
    Ok(())
}
