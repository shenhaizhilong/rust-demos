/**
Blanket implementation refer to a way to implement a trait for all types or some types meet certain condition.

We define a trait Hello with a function hello. Then implement this Hello trait for generic type parameter T.
We didn’t add any constraint to this generic type parameter T,
so this T can be any type. We create two structs , We can call hello function from both structs.
**/
trait Hello {
    fn hello() {
        println!("hello!");
    }
}

impl<T> Hello for T {}

struct MyStructA {}

struct MyStructB {}

struct C {}

// impl Hello for C {
//     fn hello() {
//         println!("Hello C")
//     }
// }

#[cfg(test)]
mod tests {
    use crate::struct_demo::{C, Hello, MyStructA, MyStructB};

    #[test]
    fn test1() {
        MyStructA::hello();
        MyStructB::hello();
        C::hello();
    }
}