extern crate prost_build;

fn main() {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(Serialize, Deserialize)]");
    config.compile_well_known_types();
    config
        .compile_protos(&["src/proto/media.proto"], &["src/"])
        .unwrap();
}
