use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut total = 0;
    let file = File::open("input.txt")?;
    for line in BufReader::new(file).lines() {
        let num = line?.parse::<i32>()?;
        println!("Frequency chage of {}", num);
        total += num;
    }
    println!("\nResulting frequency = {}", total);
    Ok(())
}
