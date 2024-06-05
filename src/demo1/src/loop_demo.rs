#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let a = loop {
            break 10;
        };
        println!("{}", a);
    }

    #[test]
    fn test2() {
        let mut m = 1;
        let mut n = 1;
        'a: loop {
            if (m < 100) {
                m += 1;
            } else {
                'b: loop {
                    if (m + n < 150) {
                        n += 1;
                        break 'a;
                    } else {
                        continue 'a;
                    }
                }
            }
        }
        print!("m={}, n={}", m, n);
    }
}