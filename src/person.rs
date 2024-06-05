use std::fmt::{Debug, Display, format};

struct Person {
    name: String,
    age: u8,
}

trait Descriptive {
    fn describe(&self) -> String;
}

trait Print {
    fn print_function(&self) -> String;
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        return format!("{}", &self.name);
    }
}

impl Print for Person {
    fn print_function(&self) -> String {
        format!("{}", &self.age)
    }
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone, U: Clone + Debug {
    return 10;
}

fn largest<T>(list: &[T]) -> T where T: PartialOrd + Copy {
    let mut ans = list[0];
    for &i in list {
        if i > ans {
            ans = i;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::person::{Descriptive, largest, Person, Print};

    #[test]
    fn test1() {
        let p = Person {
            name: String::from("tom"),
            age: 10,
        };

        fn output(obj: impl Print + Descriptive) -> String {
            format!("{}, {}", obj.describe(), obj.print_function())
        }

        println!("{}", output(p));
    }

    #[test]
    fn test2() {
        let number_list = vec![34, 50, 21, 100, 44];
        let result = largest(&number_list);
        println!("{}", result);

        let number_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&number_list);
        println!("{}", result);
    }

}