use std::ffi::c_int;


extern "C" {
    fn abs(input: i32) -> i32;

    fn multiply(a: i32, b: i32) -> i32;
    fn bar_function(x: i32) -> i32;

}

#[cfg(test)]
mod tests {
    use std::ffi::c_int;
    use crate::c_demo::{abs, bar_function, multiply};

    #[test]
    fn test1() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    #[test]
    fn test2() {
        unsafe {
            println!("bar function c:{}", bar_function(50));
            println!("multiply function c:{}", multiply(50, 10));
        }

    }

}