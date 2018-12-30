use std::fs;
use std::cmp::max;

type ErrorHolder = Box<std::error::Error>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    fn distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
    }
}

#[derive(Debug)]
struct Nanobot {
    pos: Position,
    signal_radius: i32,
}

impl Nanobot {
    fn new(x: i32, y: i32, z: i32, r: i32) -> Nanobot {
        Nanobot {
            pos: Position { x, y, z },
            signal_radius: r,
        }
    }

    fn is_bot_in_range(&self, other: &Nanobot) -> bool {
        self.pos.distance(&other.pos) <= self.signal_radius
    }

    fn is_pos_in_range(&self, other: &Position) -> bool {
        //assert!(self.pos.distance(&other) >= 0);
        self.pos.distance(&other) <= self.signal_radius
    }
}

fn s_to_i(s: &str) -> i32 {
    s.parse().expect("Failed to parse str as i32")
}

fn parse_input(input: &String) -> Vec<Nanobot> {
    let mut nanobots = vec![];
    for line in input.lines() {
        let pos_start = line.find("<").expect("Failed to find < in input line");
        let pos_end = line.find(">").expect("Failed to find > in input line");
        let pos_str = &line[(pos_start + 1)..pos_end];
        let pos: Vec<_> = pos_str.split(",").map(s_to_i).collect();

        let r_start = line.find("r=").expect("Failed to find r= in input line");
        let r = s_to_i(&line[(r_start + 2)..]);

        nanobots.push(Nanobot::new(pos[0], pos[1], pos[2], r));
    }
    nanobots
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    // Part 1
    let nanobots = parse_input(&input);
    let strongest = nanobots.iter().max_by_key(|n| n.signal_radius).unwrap();

    let mut in_range_count = 0;
    for ref n in &nanobots {
        if strongest.is_bot_in_range(n) {
            in_range_count += 1;
        }
    }
    println!("There are {} nanobots in range of the strongest nanobot.",
             in_range_count);


    // Part 2. I couldn't think of a good way around brute force so this method
    // is from https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecdmwui

    let origin = Position::new(0, 0, 0);
    let mut min_x = nanobots.iter().min_by_key(|n| n.pos.x).unwrap().pos.x;
    let mut max_x = nanobots.iter().max_by_key(|n| n.pos.x).unwrap().pos.x;
    let mut min_y = nanobots.iter().min_by_key(|n| n.pos.y).unwrap().pos.y;
    let mut max_y = nanobots.iter().max_by_key(|n| n.pos.y).unwrap().pos.y;
    let mut min_z = nanobots.iter().min_by_key(|n| n.pos.z).unwrap().pos.z;
    let mut max_z = nanobots.iter().max_by_key(|n| n.pos.z).unwrap().pos.z;

    // Set the initial grid size so that we have ~8 grid points per side of the
    // cube containing all nanobots as a fairly random starting resolution
    let longest_side = max(max_x - min_x, max(max_y - min_y, max_z - min_z));
    // Minimum power of 2 to fit all bots in a box of this side length
    let min_pow = (longest_side as f64).log2().floor() as u32  + 1;
    // Aim for several grid points per side (~2^3)
    let mut grid_size = 2_usize.pow(min_pow - 3);

    // Start initially at a large grid size and find which areas of the space
    // have many nanobots in range. Over time refine the grid size and zoom in
    // on the area with most nanobots in range to find the position with most
    // nanobots in range which is closest to the origin. I don't think this
    // is guarnteed to work in a general case since we refine quite aggressively
    // and only sample points in the space rather than doing something more
    // complete like finding all nanbots with ranges overlapping a cube. Having
    // said that it works with the supplied problem input
    loop {
        let mut best_count = -1;
        let mut best_pos = None;
        let mut best_pos_origin_distance = std::i32::MAX;

        for x in (min_x..=max_x).step_by(grid_size) {
            for y in (min_y..=max_y).step_by(grid_size) {
                for z in (min_z..=max_z).step_by(grid_size) {

                    let pos = Position { x, y, z };

                    let mut bots_in_range_count = 0;
                    for n in &nanobots {
                        if n.is_pos_in_range(&pos) {
                            bots_in_range_count += 1;
                        }
                    }

                    // We've found a new best position with most bots in range
                    if bots_in_range_count > best_count {
                        best_count = bots_in_range_count;
                        best_pos_origin_distance = pos.distance(&origin);
                        best_pos = Some(pos);
                    }
                    // Use distance to the origin as a tiebreaker
                    else if bots_in_range_count == best_count {
                        if pos.distance(&origin) < best_pos_origin_distance {
                            best_pos_origin_distance = pos.distance(&origin);
                            best_pos = Some(pos);
                        }
                    }
                }
            }
        }

        // If we're down to a grid size of one then we've finished our search
        if grid_size == 1 {
            println!("The distance from the origin to the point in range of \
                     most nanobots is {}.", best_pos_origin_distance);
            break;
        }

        // Update the search area limits, focusing in on the 'best' position
        let best = best_pos.unwrap();
        let size = grid_size as i32;
        min_x = best.x - size;
        max_x = best.x + size;
        min_y = best.y - size;
        max_y = best.y + size;
        min_z = best.z - size;
        max_z = best.z + size;

        // Refine the grid size before the next iteration
        grid_size /= 2;
    }

    Ok(())
}
