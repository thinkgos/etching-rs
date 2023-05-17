mod entities;
pub use entities::*;

// Include the `items` module, which is generated from items.proto.
pub mod mapper {
    include!(concat!(env!("OUT_DIR"), "/mapper.announce.rs"));
}
