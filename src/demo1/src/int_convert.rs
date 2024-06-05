#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let i = 42;
        let mut p: *mut i32 = &i as *const i32 as *mut i32;
        // unsafe { *p = *p + 1; }
        println!("{:?}", p);
        unsafe { println!("{:?}", *p); }
        println!("{}", i);
    }
}