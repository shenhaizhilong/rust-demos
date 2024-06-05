use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct Node2 {
    value: i32,
    next: Option<Rc<Node2>>
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::rc_demo::{Node, Node2};

    #[test]
    fn test1() {
        let n1  = Node {
            value: 1,
            next: None
        };
        let n2 = Node {
            value: 2,
            next: Some(Box::new(n1)),
        };
        // Value used after being moved, 无法编译
        // let n3 = Node {
        //     value: 3,
        //     next: Some(Box::new(n1))
        // };

    }

    // We can use Rc::strong_count function to show the number of the reference count.
    // We change Box to Rc to hold Node type in our Node type definition.
    // When we create a variable we put a new Node instance in Rc::new(),
    // reference count is 1, then we put reference to a in Rc::clone function ,
    // increase number of reference count to 2. (Rc::clone not clone the data, only increase the number of reference count.)
    // So a and b variable can share the ownership .So data will not be cleaned up until the number of reference count is 0.
    #[test]
    fn test2() {
        let n1  = Rc::new(Node2 {
            value: 1,
            next: None
        });

        println!("count is {}", Rc::strong_count(&n1));

        let n2 = Node2 {
            value: 2,
            next: Some(Rc::clone(&n1)),
        };

        println!("count is {}", Rc::strong_count(&n1));


        let n2 = Node2 {
            value: 3,
            next: Some(Rc::clone(&n1)),
        };

        println!("count is {}", Rc::strong_count(&n1));



        // Value used after being moved, 无法编译
        // let n3 = Node {
        //     value: 3,
        //     next: Some(Box::new(n1))
        // };

    }
}