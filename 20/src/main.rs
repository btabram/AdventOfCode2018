use std::fs;
use std::collections::HashMap;

type ErrorHolder = Box<std::error::Error>;
type Pos = (i32, i32);

fn update_pos(pos: Pos, direction: char) -> Pos {
    let (x, y) = pos;
    match direction {
        'N' => (x, y + 1),
        'E' => (x + 1, y),
        'S' => (x, y - 1),
        'W' => (x - 1, y),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Regex {
    chars: Vec<char>,
    visited: HashMap<Pos, usize>,
}

impl Regex {
    // Making an assumption that the paths do not rejoin once the regex branches
    fn parse_branch(&mut self, i: &mut usize, length_so_far: usize, p: Pos) {

        assert_eq!(self.chars[*i], '(');
        *i += 1;

        let mut lengths = vec![];
        // loop over whole branching pattern of the regex
        'outer: loop {

            // Track the curent room coordinates so we know if we've already
            // visited this room or not. We only care about the shortest path
            // to a given room
            let mut pos = p;

            let mut length = 0;
            // loop through a particular branch
            loop {
                let c = self.chars[*i] ;
                match c {
                    'N' | 'E' | 'S' | 'W' => {
                        length += 1;
                        *i += 1;

                        pos = update_pos(pos, c);
                        let cur_len = length + length_so_far;

                        let prev_value = self.visited.insert(pos, cur_len);
                        // If we've already visited this room them ensure
                        // we remember the shortest path in the visited map
                        if prev_value != None {
                            self.visited.insert(pos,
                                std::cmp::min(cur_len, prev_value.unwrap()));
                        }
                    },
                    // This particular branch is finished
                    '|' => {
                        lengths.push(length);
                        *i += 1;
                        continue 'outer;
                    },
                    // This whole branching pattern in the regex is finished
                    ')' => {
                        lengths.push(length);
                        *i += 1;
                        break 'outer;
                    }
                    '(' => {
                        self.parse_branch(i, length_so_far + length, pos);
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    fn parse(&mut self) {
        assert_eq!(self.chars[0], '^');

        let mut i = 1;

        // Track the curent room coordinates and length of current path (number
        // of doors opened) so we know if we've already visited this room or
        // not. We only care about the shortest path to a given room
        let mut pos = (0, 0);
        let mut length = 0;

        self.visited.insert(pos, length);

        loop {
            let c = self.chars[i];
            match c {
                'N' | 'E' | 'S' | 'W' => {
                    length += 1;
                    i += 1;

                    pos = update_pos(pos, c);
                    self.visited.insert(pos, length);
                },
                '(' => {
                    self.parse_branch(&mut i, length, pos);
                },
                '$' => break,
                _ => unreachable!(),
            }
        }
    }
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    let chars: Vec<char> = input.trim().chars().collect();
    let mut regex = Regex { chars, visited: HashMap::new() };

    regex.parse();

    let part1 = regex.visited.values().max().expect("Failed to find max");
    println!("\nThe longest path to a room is {}.", part1);

    let limit = 1000;
    let part2 = regex.visited.values().filter(|&&l| l >= limit).count();
    println!("There are {} rooms with a shortest path of at least {}.",
             part2, limit);

    Ok(())
}
