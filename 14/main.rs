use std::fs;
use std::collections::VecDeque;

type ErrorHolder = Box<std::error::Error>;

macro_rules! unexpected {
    ($x:expr) => {{
        println!("Error! Unexpected {}", $x);
        std::process::exit(1);
    }};
}

#[derive(Debug)]
struct Elf {
    pos: usize,
}

#[derive(Debug)]
struct RecipeBoard {
    recipes: Vec<usize>,
    elves: Vec<Elf>,
}

impl std::fmt::Display for RecipeBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let elf1_pos = self.elves[0].pos;
        let elf2_pos = self.elves[1].pos;
        assert!(elf1_pos != elf2_pos);

        let mut recipe_string = String::new();

        for (i, r) in self.recipes.iter().enumerate() {
            match i {
                _ if i == elf1_pos => recipe_string.push_str("("),
                _ if i == elf2_pos => recipe_string.push_str("["),
                _ => recipe_string.push_str(" "),
            }
            recipe_string.push_str(&r.to_string());
            match i {
                _ if i == elf1_pos => recipe_string.push_str(")"),
                _ if i == elf2_pos => recipe_string.push_str("]"),
                _ => recipe_string.push_str(" "),
            }
        }
        write!(f, "{}", recipe_string)
    }
}

impl RecipeBoard {
    fn new() -> RecipeBoard {
        RecipeBoard {
            recipes: vec![3, 7],
            elves: vec![
                Elf { pos: 0 },
                Elf { pos: 1 },
            ],
        }
    }

    fn create_new_recipes(&mut self) {
        let sum = self.elves.iter().map(|e| self.recipes[e.pos]).sum();

        // Single digit sum
        if sum < 10 {
            self.recipes.push(sum);
        }
        // Double digit sum
        else {
            assert!(sum < 19);
            self.recipes.push(1);
            self.recipes.push(sum - 10);
        }

        // move the elves
        for e in &mut self.elves {
            let steps_forward = self.recipes[e.pos] + 1;
            e.pos = (e.pos + steps_forward) % self.recipes.len();
        }

    }

    // If the target sequence is found then return the number of recipes to the
    // left of it. We only need to check the final two sequences since the vec
    // of scores grows by up to 2 each iteration
    fn has_sequence(&self, target_sequence: &VecDeque<usize>) -> Option<usize> {
        let target_len = target_sequence.len();
        let len = self.recipes.len();
        let diff = len - target_len;

        if target_len > len {
            return None;
        }

        // Use VecDeque to efficiently add and remove from sequence as we go
        // through all the recipe scores, comparing every consecutive [len]
        // scores against target_sequence
        let mut sequence = VecDeque::new();
        for s in self.recipes.iter().skip(diff - 1).take(target_len) {
            sequence.push_back(*s);
        }
        if sequence == *target_sequence {
            return Some(diff - 1);
        }

        sequence.pop_front();
        sequence.push_back(self.recipes[len - 1]);
        if sequence == *target_sequence {
            return Some(diff);
        }

        None
    }
}

fn main() -> Result<(), ErrorHolder> {
    let input_str = fs::read_to_string("input.txt")?;
    let input: usize = input_str.trim().parse().expect("Failed to parse input");

    part1(input);
    part2(&input_str);

    Ok(())
}

fn part1(input: usize) {
    let mut recipe_board = RecipeBoard::new();
    //println!("{}", recipe_board);

    loop {
        recipe_board.create_new_recipes();
        //println!("{}", recipe_board);

        if recipe_board.recipes.len() > input + 10 {
            break;
        }
    }
    let next_10_scores = &recipe_board.recipes[input..input+10];
    print!("\nThe next 10 scores after {} recipes are: ", input);
    for s in next_10_scores {
        print!("{}", s);
    }
    print!("\n\n");
}

fn part2(input_str: &String) {
    let mut seq = VecDeque::new();
    for c in input_str.trim().chars() {
        seq.push_back(c.to_digit(10).expect("Failed char to digit") as usize);
    }

    let mut recipe_board = RecipeBoard::new();
    //println!("{}", recipe_board);

    loop {
        recipe_board.create_new_recipes();
        //println!("{}", recipe_board);

        match recipe_board.has_sequence(&seq) {
            Some(count) => {
                println!("There are {} recipes to the left of sequence {}",
                         count, input_str);
                break;
            },
            None => {},
        }
    }
}

