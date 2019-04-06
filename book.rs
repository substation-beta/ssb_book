use mdbook::MDBook;
use std::process::Command;

fn main() {
    // Compile book
    MDBook::load(&env!("CARGO_MANIFEST_DIR")).expect("Couldn't load book descriptor!")
        .build().expect("Book building failed!");
    // Compile book SVG to PDF
    Command::new("inkscape")
        .args(&[
            "--without-gui",
            "--export-pdf=target/book/ssb_reference_card.pdf",
            "target/book/ssb_reference_card.svg"
        ])
        .output().expect("Couldn't compile svg to pdf with inkscape!");
}