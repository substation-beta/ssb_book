use std::path::PathBuf;
use mdbook::Config;
use mdbook::MDBook;

fn main() {
    // Output directory
    let mut config = Config::default();
    config.build.build_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/book"));
    // Compile markdown book
    MDBook::load_with_config(&env!("CARGO_MANIFEST_DIR"), config).expect("Couldn't load book sources!")
        .build().expect("Book building failed!");
}