/**
https://leetcode.com/problems/sum-of-digits-of-string-after-convert/


You are given a string s consisting of lowercase English letters, and an integer k.

First, convert s into an integer by replacing each letter with its position in the alphabet (i.e., replace 'a' with 1, 'b' with 2, ..., 'z' with 26). Then, transform the integer by replacing it with the sum of its digits. Repeat the transform operation k times in total.

For example, if s = "zbax" and k = 2, then the resulting integer would be 8 by the following operations:

Convert: "zbax" ➝ "(26)(2)(1)(24)" ➝ "262124" ➝ 262124
Transform #1: 262124 ➝ 2 + 6 + 2 + 1 + 2 + 4 ➝ 17
Transform #2: 17 ➝ 1 + 7 ➝ 8
Return the resulting integer after performing the operations described above.

Example 1:

Input: s = "iiii", k = 1
Output: 36
Explanation: The operations are as follows:
- Convert: "iiii" ➝ "(9)(9)(9)(9)" ➝ "9999" ➝ 9999
- Transform #1: 9999 ➝ 9 + 9 + 9 + 9 ➝ 36
Thus the resulting integer is 36.
Example 2:

Input: s = "leetcode", k = 2
Output: 6
Explanation: The operations are as follows:
- Convert: "leetcode" ➝ "(12)(5)(5)(20)(3)(15)(4)(5)" ➝ "12552031545" ➝ 12552031545
- Transform #1: 12552031545 ➝ 1 + 2 + 5 + 5 + 2 + 0 + 3 + 1 + 5 + 4 + 5 ➝ 33
- Transform #2: 33 ➝ 3 + 3 ➝ 6
Thus the resulting integer is 6.
Example 3:

Input: s = "zbax", k = 2
Output: 8
Approach. O(n)
Logic
Convert all characters of string to corresponding int.
"leetcode" => "12552031545"
Sum all digits of converted string
1 + 2 + 5 + 5 + 2 + 0 + 3 + 1 + 5 + 4 + 5 ➝ 33
Keep adding digits to create a number k-1 times
Complexity
Time: O(n)
Space: O(n
**/

pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut sum: i32 = 0;
    for c in s.chars() {
        sum = sum * 10 + (c as u8 - 'a' as u8 + 1) as i32;
    }

    for _i in 1..=k {
        sum = sum_all_digits(sum);
    }

    return sum;
}


pub fn sum_all_digits(num: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut t = num;
    while t > 0 {
        sum += t % 10;
        t /= 10;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::get_lucky::get_lucky;

    #[test]
    fn test_get_lucky() {
        assert_eq!(get_lucky(String::from("iiii"), 1), 36);
        assert_eq!(get_lucky(String::from("leetcode"), 2), 6);
        assert_eq!(get_lucky(String::from("zbax"), 2), 8);
    }
}