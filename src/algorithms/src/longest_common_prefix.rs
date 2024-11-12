/**
14. 最长公共前缀


题目描述
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。



示例 1：

输入：strs = ["flower","flow","flight"]
输出："fl"
示例 2：

输入：strs = ["dog","racecar","car"]
输出：""
解释：输入不存在公共前缀。


提示：

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] 仅由小写英文字母组成

**/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let mut str_chars = strs.iter().map(|s| s.as_bytes().to_vec()).collect::<Vec<_>>();
    let mut min_len = str_chars.iter().map(|s| s.len()).min().unwrap();
    for i in 0..min_len {
        for s in &str_chars {
            if (s[i] != str_chars[0][i]) {
                return String::from_utf8(str_chars[0][0..i].to_vec()).unwrap();
            }
        }
    }
    return strs[0].to_string();
}

pub fn longest_common_prefix2(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    // 获取每个字符串的 Vec<char>
    let mut str_chars = strs.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    // 获取所有字符串 vec<char> 数量的最小值
    let mut min_len = str_chars.iter().map(|s| s.len()).min().unwrap();
    for i in 0..min_len {
        // 遍历所有的字符串
        for s in &str_chars {
            // 对比每个字符串与base的第i个字符
            if (s[i] != str_chars[0][i]) {
                return str_chars[0][0..i].iter().collect::<String>();
            }
        }
    }
    return strs[0].to_string();
}
#[cfg(test)]
mod tests {
    use crate::longest_common_prefix::{longest_common_prefix2, longest_common_prefix};

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(vec![]), "".to_string());
        assert_eq!(longest_common_prefix(vec![String::from("a")]), "a".to_string());
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
        assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string());
    }

    #[test]
    fn test_longest_common_prefix2() {
        assert_eq!(longest_common_prefix2(vec![]), "".to_string());
        assert_eq!(longest_common_prefix2(vec![String::from("a")]), "a".to_string());
        assert_eq!(longest_common_prefix2(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
        assert_eq!(longest_common_prefix2(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string());
    }
}