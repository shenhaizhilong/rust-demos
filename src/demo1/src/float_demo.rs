#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let inf = f32::INFINITY;
        println!("{}, {}, {}", inf * 0.0, 1.0 / inf, inf / inf);
    }

    #[test]
    fn test2() {

        // 因为nan  的存在浮点数是不具备全序关系的（total order）
        let nan = f32::NAN;
        println!("{} {} {}", nan < nan, nan > nan, nan == nan);
    }
}