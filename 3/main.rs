use std::env;
use std::fs;

#[derive(Debug)]
struct Claim {
    id: usize,
    x_offset: usize,
    y_offset: usize,
    x_len: usize,
    y_len: usize,
}

// Convert a &str to a u32 whilst removing any unwanted characters
fn get_usize(s: &str) -> usize {
    let mut local_s = s.to_string();
    local_s.retain(|c| c != '#' && c != ':');
    return local_s.parse().expect("Failed to parse string as usize");
}

fn make_claim(input_line: &str) -> Claim {
    let split_input: Vec<String> = input_line.to_string().split(" ").map(|s| s.to_string()).collect();
    assert!(split_input.len() == 4);

    let mut claim = Claim { id: 0, x_offset: 0, y_offset: 0, x_len: 0, y_len: 0 };

    for (i, s) in split_input.iter().enumerate() {
        match i {
            // Parse the ID
            0 => { 
                claim.id = get_usize(s);
            },
            // Extract the x and y offsets
            2 => {
                let offsets: Vec<usize> = s.split(",").map(get_usize).collect();
                claim.x_offset = offsets[0];
                claim.y_offset = offsets[1];
            },
            // Extract the x and y lengths
            3 => {
                let lengths: Vec<usize> = s.split("x").map(get_usize).collect();
                claim.x_len = lengths[0];
                claim.y_len = lengths[1];
            },
            // Discard the '@' 
            _ => {},
        };
    }
    return claim;
}

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let mut part2 = false;
    if args.len() > 1 {
        part2 = true;
    }

    let input = fs::read_to_string("input.txt")?;
    let mut grid = vec![vec![0usize; 1000]; 1000];
    let claims: Vec<Claim> = input.lines().map(make_claim).collect();

    for claim in &claims {
        println!("{:?}", claim); 

        let x1 = claim.x_offset;
        let x2 = claim.x_offset + claim.x_len;
        let y1 = claim.y_offset;
        let y2 = claim.y_offset + claim.y_len;

        for vec in &mut grid[x1..x2] {
            for point in &mut vec[y1..y2] {
                *point += 1;
            }
        }
    }

    let mut two_or_more_count = 0;
    for vec in &grid {
        two_or_more_count += vec.iter().filter(|p| **p > 1).count();
    }

    println!("\nThere are {} squares within two or more claims\n", two_or_more_count);

    if part2 {
        // Find the claim which does not overlap with any other
        'claim_loop: for claim in &claims {
            let x1 = claim.x_offset;
            let x2 = claim.x_offset + claim.x_len;
            let y1 = claim.y_offset;
            let y2 = claim.y_offset + claim.y_len;

            for vec in &grid[x1..x2] {
                for point in &vec[y1..y2] {
                    if *point > 1 {
                        // There must be overlap so skip this claim
                        continue 'claim_loop;
                    }
                }
            }

            println!("The claim with no overlap has ID {}: {:?}", claim.id, claim);
            return Ok(());
        }
    }

    Ok(())
}
