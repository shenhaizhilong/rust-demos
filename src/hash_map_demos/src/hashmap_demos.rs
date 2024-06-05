
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn t1() {
        let mut map = HashMap::new();
        map.insert("tom", 10);
        map.insert("alice", 12);
        for (k, v) in map {
            println!("{k}, {v}")
        }
    }

    #[test]
    fn t2() {
        // Create a new empty HashMap
        let mut my_map = HashMap::new();

        // Insert key-value pairs into the HashMap
        my_map.insert("key1", "value1");
        my_map.insert("key2", "value2");
        my_map.insert("key3", "value3");

        // Access values using keys
        match my_map.get("key2") {
            Some(value) => println!("Value for key2: {}", value),
            None => println!("Key2 not found"),
        }

        // Iterate over key-value pairs
        for (key, value) in &my_map {
            println!("Key: {}, Value: {}", key, value);
        }

        // Check if a key exists in the HashMap
        if my_map.contains_key("key4") {
            println!("Key4 exists");
        } else {
            println!("Key4 does not exist");
        }

        // Update a value associated with a key
        my_map.insert("key2", "new_value2");
        println!("Updated value for key2: {}", my_map["key2"]);

        // Remove a key-value pair
        my_map.remove("key1");

        // Check the size of the HashMap
        println!("Size of the HashMap: {}", my_map.len());
    }

    #[test]
    fn t3() {
        let tems = vec!["Blue", "Yello"];
        let score = vec![10, 20];
        let scores: HashMap<_, _> = tems.iter().zip(score.iter()).collect();
        for (k, v) in &scores {
            println!("{}-> {}", k, v);
        }

        for p in scores.iter() {
            println!("{:?}", p);
        }

        for p in scores.iter() {
            println!("{:?}", p);
        }
    }


    #[test]
    fn t4() {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    #[test]
    fn t5() {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }

    #[test]
    fn t6() {

    }
}