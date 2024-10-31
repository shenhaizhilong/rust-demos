#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn btree_map() {
        // 有序map
        let mut map = BTreeMap::new();
        map.insert(3, 4);
        map.insert(1, 2);
        map.insert(5, 6);

        println!("{:#?}", map);
    }
}