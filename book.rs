use mdbook::MDBook;
use std::{
    process::Command,
    thread::sleep,
    time::Duration
};

fn main() {
    // Compile book
    MDBook::load(&env!("CARGO_MANIFEST_DIR")).expect("Couldn't load book descriptor!")
        .build().expect("Book building failed!");
}