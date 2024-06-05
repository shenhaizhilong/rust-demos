#[derive(Debug)]
struct D(i32);

impl Drop for D {
    fn drop(&mut self) {
        println!("destruct {}", self.0);
    }
}


#[cfg(test)]
mod tests {
    use crate::drop_demo::D;

    #[test]
    fn test1() {
        let d1 = D(10);
        println!("construct {:?}", d1);
        {
            let d2 = D(20);
            println!("construct {:?}", d2);
        }
    }
}