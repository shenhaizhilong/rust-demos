#[cfg(test)]
mod tests {
    use std::collections::{HashSet};

    #[test]
    fn hashset_demo() {
        // 无序集合
        let mut set = HashSet::new();
        for i in 0..10 {
            set.insert(i);
            set.insert(i);
        }

        assert_eq!(set.len(), 10);

        assert_eq!(set.contains(&7), true);
        assert_eq!(set.contains(&80), false);

        println!("{:?}", set);
    }

    #[test]
    fn hashmap_demo2() {
        let mut a = HashSet::from([1, 2, 3, 4, 5, 5, 1]);
        println!("{:?}", a);
    }
}