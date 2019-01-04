use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut frequency = 0;
    let mut previous_frequencies = HashSet::new();

    while true {
        let file = File::open("input.txt")?;
        for line in BufReader::new(file).lines() {

            frequency += line?.parse::<i32>()?;
            println!("Frequency is {}", frequency);

            if previous_frequencies.contains(&frequency) {
                println!("First repeated frequency is {}", frequency);
                println!("{} frequencies reached before the first repeat!", previous_frequencies.len());
                return Ok(());
            } else {
                previous_frequencies.insert(frequency);
            }
        }
    }
    Ok(())
}
