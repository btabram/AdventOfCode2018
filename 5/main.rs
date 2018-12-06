use std::fs;

fn to_polarity_tuple(c: &char) -> (char, bool) {
    let upper = c.is_uppercase();
    let lower = c.is_lowercase();
    assert!(lower != upper);
    
    (c.to_ascii_lowercase(), upper)
}

fn react_polymer(units: &mut Vec<(char, bool)>) -> usize {
    loop {
        let mut prev = None;  
        let mut reaction_index = None;

        for (i, &unit) in units.iter().enumerate() {
            let matching_unit = Some((unit.0, !unit.1));
            if prev == matching_unit {
                reaction_index = Some(i);
                break;
            }
            prev = Some(unit);
        }

        if reaction_index.is_some() {
            for _ in 0..2 {
                // Remove the prev unit and one after it which is initially at reaction_index
                units.remove(reaction_index.unwrap() - 1);
            }
        } else {
            // If we don't have a reaction_index then we've finished reacting
            break;
        }
    }
    units.len()
}

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let chars: Vec<_> = input.chars().filter(|&c| c != '\n' && c != '\r').collect();
    let units: Vec<_> = chars.iter().map(to_polarity_tuple).collect();

    // Part 1
    println!("The initial length of the polymer is {}", units.len());
    println!("The length of the reacted polymer is {}\n", react_polymer(&mut units.clone()));

    // Part 2
    let mut unique_chars: Vec<_> = chars.iter().map(|c| c.to_ascii_lowercase()).collect();
    unique_chars.sort_unstable();
    unique_chars.dedup();

    let mut min_len = 50000;
    let mut best_to_remove = None;
    for u in unique_chars {
        let mut reduced_polymer: Vec<_> = units.iter().map(|x| *x).filter(|(c, _)| *c != u).collect();
        let reacted_len = react_polymer(&mut reduced_polymer);
        println!("The length of the reacted polymer after removing {} is {}", u, reacted_len);

        if reacted_len < min_len {
            min_len = reacted_len;
            best_to_remove = Some(u);
        }
    }

    match best_to_remove {
        Some(u) => println!("\nThe minimum reacted length is {} after removing {}", min_len, u),
        None => { println!("\nFailed to find the best unit to remove!")},
    }

    Ok(())
}
