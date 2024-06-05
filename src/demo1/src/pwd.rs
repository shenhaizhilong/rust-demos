use std::env;
use std::path::PathBuf;

fn get_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn print_working_dir() {
    let result = get_working_dir();
    match result {
        Ok(path_buf) => { println!("{}", path_buf.display()); }
        Err(e) => { eprintln!("Application Error: {}", e); }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::pwd::print_working_dir;

    #[test]
    fn test1() {
        print_working_dir();
    }
}