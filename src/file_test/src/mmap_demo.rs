use std::error::Error;
use std::fs::File;
use std::io;
use memmap::Mmap;


pub fn count_newlines_mem_map(filename: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mmap = unsafe {
        Mmap::map(&file)?
    };
    let count = mmap.iter().filter(|&&n| n == b'\n').count() + 1;

    return Ok(count);
}

#[cfg(test)]
mod tests {
    use crate::mmap_demo;

    #[test]
    fn test() {
        let v = mmap_demo::count_newlines_mem_map("/Users/shenlong/RustroverProjects/demos/src/file_test/src/foo2.txt");
        match v {
            Ok(c) => {
                println!("lines: {:?}", c);
            }
            Err(msg) => {
                println!("err: {:?}", msg);
            }
        }
    }
}

