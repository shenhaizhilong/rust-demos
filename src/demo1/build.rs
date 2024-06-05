fn main() {
    cc::Build::new()
        .file("src/bar.c")
        .file("src/multiply.c")
        .compile("bar");
    println!("cargo:rerun-if-changed=src/bar.c")
}


