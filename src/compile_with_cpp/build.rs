
fn main() {
    cc::Build::new()
        .file("src/foo.cpp")
        .compile("foo")
}