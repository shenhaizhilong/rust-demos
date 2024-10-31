
pub fn climb_stairs(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    let n = n as usize;
    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[1] = 1;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    return dp[n] as i32;
}


#[cfg(test)]
mod tests {
    use crate::climb_stairs::climb_stairs;

    #[test]
    fn test() {
        let size = 10;
        let mut arr: Vec<i32> = vec![0; size];
        arr.fill(1);

        // 访问 arr[0]
        let first_element = arr[0];

        println!("First element: {}", first_element);
    }

    #[test]
    fn test2() {
        for i in 0..10 {
            println!("{i}={}", climb_stairs(i));
        }
    }
}