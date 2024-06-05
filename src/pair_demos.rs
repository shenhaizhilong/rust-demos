use std::fmt::Display;

#[derive(Debug)]
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn com_display(&self) {
        if self.x >= self.y {
            println!("the largest number is x= {}", self.x);
        } else {
            println!("the largest number is y= {}", self.y);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::pair_demos::{Pair};

    #[test]
    fn test1() {
        let a = Pair{x: 1, y: 2};
        println!("{:?}", a);
        a.com_display();

    }



}