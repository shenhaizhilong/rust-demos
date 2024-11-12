/**

22. 括号生成
https://leetcode.cn/problems/generate-parentheses

题目描述
数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。



示例 1：

输入：n = 3
输出：["((()))","(()())","(())()","()(())","()()()"]
示例 2：

输入：n = 1
输出：["()"]


提示：

1 <= n <= 8

**/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    dfs(0, 0, n, &mut result, "".to_string());
    return result;
}

pub fn dfs(left: i32, right: i32, n: i32, result: &mut Vec<String>, path: String) {
    if (left > n || right > n || right > left) {
        return;
    }
    if (left == right && left == n) {
        result.push(path);
        return;
    }
    // 添加左括号
    dfs(left + 1, right, n, result, path.clone() + "(");
    // 添加有括号
    dfs(left, right + 1, n, result, path.clone() + ")");
}

#[cfg(test)]
mod tests {
    use crate::generate_parenthesis::generate_parenthesis;

    #[test]
    fn test_generate_parenthesis() {
        let result = generate_parenthesis(3);
        assert_eq!(result.len(), 5);
        println!("{:?}", result);

    }
}