use std::path::Path;

#[test]
fn test1() {
    let path = Path::new("data/hello.txt");
    println!("{}", path.display());
    println!("{:?}", path.exists());
    println!("{:?}", path.extension());
}