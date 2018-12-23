use std::fs;
use std::collections::{HashMap, VecDeque};

type ErrorHolder = Box<std::error::Error>;

#[derive(Debug)]
struct Plants<'a> {
    state: VecDeque<char>,
    zero_pos: usize,
    rules: HashMap<&'a str, char>,
}

impl<'a> Plants<'a> {
    // We need a buffer of at least 2 empty pots either end of our array of pots
    // we care about so we can calculate the next generation correctly. Choose a
    // buffer of 5 empty pots so that we won't miss any plant spreading since we
    // assume that 5 empty pots -> empty pot.
    fn pad_state(&mut self) {
        for i in 0..5 {
            if self.state[i] != '.' {
                self.state.push_front('.');
                self.zero_pos += 1;
            }

            if self.state[self.state.len() - (i+1)] != '.' {
                self.state.push_back('.');
            }
        }

        // Trim any excess leading empty pots
        loop {
            if self.state[5] == '.' {
                self.state.pop_front();
                self.zero_pos -= 1;
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        self.pad_state();
        let mut next_state = self.state.clone();

        // Account for needing 2 pots either side to calculate
        for i in 0..(self.state.len() - 4) {
            let pos = i + 2;
            let future_plant = self.get_future_plant(pos);
            std::mem::replace(&mut next_state[pos], future_plant);
        }

        self.state = next_state;
    }

    fn get_future_plant(&self, pos: usize) -> char {
        let s: String = self.state.iter().collect();
        let neighbour_state = &s[pos-2..pos+3];
        //println!("{}: {} -> {:?}", pos, neighbour_state, self.rules.get(&neighbour_state));
        match self.rules.get(&neighbour_state) {
            Some(c) => *c,
            None => '.',
        }
    }

    fn sum_plant_pot_numbers(&self) -> usize {
        let mut sum = 0;
        for (i, p) in self.state.iter().enumerate() {
            let pot_pos = i - self.zero_pos;
            if *p == '#' {
                sum += pot_pos;
            }
        }
        sum
    }
}

impl<'a> std::fmt::Display for Plants<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut printing_state = self.state.clone();
        for _ in self.zero_pos..25 {
            printing_state.push_front('.');
        }
        write!(f, "{}", printing_state.iter().collect::<String>())
    }
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    //let input = fs::read_to_string("test.txt")?;

    let first = input.lines().next().expect("Failed to get first input line");
    let colon_pos = first.find(": ").expect("Didn't find colon");
    let initial_state = &first[colon_pos+2..];
    println!("{}", initial_state);

    let mut rules = HashMap::new();
    for line in input.lines().skip(2) {
        let key = &line[0..5];
        let value = line.chars().skip(9).next()
                        .expect("Didn't find result while parsing rules");
        rules.insert(key, value);
    }
    println!("{:?}", rules);

    let mut state = VecDeque::new();
    for c in initial_state.chars() {
        state.push_back(c);
    }
    let mut plants = Plants { state, rules, zero_pos: 0 };

    plants.pad_state();
    println!("\n{}", plants);

    let mut prev_state = plants.state.clone();
    let mut prev_sum = plants.sum_plant_pot_numbers();

    let mut steady_state_offset = None;
    let mut steady_state_count = 0;

    let generations = 50000000000usize;
    for i in 0..generations {

        plants.advance();
        println!("{}", plants);

        // Try and notice when a steady state is reached so that we don't have
        // to calculate all the steps individually
        if prev_state == plants.state {
            if steady_state_offset == None {
                steady_state_offset =
                    Some(plants.sum_plant_pot_numbers() - prev_sum);
            } else {
                let offset = plants.sum_plant_pot_numbers() - prev_sum;
                assert!(steady_state_offset == Some(offset));
            }
            steady_state_count += 1;

            // Been in a steady state for a while, assume it just carries on
            if steady_state_count == 5 {
                let mut sum = plants.sum_plant_pot_numbers();
                let remaining_generations = generations - 1 - i;
                sum += steady_state_offset.unwrap() * remaining_generations;

                // Part 2
                println!("\nThe sum after {} generations is {}\n",
                         generations, sum);
                return Ok(());
            }
        }

        // Part 1 (we index from 0)
        if i == 19 {
            println!("\nThe sum after {} generations is {}\n",
                     i + 1, plants.sum_plant_pot_numbers());
        }

        prev_state = plants.state.clone();
        prev_sum = plants.sum_plant_pot_numbers();
    }

    println!("\nThe sum after {} generations is {}\n",
             generations, plants.sum_plant_pot_numbers());

    Ok(())
}

