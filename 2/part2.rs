use std::fs;

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut correct_id = String::new();
    let mut diff_index: usize = 0;

    'main_loop: for id in input.lines() {
        'other_id_loop: for other_id in input.lines() {
            // Don't compare an ID with itself.
            if id == other_id {
                continue;
            }

            let mut had_one_diff = false;
            for (i, (letter, other_letter)) in id.chars().zip(other_id.chars()).enumerate() {
                if letter != other_letter {
                    if had_one_diff {
                        // We've now got two differing letters so [other_id] is not a suitable
                        // match for [id]. Hence we continue and try the next id.
                        continue 'other_id_loop;
                    }
                    had_one_diff = true;
                    diff_index = i;
                }
            }

            println!("ID {} has only one difference form other ID {}", id, other_id);
            correct_id = id.to_string();
            break 'main_loop;
        }
    }

    correct_id.remove(diff_index);
    println!("The common letters between the correct box IDs are \"{}\"", correct_id);

    Ok(())
}
