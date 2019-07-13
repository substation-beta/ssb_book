use mdbook::MDBook;
use std::fs::*;

fn main() {
    // Working directory
    let build_dir = env!("CARGO_MANIFEST_DIR").to_owned();
    // Compile book
    MDBook::load(build_dir.clone()).expect("Couldn't load book descriptor!")
        .build().expect("Book building failed!");
    // Copy assets
    copy(build_dir.clone() + "/assets/ssb_reference_card.pdf", build_dir.clone() + "/target/book/ssb_reference_card.pdf").expect("Couldn't copy reference card!");
    create_dir_all(build_dir.clone() + "/target/book/assets/img").expect("Couldn't create assets image directory!");
    copy_files(build_dir.clone() + "/assets/img/", build_dir.clone() + "/target/book/assets/img/");
}

fn copy_files(src_dir: String, dst_dir: String) {
    if let Ok(dir) = read_dir(src_dir) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Ok(entry_type) = entry.file_type() {
                    if entry_type.is_file() {
                        let from = entry.path();
                        let to = dst_dir.clone() + entry.file_name().to_str().unwrap();
                        copy(&from, &to).expect(&format!("Couldn't copy file '{}' to '{}'!", from.display(), to));
                    }
                }
            }
        }
    }
}