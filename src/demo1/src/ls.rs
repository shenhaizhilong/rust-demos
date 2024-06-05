use std::{fs, process};
use std::path::Path;

/**
  LS command in linux
**/
pub struct LsCommand<'a> {
    pub path: &'a Vec<String>,
}

impl LsCommand<'_> {
    fn new(path: &Vec<String>) -> LsCommand {
        LsCommand { path }
    }

    pub fn execute(&self) {
        if self.path.len() == 0 {
            let dir = fs::read_dir("./").unwrap();
            for item in dir {
                println!("{}", item.unwrap().path().display());
            }
            return;
        }

        for path in self.path {
            if !Path::new(path).is_dir() {
                println!("not a dir:{}", path);
                continue;
            }
            let dir = fs::read_dir(path).unwrap_or_else(|e| {
                eprintln!("Application Error: {}", e);
                process::exit(1);
            });

            println!("Files in {}:", path);
            for item in dir {
                println!("{}", item.unwrap().path().display());
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ls::LsCommand;

    #[test]
    fn test1() {
        let paths = vec!["/Users/shenlong/RustroverProjects/demo1".to_string(), "/Users/shenlong/RustroverProjects".to_string(), "/Users/shenlong/RustroverProjects/demo1/Cargo.toml".to_string()];
        let ls_command = LsCommand::new(&paths);
        ls_command.execute();
    }
}