fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct定义中的生命周期标注
// 我们前面学struct的时候，都是自持有类型（比如：i32，String）
// 如果struct字段是引用类型，需要添加生命周期标注
struct ImportantExcerpt<'a> {
    part: &'a str
}

#[cfg(test)]
mod tests {
    use crate::lifetime_demos::ImportantExcerpt;

    #[test]
    fn test2() {
        let string1 = "abcd";
        let mut result = "";
        {
            // `string2` does not live long enough
            // let string2 = String::from("xyz");
            // result = super::longest(string1, string2.as_str());
        }
        println!("The longest string is {}", result);
    }

    #[test]
    fn test3() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        print!("{}", first_sentence);
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}