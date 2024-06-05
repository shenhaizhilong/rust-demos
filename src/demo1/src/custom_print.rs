use std::fmt::Debug;

fn my_print<T: Debug>(x: T) {
    println!("The value of x is: {:?}", x);
}

fn my_print2<T> (x: T) where T: Debug {
    println!("The value of xx is: {:?}", x);
}

#[cfg(test)]
mod tests {
    use crate::custom_print::{my_print, my_print2};

    #[test]
    fn test1() {
        my_print("China");
        my_print(10);
        my_print(true);
        my_print(['a', 'b', 'c'])
    }

    #[test]
    fn test2() {
        my_print2("China");
        my_print2(10);
        my_print2(true);
        my_print2(['a', 'b', 'c'])
    }
}