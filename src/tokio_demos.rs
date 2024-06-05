// use tokio::io;
// use tokio::fs::File;
// use tokio::io::AsyncReadExt;
//
//
// #[tokio::main]
// pub async fn main() -> io::Result<()> {
//     let mut f: File = File::open("/Users/shenlong/RustroverProjects/demos/foo.txt").await?;
//     let mut buffer = vec![];
//     // read the whole file
//     f.read_to_end(&mut buffer).await?;
//     println!("buffer: {:?}", buffer);
//     Ok(())
// }

