#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    #[test]
    fn btree_set() {
        let mut btree_set = BTreeSet::<i32>::new();
        for i in 0..10 {
            btree_set.insert(9 - i);
        }
        for i in 0..10 {
            btree_set.insert(i);
        }
        assert_eq!(btree_set.len(), 10);

        println!("{:?}", btree_set);
    }
}