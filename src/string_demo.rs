#[cfg(test)]
mod tests {
    #[test]
    fn f1() {
        println!("{:?}", b"abc");
        println!("{:?}", &[97, 98, 99]);
    }

    #[test]
    fn t1() {
        let s1 = String::from("hello ");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        println!("{}", s1);
        println!("{}", s3);
    }

    #[test]
    fn t2() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);

        println!("{}", s);
    }

    #[test]
    fn t3() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("{s2}");
        println!("{s2}");
        println!("{}", s);
    }

    #[test]
    fn t4() {
        let len1 = String::from("Hola").len();

        let len2 = String::from("你好").len();

        println!("{}", len1);

        println!("{}", len2);

        let s1 = String::from("hello");
        // let h = s1[0];  `String` cannot be indexed by `{integer}` the trait `Index<{integer}>` is not implemented for `String`
    }

    #[test]
    fn t5() {
        let s1 = "hello world!";
        let s2 = &s1[0..5];
        println!("{}", s2);
    }


    #[test]
    fn t6() {
        let s1 = "你好世界！";
        let s2 = &s1[0..15];
        println!("{}", s2);
    }

    #[test]
    fn t7() {
        let s1 = "你好世界!";
        for i in s1.chars() {
            println!("{i}");
        }
        println!("{}", s1)
    }

    #[test]
    fn t8() {
        let s1 = "你好世界!";
        for i in s1.bytes() {
            println!("{i}");
        }
    }

    #[test]
    fn test9() {
        let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
        println!("a: {a:?}");

        let s: &[i32] = &a[2..4];
        // cannot assign to `a[_]` because it is borrowed, `a[_]` is assigned to here but it was already borrowed,
        // 可以把  a[3] = 10 放到切片前或者放到打印切片s语句后面, 当切片不在使用的时候
        // a[3] = 10;
        println!("s: {s:?}");

    }
}
