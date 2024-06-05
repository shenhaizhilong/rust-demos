use std::fs;

pub fn read_to_str(path: &str) {
    let content = fs::read_to_string(path).unwrap();
    println!("{:?} content:{:?}", path, content);
}


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::file_demo1::read_to_str;

    #[test]
    fn test1() {
        read_to_str("/Users/shenlong/RustroverProjects/demos/src/file_test/src/foo2.txt");
    }

    #[test]
    fn read_line_with_buffer() {
        let file = File::open("/Users/shenlong/RustroverProjects/demos/src/file_test/src/foo2.txt").unwrap();
        let buff_reader = BufReader::new(file);
        for line in buff_reader.lines() {
            match line {
                Ok(curr_line) => {
                    println!("{}", curr_line);
                }
                Err(err) => {}
            }
        }
    }
}