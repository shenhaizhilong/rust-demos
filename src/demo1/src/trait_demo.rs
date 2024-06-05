trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook::start");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash::start")
    }
}

#[cfg(test)]
mod tests {
    use crate::trait_demo::{Chef, Cook, Wash};

    #[test]
    fn test1() {
        let me = Chef;
        <dyn Cook>::start(&me);
        <dyn Wash>::start(&me);
    }
}