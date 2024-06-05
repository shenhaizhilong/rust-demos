/**
This time we define a new trait Say, and when we implement Hello trait ,we use where to add trait bound Say to generic type parameter T,
so this T must be the type which also implement Say trait.
We create 2 structs, only MyStructA implement Say trait. So we only can call hello function from MyStructA.
 **/
trait Hello {
    fn hello() {
        println!("hello!");
    }
}

trait Say {}

impl<T> Hello for T where T: Say
{}

struct MyStructA {}

struct MyStructB {}

impl Say for MyStructA {}

#[cfg(test)]
mod tests {
    use crate::struct_demo2::{Hello, MyStructA, MyStructB};

    #[test]
    fn test1() {
        MyStructA::hello();
        // MyStructB::hello();
    }
}