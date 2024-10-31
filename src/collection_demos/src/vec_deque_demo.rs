#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn vec_deque_demo_test() {
        // 双端队列
        let mut vec_deque = VecDeque::new();
        vec_deque.push_front(1);
        vec_deque.push_front(2);
        vec_deque.push_back(3);
        vec_deque.push_back(4);
        println!("{:?}", vec_deque);
        if let Some(v) = vec_deque.pop_front() {
            assert_eq!(v, 2);
        }

        if let Some(v) = vec_deque.pop_back() {
            assert_eq!(v, 4);
        }
    }
}