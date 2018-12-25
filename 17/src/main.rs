use std::fs;
use std::ops::Range;
use std::collections::BTreeSet;

type ErrorHolder = Box<std::error::Error>;
type PossibleRange = (i32, Option<i32>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Clay,
    Sand,
    WettedSand,
    Water,
    Spring,
}
use self::BlockType::*;

#[derive(Debug)]
struct GroundScan {
    grid: Vec<BlockType>,
    x_min: i32,
    y_min: i32,
    width: usize,
    height: usize,
}

impl GroundScan {
    fn new(x_min: i32, y_min: i32, width: usize, height: usize) -> GroundScan {
        let (y_min, height) = if y_min > 0 {
            (0, height + (y_min as usize))
        } else {
            (y_min, height)
        };
        let grid = vec![Sand; width * height];
        let mut gs = GroundScan { grid, x_min, y_min, width, height };

        // Add the spring
        gs.set(500, 0, Spring);
        gs
    }

    fn add_clay(&mut self, x_range: Range<i32>, y_range: Range<i32>) {
        for x in x_range {
            for y in y_range.clone() {
                self.set(x, y, Clay);
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> BlockType {
        let x_i = (x - self.x_min) as usize;
        let y_i = (y - self.y_min) as usize;
        self.grid[x_i + (self.width * y_i)]
    }

    fn set(&mut self, x: i32, y: i32, value: BlockType) {
        let x_i = (x - self.x_min) as usize;
        let y_i = (y - self.y_min) as usize;
        self.grid[x_i + (self.width * y_i)] = value;
    }
}

impl std::fmt::Display for GroundScan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printing_grid = self.grid.iter().map(|b| match b {
                                                Clay => '#',
                                                Sand => '.',
                                                WettedSand => '|',
                                                Water => '~',
                                                Spring => '+',
                                            }).collect::<Vec<_>>();
        let mut grid_string = String::new();
        for row_index in 0..self.height {
            let mut row: String = printing_grid.iter()
                                    .skip(row_index * self.width)
                                    .take(self.width).collect();
            row.push('\n');
            grid_string.push_str(&row);
        }
        write!(f, "{}", grid_string)
    }
}

fn s_to_i(s: &str) -> i32 {
    s.parse().expect("Failed to parse str as i32")
}

fn parse_possible_range(s: &str) -> PossibleRange {
    match s.contains("..") {
        true => {
            let split: Vec<_> = s.split("..").map(s_to_i).collect();
            assert_eq!(split.len(), 2);
            (split[0], Some(split[1]))
        },
        false => {
            (s_to_i(s), None)
        }
    }
}

fn parse_x_and_y(line: &str) -> (PossibleRange, PossibleRange) {
    let split: Vec<_> =  line.split(", ").collect();
    assert_eq!(split.len(), 2);

    // Determine if x or y is given first
    let (x_index, y_index) = match split[0].contains("x") {
        true => (0, 1),
        false => (1, 0),
    };

    let x = parse_possible_range(&split[x_index][2..]);
    let y = parse_possible_range(&split[y_index][2..]);
    (x, y)
}

// This is a very ugly function... I should have made a PossibleRange struct...
fn get_grid_limits(clay: &Vec<(PossibleRange, PossibleRange)>) ->
                                                        (i32, i32, i32, i32) {
    let err_str = "Failed to find grid limit";

    let ((x_min, _), (_, _)) =
        clay.iter().min_by_key(|((x, _), (_, _))| x).expect(err_str);
    let ((_, _), (y_min, _)) =
        clay.iter().min_by_key(|((_, _), (y, _))| y).expect(err_str);

    let ((x_l_max, _), (_, _)) =
        clay.iter().max_by_key(|((x, _), (_, _))| x).expect(err_str);
    let ((_, _), (y_l_max, _)) =
        clay.iter().max_by_key(|((_, _), (y, _))| y).expect(err_str);

    let ((_, x_r_max_option), (_, _)) =
        clay.iter()
        .max_by_key(|((_, x), (_, _))| match *x { None => 0, Some(x) => x })
        .expect(err_str);
    let ref x_r_max = x_r_max_option.unwrap();
    let ((_, _), (_, y_r_max_option)) =
        clay.iter()
        .max_by_key(|((_, _), (_, y))| match *y { None => 0, Some(y) => y })
        .expect(err_str);
    let ref y_r_max = y_r_max_option.unwrap();

    let x_max = if x_l_max > x_r_max { x_l_max } else { x_r_max };
    let y_max = if y_l_max > y_r_max { y_l_max } else { y_r_max };

    (*x_min, *x_max, *y_min, *y_max)
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    let mut clay = vec![];
    for ref line in input.lines() {
        clay.push(parse_x_and_y(line));
    }

    let (x_min, x_max, y_min, y_max) = get_grid_limits(&clay);
    let width = (1 + x_max - x_min) as usize;
    let height = (1 + y_max - y_min) as usize;

    // Add some padding in the x direction
    let mut scan = GroundScan::new(x_min - 1, y_min, width + 2, height);
    for c in clay {
        let ((x1, x2_option), (y1, y2_option)) = c;

        let x2 = match x2_option {
            Some(x) =>  x + 1,
            None => x1 + 1,
        };
        let y2 = match y2_option {
            Some(y) =>  y + 1,
            None => y1 + 1,
        };

        scan.add_clay(x1..x2, y1..y2);
    }

    let mut water_falling_points = BTreeSet::new();
    water_falling_points.insert((500, 0));

    'outer: loop {
        if  water_falling_points.len() == 0 {
            break;
        }

        // Clone so we don't keep an immutable reference to water_falling_points
        let key = water_falling_points.iter().next().unwrap().clone();
        let (x_source, y_source) = key;

        // Find a block below and to start spreading out from
        let mut j = 1;
        loop {
            if j == (scan.height as i32) - scan.y_min - y_source {
                // We've hit the bottom of the map so stop resolving this source
                water_falling_points.remove(&key);
                continue 'outer;
            }

            let block_below = scan.get(x_source, y_source + j);
            match block_below {
                Sand => scan.set(x_source, y_source + j, WettedSand),
                Clay | Water => {
                    // Go back up a level since we've hit something tha water
                    // will sit on
                    j -= 1;
                    break;
                },
                // Water has already flowed here, so stop resolving this source
                WettedSand => {
                    water_falling_points.remove(&key);
                    continue 'outer;
                },
                _ => unreachable!(),
            }
            j += 1;
        }

        // Start spreading out now that we're done falling down
        loop {
            scan.set(x_source, y_source + j, Water);

            // Consider spreading sideways
            // First spread right
            let mut new_falling_point = false;
            let mut right_edge_contained = false;
            let mut i = 1;
            loop {
                let block_right = scan.get(x_source + i, y_source + j);
                match block_right {
                    Sand | WettedSand => {
                        let block_below = scan.get(x_source + i, y_source + j + 1);
                        match block_below {
                            Sand => {
                                scan.set(x_source + i, y_source + j, WettedSand);
                                water_falling_points.insert((x_source + i, y_source + j));
                                new_falling_point = true;
                                break;
                            },
                            Clay | Water => scan.set(x_source + i, y_source + j, Water),
                            WettedSand => break,
                            _ => unreachable!(),
                        }
                    },
                    Clay => {
                        right_edge_contained = true;
                        break;
                    },
                    Water => break,
                    _ => unreachable!(),
                }
                i += 1;
            }
            let i_max = i;

            // Then spread left
            let mut left_edge_contained = false;
            i = -1;
            loop {
                let block_left = scan.get(x_source + i, y_source + j);
                match block_left {
                    Sand | WettedSand => {
                        let block_below = scan.get(x_source + i, y_source + j + 1);
                        match block_below {
                            Sand => {
                                scan.set(x_source + i, y_source + j, WettedSand);
                                water_falling_points.insert((x_source + i, y_source + j));
                                new_falling_point = true;
                                break;
                            },
                            Clay | Water => scan.set(x_source + i, y_source + j, Water),
                            WettedSand => break,
                            _ => unreachable!(),
                        }
                    },
                    Clay => {
                        left_edge_contained = true;
                        break;
                    },
                    Water => break,
                    _ => unreachable!(),
                }
                i -= 1;
            }
            let i_min = i;

            // If we've found a new falling point then blocks on this level are
            // where water flowed, not sat. It also means we've finished
            // resolving this falling point so exit the loop
            if new_falling_point {
                // Account for fact we have searched one block either side to
                // find clay walls or falling points
                for k in (i_min + 1)..i_max {
                    scan.set(x_source + k, y_source + j, WettedSand);
                }

                water_falling_points.remove(&key);
                continue 'outer;
            }

            // If both edges are contained by clay walls then start filling up
            // by spreading left and right again at a lower y value
            if right_edge_contained && left_edge_contained {
                j -= 1;
            }
            else {
                break;
            }
        }
    }

    println!("{}", scan);

    // scan.y_min may be lower than y_min from the input in order to fit the
    // initial spring on the grid. When counting up the water for the answers
    // we don't inlcude anything with y coordinate less than the miniumum y
    // value from the input so we may need to skip a few rorws of the grid
    let y_skip = (y_min - scan.y_min) as usize;

    let part1_count = scan.grid.iter().skip(y_skip * scan.width)
                        .filter(|&&b| b == Water || b == WettedSand).count();
    println!("The water reaches {} tiles!", part1_count);

    let part2_count = scan.grid.iter().skip(y_skip * scan.width)
                        .filter(|&&b| b == Water).count();
    println!("{} water tiles are left when the spring dries up!", part2_count);

    Ok(())
}
