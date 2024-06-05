#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
pub struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vector_sort::Person;

    #[test]
    fn test1() {
        let mut v1 = vec![1, 3, 2, 0, 9];
        v1.sort();
        println!("v1: {:?}", v1);
        assert_eq!(v1, vec![0, 1, 2, 3, 9]);
    }

    #[test]
    fn test2() {
        let mut v1 = vec![1.0, 0.0, 3.1, 2.9, 9.1];
        v1.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("v1: {:?}", v1);
        assert_eq!(v1, vec![0.0, 1.0, 2.9, 3.1, 9.1]);
    }

    #[test]
    fn test_sort() {
        let mut v1 = vec![
            Person::new("tom".to_string(), 10),
            Person::new("alice".to_string(), 30),
            Person::new("jack".to_string(), 11),
        ];
        // sort by derived natural order (name, age)
        v1.sort();

        println!("{:?}", v1);
    }
    #[test]
    fn test_sort_by() {
        let mut v1 = vec![
            Person::new("tom".to_string(), 10),
            Person::new("alice".to_string(), 30),
            Person::new("jack".to_string(), 11),
        ];

        v1.sort_by(|a, b| b.age.cmp(&a.age));

        println!("{:?}", v1);
    }
}