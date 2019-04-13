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
    // Compile book FODG to PDF
    Command::new("soffice")
        .args(&[
            "--headless",
            "--convert-to",
            "pdf:draw_pdf_Export",
            "--outdir",
            concat!(env!("CARGO_MANIFEST_DIR"), "/target/book/"),
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/ssb_reference_card.fodg")
        ])
        .output().expect("Couldn't compile fodg to pdf with libreoffice!");
    // Libreoffice sends commands to a remote controller (soffice.bin), so we need to wait for async processing
    sleep(Duration::from_secs(5));
}