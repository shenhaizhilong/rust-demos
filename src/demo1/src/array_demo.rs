fn print_slice(a: &[i32]) {
    println!("Length:{}", a.len());
    for i in a {
        print!("{} \t", i);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::array_demo::print_slice;

    #[test]
    fn test1() {
        let array = &[1, 2, 3, 4, 5];
        for i in array {
            println!("{}", i);
        }
    }

    #[test]
    fn test2() {
        let r = 1..=10;
        for i in r {
            println!("{}", i);
        }

        let range = (1..=10).rev().map(|i| i * 10);
        for i in range {
            println!("{}", i);
        }
    }

    #[test]
    fn test3() {
        let arr = [1, 2, 3, 4, 5];
        print_slice(&arr[2..]);
        print_slice(&arr);

        print_slice(&arr[..2]);
    }

    #[test]
    fn test4() {
        let x = 10;
        match x {
            e @ 1..=5 => { println!("element: {}", e); }
            _ => println!("anything")
        }
    }
}