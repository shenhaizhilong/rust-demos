
/**

Finding Number is power of 2 using bitwise Operator

Given an integer n, return true if it is a power of two. Otherwise, return false.

An integer n is a power of two, if there exists an integer x such that n == 2x.

Example 1:

Input: n = 1
Output: true
Explanation: 20 = 1
Example 2:

Input: n = 16
Output: true
Explanation: 24 = 16
Example 3:

Input: n = 3
Output: false
Logic
Use Bitwise operator to check power of 2
if n is power of 2, then n&(n-1) is 0
Example1(n is power of 2):
n = 16. Bitwise Representation = 10000
n-1=15. Bitwise Representation = 01111
                                &
Applying & on n and n-1 -------------------
                                 00000  //1&0=0, 0&1=0

Example2(n is not power of 2):
n = 5. Bitwise Representation = 101
n-1=4. Bitwise Representation = 100
                                &
Applying & on n and n-1 -------------------
                                100  //1&0=0, 0&1=0
**/
pub fn is_power_of_two(n: i32) -> bool {
    let n1 = n;
    let n2 = n - 1;
    if n == 0 {
        return false;
    }
    return n1 & n2 == 0;
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_is_power_of_two() {
        assert!(super::is_power_of_two(1));
        assert!(super::is_power_of_two(2));
        assert!(!super::is_power_of_two(3));
        assert!(super::is_power_of_two(4));
    }
}
