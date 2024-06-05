extern {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    let v = unsafe {
        multiply(10, 2)
    };

    println!("{:?}", v);
}
