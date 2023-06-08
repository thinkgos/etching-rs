use prost_build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    prost_build::compile_protos(&["mapper/announce.proto"], &["mapper/"])?;

    Ok(())
}
