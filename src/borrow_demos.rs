#[cfg(test)]
mod tests {
    use std::io::stdout;

    #[test]
    fn test() {
        let mut v = vec![1, 2, 3];
        let v0 = v[0];
        v.push(4);
        println!("v0={v0}");
    }

    #[test]
    fn test2() {
        let mut v = vec![1, 2, 3];
        let v0 = v.get_mut(0);
        // v.push(4);
        println!("v0={}", v0.unwrap());
        ;
    }

    #[test]
    fn test3() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        let v0 = v.get_mut(0);
        println!("v0={}", v0.unwrap());
        ;
    }

    #[test]
    fn test4() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        let v0 = v.get(0);
        println!("v0={}", v0.unwrap());
        ;
    }

    #[test]
    fn test5() {
        let mut v = vec![1, 2, 3];
        let v0 = v.get(0);
        println!("v0={}", v0.unwrap());
        ;
        v.push(4);
    }

    // https://zhuanlan.zhihu.com/p/676999125
    #[test]
    fn test6() {
        let a = 5;
        let b = a;  // a 为 i32 类型 实现了 Copy trait，拥有复制语义，运行时 Rust会自动复制一个5并赋值给 b，且不会带来任何性能开销
        print!("{b}")
    }
    // 借用，就是向某个值的owner借一个值，用完再还给owner。这和平常人与人之间借东西一样。
    // 拥有所有权≠一直持有这个值

    #[test]
    fn test7() {
        let a = 10;
        let mut b = 10;
        // 此为不可变借用
        let ref_a = &a;
        // 此为可变借用
        let ref_b = &mut b;
        *ref_b = 20;
        *ref_b *= 3;
        assert_eq!(b, 60);

        println!("b={}", b);
        let c = 20;
        // 可以对不可变值或可变值进行不可变借用，但不可以对不可变值进行可变借用！
        // 如以下被注释掉的代码，去掉注释，程序就不能正常编译运行
        // let invalid_mutable_borrow = &mut c;

        // Rust还支持这种方式的借用声明
        let mut d = 30;
        let ref ref_d = d;
        println!("ref_d={ref_d}");
    }

    #[test]
    fn test_str() {
        let a = "hell";
        let b = "world!";
        let c = longest(a, b);
        println!("{c}")
    }

    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if (a.len() > b.len()) {
            a
        } else { b }
    }

    #[test]
    fn test_string() {
        let x = String::from("hello");
        {
            let a;
            let y = String::from("world!");
            a = longest2(&x, &y);
            println!("{a}");
        }
    }

    fn longest2<'a>(a: &'a String, b: &'a String) -> &'a String {
        if a.len() > b.len() {
            return a;
        }
        return b;
    }


    #[test]
    fn test_string2() {
        let x = String::from("hello");
        let a;
        {
            let y = String::from("world!");
            a = longest3(x, y);
        }
        println!("{a}");
    }

    fn longest3(a: String, b: String) -> String {
        if a.len() > b.len() {
            return a;
        }
        return b;
    }



    #[test]
    fn test_borrow() {
        {
            // let r;                // ---------+-- 'a
            //                             //          |
            // {                           //          |
            //     let x = 5;         // -+-- 'b  |
            //     r = &x;                 //  |       |
            // }                           // -+       |
            // //          |
            // println!("r: {}", r); //                |
        }
    }

    #[test]
    fn test_6() {
        let mut var = 0_i32;
        {
            let p1 = &mut var;
            *p1 = 1;
            println!("p1 change var: {var:?}")
        }
        {
            let tmp = 2_i32;
            let mut p2 = &var;
            p2 = &tmp;
            println!("p2 change var: {var:?}")
        }
        {
            let mut tmp = 3_i32;
            let mut p3 = &mut var;
            *p3 = 4;
            p3 = &mut tmp;
            println!("p3 change var: {var:?}");
            println!("p3 : {p3:?}");
        }
    }


}