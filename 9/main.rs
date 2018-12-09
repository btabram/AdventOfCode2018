use std::collections::{VecDeque, HashMap};

type ErrorHolder = Box<std::error::Error>;

fn play_game(players: usize, max_marble: usize) -> HashMap<usize, usize> {
    // Use VecDeque for more efficient insertions at both ends than Vec
    let mut marbles = VecDeque::with_capacity(max_marble);
    marbles.push_back(0);

    let mut scores = HashMap::new();
    let mut current_marble = 0;
    let mut current_player = 1;

    for m in 1..=max_marble {

        if m % 23 == 0 {
            // Add the marble that would have been placed to the players score
            let current_player_score = scores.entry(current_player).or_insert(0);
            *current_player_score += m;

            // Also remove the marbles 7 marbles councter-clockwise from the current marble
            // and add it to the players score
            let remove_index;
            match current_marble {
                cm if cm >= 7 => remove_index = cm - 7,
                cm => remove_index = cm + marbles.len() - 7,
            }
            *current_player_score += marbles.remove(remove_index).unwrap();

            // Marble next to the removed marble becomes the current marble
            current_marble = remove_index;
        }
        else {
            // Insert the next marble 2 places clockwise from the current marble
            // The new marble becomes current marble
            current_marble += 2;
            match current_marble {
                i if i == marbles.len() => marbles.push_back(m),
                i if i > marbles.len() => {
                    current_marble -= marbles.len();
                    marbles.insert(current_marble, m);
                },
                _ => marbles.insert(current_marble, m),
            }
        }

        // PLayers are numbered 1..=players and simply take turns
        match (current_player + 1) % players {
            0 => current_player = players,
            x => current_player = x,
        }
    }
    scores
}

fn main() -> Result<(), ErrorHolder> {
    let players = 465;
    let max_marble = 71940;

    let part1_scores = play_game(players, max_marble);
    let max_score_1 = part1_scores.values().max().unwrap();
    println!("The max score the game in part 1 is {}\n", max_score_1);

    // Part 2 really requires a linked list for it to be quick since insertions into the middle
    // of the vector involve moving all subsequent elements and really slow things down.
    // Unfortunately, I haven't done this so there's quite a wait for part 2...
    println!("Part 2 will take some time to calculate...");

    let part2_scores = play_game(players, 100*max_marble);
    let max_score_2 = part2_scores.values().max().unwrap();
    println!("The max score the game in part 2 is {}", max_score_2);

    Ok(())
}

