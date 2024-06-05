#[cfg(test)]
pub mod test {
    #[test]
    fn test1() {
        println!("{:o}", 9);
        println!("{:x}", 255);
        println!("{:X}", 255);
        println!("{:p}", &0);
        println!("{:b}", 15);
        println!("{:e}", 1000f32);

    }
}