use std::ffi::{c_char, CString};
use std::fmt::Error;
use std::io::Write;

extern {
    fn hello();
    fn greet(name: *const c_char);
}

fn prompt(s: &str) -> Result<String, std::io::Error> {
    println!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn main() -> Result<(), std::io::Error> {
    unsafe {
        hello();
    }

    let name = prompt("What's your name ?")?;
    let c_name = CString::new(name)?;
    unsafe {
        greet(c_name.as_ptr());
    }
    Ok(())
}
