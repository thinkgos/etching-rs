use prost_build;

fn main() {
    prost_build::compile_protos(&["mapper/announce.proto"], &["entities/src"]).unwrap();
}
