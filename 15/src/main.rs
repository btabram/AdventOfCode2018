use std::fs;

extern crate pathfinding;
use pathfinding::prelude::astar;

type ErrorHolder = Box<std::error::Error>;

macro_rules! unexpected {
    ($x:expr) => {{
        println!("Error! Unexpected {}", $x);
        std::process::exit(1);
    }};
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    println!("{}", input);

    Ok(())
}
