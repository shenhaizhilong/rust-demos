pub fn add_two(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, add_two(2, 0));
    }
}
