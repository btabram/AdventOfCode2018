use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut double_count = 0;
    let mut triple_count = 0;
    let file = File::open("input.txt")?;
    for id in BufReader::new(file).lines() {

        let mut letter_count = HashMap::new();

        for letter in id?.chars() {
            let entry = letter_count.entry(letter).or_insert(0);
            *entry += 1;
        }
        println!("Letter occurances in ID: {:?}", letter_count);

        let mut id_has_double = false;
        let mut id_has_triple = false;
        for (_letter, count) in letter_count {

            if !id_has_double && count == 2 {
                println!("ID has a letter repeated 2 times!");
                id_has_double = true;
                double_count += 1;

            } else if !id_has_triple && count == 3 {
                println!("ID has a letter repeated 3 times!");
                id_has_triple = true;
                triple_count += 1;
            }
        }
    }

    let checksum = double_count * triple_count;
    println!("\nChecksum is {}", checksum);

    Ok(())
}
