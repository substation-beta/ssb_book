use open::that;

fn main() {
    println!("Documents were generated at compile time. Look into project outputs 'book' directory!");
    that(concat!(env!("CARGO_MANIFEST_DIR"), "/target/book/index.html")).expect("Should have opened documentation in browser!");
}