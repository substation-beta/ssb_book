fn main() {
    mdbook::MDBook::load(&env!("CARGO_MANIFEST_DIR")).expect("Couldn't load book descriptor!")
        .build().expect("Book building failed!");
}