#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        //We can use take method on an Option<T> value,
        // it will take the value out of the option,
        // leaving a None in its place.
        let mut a = Some(5);
        let b = a.take();
        println!("{a:?}");
        println!("{b:?}");
    }
}