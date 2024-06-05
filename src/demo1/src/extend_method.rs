trait Double {
    fn double(&self) -> Self;
}


/**
扩展方法
 **/
impl Double for i32 {
    fn double(&self) -> Self {
        return *self * 2;
    }
}

#[cfg(test)]
mod tests {
    use crate::extend_method::Double;

    #[test]
    fn test1() {
        let a = 2.double();
        println!("{}", a);
    }
}