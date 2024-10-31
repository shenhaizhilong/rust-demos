#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    #[test]
    fn binary_heap_test() {
        // 创建一个空的最大堆
        let mut heap = BinaryHeap::new();
        assert_eq!(heap.peek(), None);
        heap.push(1);
        heap.push(20);
        heap.push(3);
        heap.push(5);
        heap.push(4);
        assert_eq!(heap.peek(), Some(&20));
        assert_eq!(heap.pop(), Some(20));
    }
}