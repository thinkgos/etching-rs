use etching_rs::configuration;

/// env APP_DEPLOY=local cargo run --example config
fn main() {
    let c = configuration::get_configuration();
    println!("{:#?}", c.unwrap());
}
