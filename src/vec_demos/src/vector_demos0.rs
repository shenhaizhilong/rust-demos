#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];//不可变的借用
        // v.push(6);//可变的借用
        println!("first is {}", first);//不可变的借用
    }

    #[test]
    fn test_2() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.push(6);//可变的借用
        let first = &v[0];//不可变的借用
        println!("first is {}", first);//不可变的借用
    }

    #[test]
    fn test_3() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        for i in v {
            println!("{}", i);
        }
    }

    #[test]
    fn test_4() {
        // into_iter()
        // This function consumes the vector and produce an iterator that takes ownership of each element in the vector. Taking Ownership means original vector is consumed and can no longer be used.
        //     That means elements are moved, rather of being copied.
        let v = vec![1, 2, 3];
        for i in v.into_iter() {
            // i takes ownership of each element in vec
            print!("{i}")
        }
        // // vec is consumed and can no longer be used here
        // println!("{:?}", v);
    }

    #[test]
    fn test_5() {
        let v2: Vec<i32> = vec![1, 2, 34];
        let mut v = vec![0; 5]; // size 5, init with 0
        let v3: Vec<i32> = [1, 2, 3].to_vec();
    }

    #[test]
    fn test_6() {
        let v = vec![3, 2, 4];
        for (idx, num) in v.into_iter().enumerate() {
            println!("index={}, value={}", idx, num);
        }
    }

    /**
     O(n) 保留顺序
    **/
    #[test]
    fn test_remove() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.remove(1);
        println!("{:?}", v);
    }


    /**
     O(1), 不保留顺序
    **/
    #[test]
    fn test_swap_remove() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.swap_remove(1);
        println!("{:?}", v);
    }

    /**
    retain 需要一个闭包
    **/
    #[test]
    fn test_retain() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.retain(|&a| a % 2 == 0);
        println!("{:?}", v);
    }

    #[test]
    fn test_clear() {
        let mut v1 = vec![1, 2, 3, 4, 5];
        v1.clear();
        println!("{:?}", v1);
    }


    /**
    pop 方法移除 vec 中的最后一个元素
    **/
    #[test]
    fn test_pop() {
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.pop(), Some(3));
        assert_eq!(vec, [1, 2]);

    }
}