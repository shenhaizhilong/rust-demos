#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub element: T,
    pub next: Box<ListNode<T>>,
}

impl<T> ListNode<T> {
    #[inline]
    fn new(val: T, list_node: ListNode<T>) -> Self {
        ListNode { element: val, next: Box::new(list_node) }
    }
}