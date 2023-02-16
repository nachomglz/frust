use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    #[arg()]
    search_value: String,
    #[arg(long = "case-sensitive", short)]
    case_sensitive: bool,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut lines = io::stdin().lines();

    while let Some(line) = lines.next() {
        let li = line.unwrap();

        let slot;
        let value: &str = if args.case_sensitive {
            &args.search_value
        } else {
            slot = args.search_value.to_lowercase();
            &slot
        };

        if li.contains(&value) {
            println!("{}", li);
        }
    }

    Ok(())
}
