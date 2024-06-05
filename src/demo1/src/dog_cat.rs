
pub struct Dog {
    pub(crate) name: String,
    pub(crate) age: i8,
}

pub struct Cat {
    pub(crate) lives: i8,
}

pub trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("Oh you're a cutie! What's your name ? {}", self.talk());
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

