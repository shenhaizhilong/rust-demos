use std::net::TcpStream;
use std::ptr::addr_of_mut;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
        }
    }

    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(mut box_node) = curr_link {
            curr_link = box_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List;

    #[test]
    fn test1() {
        let mut list = List::new();
        list.push(10);
        for i in 0..10 {
            list.push(i);
        }

        println!("{:?}", list);
    }
}