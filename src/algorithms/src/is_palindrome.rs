

/**
双指针
字符串
125. 验证回文串


题目描述
如果在将所有大写字符转换为小写字符、并移除所有非字母数字字符之后，短语正着读和反着读都一样。则可以认为该短语是一个 回文串 。

字母和数字都属于字母数字字符。

给你一个字符串 s，如果它是 回文串 ，返回 true ；否则，返回 false 。



示例 1：

输入: s = "A man, a plan, a canal: Panama"
输出：true
解释："amanaplanacanalpanama" 是回文串。
示例 2：

输入：s = "race a car"
输出：false
解释："raceacar" 不是回文串。
示例 3：

输入：s = " "
输出：true
解释：在移除非字母数字字符之后，s 是一个空字符串 "" 。
由于空字符串正着反着读都一样，所以是回文串。


提示：

1 <= s.length <= 2 * 105
s 仅由可打印的 ASCII 字符组成
**/
pub fn is_palindrome(s: String) -> bool {
    if s.is_empty() {
        return true;
    }
    let l_s = s.to_lowercase();
    let s_b = l_s.as_bytes();
    let n = s_b.len();
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        if !s_b[left].is_ascii_alphanumeric() {
            left += 1;
        } else if !s_b[right].is_ascii_alphanumeric() {
            right -= 1;
        } else {
            if s_b[left] != s_b[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

    }
    return true;

}

#[cfg(test)]
mod tests {
    use crate::is_palindrome::is_palindrome;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("A man, a plus one".to_string()), false);
        assert_eq!(is_palindrome("race a car".to_string()), false);
        assert_eq!(is_palindrome("race a car plus two".to_string()), false);
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }
}