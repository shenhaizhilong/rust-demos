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