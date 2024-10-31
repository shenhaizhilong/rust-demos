
/**
https://leetcode.com/problems/number-complement/

476. Number Complement

The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.

For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
Given an integer num, return its complement.

Example 1:

Input: num = 5
Output: 2
Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
Example 2:

Input: num = 1
Output: 0
Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
Intuition
Find mask and xor with number.

Approach
number     5 = 0000 0101
complement   = 0000 0010
Find mask (all on bits). Make last 3 numbers of mask as 0(ie left shift). 3 because number has last 3 digits which constitute the number.

mask         = 1111 1111
number     5 = 0000 0101
convert mask to
mask         = 1111 1000
Take complement of mask

mask         = 0000 0111
Take XOR of mask and number

number       = 0000 0101
mask         = 0000 0111
----------------------------XOR (same=0, different=1)
               0000 0010
Complexity
Time complexity: O(1)
Space complexity: O(1)
**/
pub fn find_complement(num: i32) -> i32 {
    // all 1
    let mut mask = !0;
    while (num & mask) > 0 {
        mask = mask << 1;
    }

    !mask ^ num
}

#[cfg(test)]
mod tests {
    use crate::find_complement::find_complement;

    #[test]
    fn test_find_complement() {
        assert_eq!(2, find_complement(5));
    }
}