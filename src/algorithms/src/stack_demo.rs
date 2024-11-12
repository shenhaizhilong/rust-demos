#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        println!("{}", is_valid(String::from("()")));
        println!("{}", is_valid(String::from("([])")));
        println!("{}", is_valid(String::from("([{}])")));
        println!("{}", is_valid(String::from("([{}])")));

        println!("{}", is_valid2(String::from("()")));
        println!("{}", is_valid2(String::from("([])")));
        println!("{}", is_valid2(String::from("([{}])")));
        println!("{}", is_valid2(String::from("([}])")));
    }

    fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut stack = vec![];
        while let Some(s) = chars.pop() {
            match s {
                '}' => stack.push('{'),
                ']' => stack.push('['),
                ')' => stack.push('('),
                _ => {
                    if stack.is_empty() || stack.pop().unwrap() != s {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }

    /**
    20. 有效的括号


    题目描述
    给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

    有效字符串需满足：

    左括号必须用相同类型的右括号闭合。
    左括号必须以正确的顺序闭合。
    每个右括号都有一个对应的相同类型的左括号。


    示例 1：

    输入：s = "()"

    输出：true

    示例 2：

    输入：s = "()[]{}"

    输出：true

    示例 3：

    输入：s = "(]"

    输出：false

    示例 4：

    输入：s = "([])"

    输出：true



    提示：

    1 <= s.length <= 104
    s 仅由括号 '()[]{}' 组成
    **/
    fn is_valid2(s: String) -> bool {
        if s.len() & 0x01 == 1 {
            return false;
        }
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                _ => {
                    if stack.is_empty() || stack.pop().unwrap() != c {
                        return false;
                    }
                }
            }
        }
        return stack.is_empty();
    }
}