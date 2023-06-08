use etching::configuration;

/// env APP_DEPLOY=local cargo run --example config
fn main() {
    let c = configuration::get_configuration();
    println!("{:#?}", c.unwrap());
    let s = "s";
    s.to_owned()
}
