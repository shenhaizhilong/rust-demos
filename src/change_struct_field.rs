use std::cell::RefCell;

#[derive(Debug)]
pub struct Demo {
    pub(crate) name: String,
    pub(crate) email: RefCell<String>,
}

impl Demo {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[cfg(unix)]
fn my_unix_function() {
    // This code will only be compiled for Unix-like systems
    println!("Hello from Unix!");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::change_struct_field::{Demo, my_unix_function};

    #[test]
    fn test1() {
        let mut a = Demo { name: "tom".into(), email: RefCell::new("tom@126.com".into()) };
        a.set_name("alice");
        a.email.replace("alice@126.com".into());
        println!("{:?}", a.name);
        println!("{:?}", a.email);
        a.name = "kk".to_string();
        println!("{:?}", a.name);

        my_unix_function(); // This will call the function if it's compiled
    }
}