const fn cube(num: usize) -> usize {
    return num * num * num;
}

const DIM: usize = cube(2);
const ARR: [i32; DIM] = [0; DIM];

fn fib(n: i32) -> i32 {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}


#[cfg(test)]
mod tests {
    use crate::const_fn::{ARR, DIM, fib};

    #[test]
    fn test1() {
        println!("{}", DIM);
        println!("{:?}", ARR);
    }

    #[test]
    fn test2() {
        for i in 0..10 {
            println!("fib({})={}", i, fib(i));
        }
    }
}
