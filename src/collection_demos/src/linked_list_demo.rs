#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn linked_list() {
        let mut list = LinkedList::new();
        for i in 0..10 {
            list.push_back(i);
        }

        for i in 1..10 {
            list.push_front(i);
        }

        println!("{:?}", list);

        while !list.is_empty() {
            let curr = list.pop_front();
            match curr {
                None => {}
                Some(v) => {println!("{:?}", v);}
            }
        }
    }
}