#[derive(Debug)]
struct Foo {
    data: i32,
}

#[derive(Debug, Copy, Clone)]
struct Bar {
    count: i32,
}


#[derive(Debug, Clone)]
struct Tag {
    value: i32,
    names: Vec<String>,
}


impl Clone for Foo {
    fn clone(&self) -> Foo {
        Foo {
            data: self.data,
        }
    }
}

impl Copy for Foo {}

#[cfg(test)]
mod test {
    use crate::copy_demo::{Bar, Foo, Tag};

    #[test]
    fn test1() {
        let f = Foo { data: 10 };
        let f2 = f;
        println!("{:?}", f);
        println!("{:?}", f2);
    }

    #[test]
    fn test2() {
        let b = Bar { count: 20 };
        let bb = b;
        println!("{:?}", b);
        println!("{:?}", bb);
    }

    #[test]
    fn test3() {
        let p = Box::new(Tag { value: 30, names: vec![] });
        println!("{:?}", p.value);
        println!("{:?}", p.names);
    }
}