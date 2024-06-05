#[cfg(test)]
mod map_demos {
    use std::collections::HashMap;

    #[test]
    fn test1() {
        let mut map = HashMap::new();
        // insert
        map.insert("tom".to_string(), 20);
        map.insert("alice".to_string(), 30);

        // find
        let name = "tom2".to_string();
        match &map.get(&name) {
            None => { println!("not found: {}", name); }
            Some(v) => { println!("found: {}, {}", &name, v); }
        }

        // iter

        for (k, v) in &map {
            println!("({}, {})", k, v);
        }

        // update if not exist
        map.entry("alice".to_string()).or_insert(31);


        for (k, v) in &map {
            println!("({}, {})", k, v);
        }
    }

    #[test]
    fn test2() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}