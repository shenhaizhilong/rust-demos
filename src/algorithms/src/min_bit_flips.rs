
/**

* https://leetcode.com/problems/minimum-bit-flips-to-convert-number/submissions/1438035720/
 * A bit flip of a number x is choosing a bit in the binary representation of x and flipping it from either 0 to 1 or 1 to 0.
 * <p>
 * For example, for x = 7, the binary representation is 111 and we may choose any bit (including any leading zeros not shown) and flip it. We can flip the first bit from the right to get 110, flip the second bit from the right to get 101, flip the fifth bit from the right (a leading zero) to get 10111, etc.
 * Given two integers start and goal, return the minimum number of bit flips to convert start to goal.
 * <p>
 * Example 1:
 * <p>
 * Input: start = 10, goal = 7
 * Output: 3
 * Explanation: The binary representation of 10 and 7 are 1010 and 0111 respectively. We can convert 10 to 7 in 3 steps:
 * - Flip the first bit from the right: 1010 -> 1011.
 * - Flip the third bit from the right: 1011 -> 1111.
 * - Flip the fourth bit from the right: 1111 -> 0111.
 * It can be shown we cannot convert 10 to 7 in less than 3 steps. Hence, we return 3.
 * Example 2:
 * <p>
 * Input: start = 3, goal = 4
 * Output: 3
 * Explanation: The binary representation of 3 and 4 are 011 and 100 respectively. We can convert 3 to 4 in 3 steps:
 * - Flip the first bit from the right: 011 -> 010.
 * - Flip the second bit from the right: 010 -> 000.
 * - Flip the third bit from the right: 000 -> 100.
 * It can be shown we cannot convert 3 to 4 in less than 3 steps. Hence, we return 3.
 * Approach. Iterative. O(1)
 * Logic
 * Compare every bit of start and goal
 * if bits differ, increment the count.
 * start  = 1 0 1 0    //10
 * goal   = 0 1 1 1    //7
 * 3 2 1 0
 * Bits at positions(0,2,3) differ hence answer=3
 * Take a = 1, Perform bitwise AND operation on start and goal if numbers are same, increment count. Keep left shifting a by 1.
 * a = 1
 * start  = 1 0 1 0
 * &   0 0 0 1
 * 0 0 0 0    //x
 * <p>
 * goal   = 0 1 1 1
 * &   0 0 0 1
 * 0 0 0 1    //y
 * <p>
 * if (x == y)
 * count++        // count=0
 * <p>
 * ---------- Left shift a by 1 ---------
 * a = 2
 * start  = 1 0 1 0
 * a   &   0 0 1 0
 * 0 0 1 0    //x
 * <p>
 * goal   = 0 1 1 1
 * a  &   0 0 1 0
 * 0 0 1 0    //y
 * <p>
 * if (x == y)    // Yes
 * count++
 *
 * @author shenhaizhilong
 * @date 2024/10/30 16:01
 */
pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut mask = 1;
    while i < 32 {
        if (start & mask) != (goal & mask) {
            ans += 1;
        }
        i += 1;
        mask = mask << 1;
    }
    return ans;
}

#[cfg(test)]
mod tests {
    use crate::min_bit_flips::min_bit_flips;

    #[test]
    fn test_i32() {
        assert_eq!(3, min_bit_flips(10, 7));
        assert_eq!(3, min_bit_flips(3, 4));
    }
}