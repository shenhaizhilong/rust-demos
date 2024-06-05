use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long)]
    file_name: String,
    #[arg(long)]
    out_put: String,
}

#[cfg(test)]
mod clap_demo {
    use clap::Parser;
    use crate::clap_demo1::Cli;

    #[test]
    fn test1() {
        let cli = Cli::parse();
        println!("one:{:?}", cli.file_name);
        println!("out_put:{:?}", cli.out_put);
    }
}
