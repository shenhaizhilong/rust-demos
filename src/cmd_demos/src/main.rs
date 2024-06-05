use std::process::Command;
use error_chain::error_chain;
use regex::Regex;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Commit {
    hash: String,
    message: String,
}

fn main() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output().unwrap();
    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(?x)([0-9a-fA-F]+)(.*)").unwrap();
    let output_str = String::from_utf8(output.stdout).unwrap();
    output_str
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            }
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));


    Ok(())
}
