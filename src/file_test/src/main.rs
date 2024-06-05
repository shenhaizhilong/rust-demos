use std::path::{Path, PathBuf};

pub mod mmap_demo;
mod file_demo1;

fn main() {
    let path = Path::new("data/hello.txt");
    println!("{}", path.display());
    println!("{:?}", path.exists());
    println!("{:?}", path.extension());
    println!("{:?}", path.file_name());
    println!("{:?}", path.parent());

    let path2 = Path::new("data");
    let path3 = path2.join("hello.txt");
    println!("{:?}", path3);

    assert_eq!(Path::new("/etc").join("passwd"), PathBuf::from("/etc/passwd"));
    assert_eq!(Path::new("/etc").join("/bin/sh"), PathBuf::from("/bin/sh"));
}
