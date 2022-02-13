use std::path::Path;

use cargo_get::toml::CargoToml;

fn main() {
    CargoToml::from_filepath(Path::new(".")).map_or_else(
        |e| println!("Failed: {:?}", e),
        |ok| println!("Ok: {:?}", ok),
    );
}
