
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn hashmap_demo_1() {
        // 无序 map
        let mut m = HashMap::new();
        for i in 1..=10 {
            m.insert(i, i * 2);
        }

        assert_eq!(m.get(&1), Some(&2));
        assert_eq!(m.get(&20), None);
        assert_eq!(m.len(), 10);

        m.remove(&1);
        assert_eq!(m.len(), 9);

        m.iter().filter(|&(k, v)| k & 0x01 == 0).for_each(|(k, v)| println!("{}, {}", k, v));

        println!("{}", "*******************");

        for (k, v) in m.iter() {
            println!("{}, {}", k, v);
        }

        println!("{}", "*******************");

        println!("{:?}", m);

    }

    #[test]
    fn hashmap_demo_2() {
        let mut m = HashMap::from([(1, 2), (1, 3), (1, 4), (2, 1), (2, 3), (2, 4)]);
        m.remove(&1);
        println!("{:?}", m);
    }
}