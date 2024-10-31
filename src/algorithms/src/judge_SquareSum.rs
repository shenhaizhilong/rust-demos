

/**

https://leetcode.com/problems/sum-of-square-numbers/
633. Sum of Square Numbers
Given a non-negative integer c, decide whether there're two integers a and b such that a2 + b2 = c.

Example 1:

Input: c = 5
Output: true
Explanation: 1 * 1 + 2 * 2 = 5
Example 2:

Input: c = 3
Output: false
Constraints:

0 <= c <= 231 - 1
**/
pub fn judge_square_sum(c: i32) -> bool {
    let mut left = 0;
    let mut right = (c as f64).sqrt() as i32;
    while left <= right {
        let sum = left * left + right * right;
        if (sum < c) {
            left += 1;
        } else if (sum > c) {
            right -= 1;
        } else {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_judge_square_sum() {
        assert!(super::judge_square_sum(4));
        assert!(!super::judge_square_sum(3));
        assert!(super::judge_square_sum(5));
    }
}