#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let name = "Rust".to_string();
        let greet = move || {
            println!("Hello, {}!", name);
        };

        greet();// Correct: the string name has been moved into the closure
        // println!("{}", name);  // Error: `name` has been moved and can no longer be accessed here
    }

    #[test]
    fn test2() {
        let numbers = vec![1, 2, 3, 4, 5];
        let even_numbers: Vec<_> = numbers.into_iter()
            .filter(|&x| x % 2 == 0)
            .collect();
        println!("even_numbers:{:?}", even_numbers);
    }

    #[test]
    fn test3() {
        // return a closure, Closures are represented by trait, Fn, FnMut and FnOnce, because traits are dynamic size types
        // we can't directly return a closure trait type, But we put a trait behind a pointer.
        fn hello() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| { x + 2 })
        }

        let a = hello()(1);
        println!("{:?}", a);
    }

    #[test]
    fn test4() {
        fn add_one(x: i32) -> i32 {
            return x + 1;
        }

        fn hello() {
            println!("Hello!");
        }

        fn do_it(f1: fn(i32) -> i32, f2: fn(), x: i32) -> i32 {
            f2();
            f1(x)
        }

        let f = do_it(add_one, hello, 2);
        println!("{:?}", f);
    }

    #[test]
    fn test5() {
        // Because function pointer implement three traits : Fn, FnMut, and FnOnce,
        // so if a method or function expect a closure, we also can pass a function as argument.
        let v1 = vec![1, 2, 3, 4, 5];
        let a : Vec<String> = v1.iter().map(ToString::to_string).collect();
        let b: Vec<String> = v1.iter().map(|x| {x.to_string()}).collect();
        println!("{:?}", a);
        println!("{:?}", b);
    }
}