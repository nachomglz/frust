use std::io;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = io::stdin().lines();

    while let Some(line) = lines.next() {
        let li = line.unwrap();

        println!("{}", li);
    }

    Ok(())
}
