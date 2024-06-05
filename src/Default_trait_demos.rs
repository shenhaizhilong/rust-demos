/**
https://medium.com/@mikecode/rust-default-trait-63000d3616e2
**/

#[derive(Debug, Default)]
struct StructC {
    name: String,
    age: u8,
}


#[derive(Debug)]
struct StructD {
    name: String,
    age: u8,
}

impl Default for StructD {
    fn default() -> Self {
        StructD { name: "tom".to_string(), age: 10 }
    }
}

#[cfg(test)]
mod tests {
    use crate::Default_trait_demos::{StructC, StructD};

    #[test]
    fn test1() {
        let a: StructC = Default::default();
        println!("{:?}", a);
        let b: StructD = Default::default();
        println!("{:?}", b);
    }
}