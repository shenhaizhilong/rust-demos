struct Apple {
    content: String,
}

// We create a struct , and implement a method on this struct, parameter self type is Box<Self>.
impl Apple {
    // When we implement method in a struct , we specify self parameter is a type of Box<Self> ,
    // this means this method only can work on an instance of this struct in a box.
    fn get_content(self: Box<Self>) {
        println!("{}", self.content);
    }
}

#[cfg(test)]
mod tests {
    use crate::box_demo::Apple;

    #[test]
    fn test1() {
        let a = Apple { content: String::from("a") };
        let b = Box::new(Apple { content: String::from("b") });
        // println!("{:?}", a.get_content());
        b.get_content();
    }
}