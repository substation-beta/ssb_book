use mdbook::MDBook;
use std::{
    fs::{copy, create_dir_all, read_dir, FileType},
    io::Result
};

fn main() {
    // Working directory
    let build_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    // Compile book
    MDBook::load(build_dir.clone()).expect("Couldn't load book descriptor!")
        .build().expect("Book building failed!");
    // Copy assets
    copy(build_dir.clone() + "/assets/ssb_reference_card.pdf", build_dir.clone() + "/target/book/ssb_reference_card.pdf").expect("Couldn't copy reference card!");
    create_dir_all(build_dir.clone() + "/target/book/assets/img").expect("Couldn't create assets image directory!");
    copy_files(build_dir.clone() + "/assets/img/", build_dir.clone() + "/target/book/assets/img/").expect("Couldn't copy image files to output!");
}

fn copy_files<P: AsRef<str>>(src_dir: P, dst_dir: P) -> Result<()> {
    for entry in read_dir(src_dir.as_ref())? {
        if let Ok(entry) = entry {
            if entry.file_type().as_ref().map(FileType::is_file).unwrap_or(false) {
                copy(
                    &entry.path(),
                    &(dst_dir.as_ref().to_owned() + &entry.file_name().to_string_lossy().into_owned())
                )?;
            }
        }
    }
    Ok(())
}