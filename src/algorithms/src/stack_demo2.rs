#[cfg(test)]
mod test {
    #[test]
    fn test() {
        println!("{}", del_duplicate_chars(String::from("abbaca")));
    }

    fn del_duplicate_chars(s: String) -> String {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            if !stack.is_empty() && stack[stack.len() - 1] == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        return stack.into_iter().collect();
    }
}