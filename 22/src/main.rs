use std::fs;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;

extern crate pathfinding;
use pathfinding::prelude::astar;

type ErrorHolder = Box<std::error::Error>;
type Moves = Vec<(CaveSystemState, usize)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RegionType {
    Rocky,
    Narrow,
    Wet,
    Unknown,
}
use self::RegionType::*;

#[derive(Debug, Clone, Copy)]
struct Region {
    t: RegionType,
    geologic_index: Option<i32>,
    erosion_level: Option<i32>,
}

impl Region {
    fn new(x: i32, y: i32, depth: i32, target: (i32, i32)) -> Region {
        let mut region = Region {
            t: Unknown,
            geologic_index: None,
            erosion_level: None,
        };

        let coord = (x, y);
        if coord == (0, 0) || coord == target {
            region.set_geologic_index(0, depth);
        }

        if y == 0 {
            region.set_geologic_index(x * 16807, depth);
        }
        if x == 0 {
            region.set_geologic_index(y * 48271, depth);
        }

        region
    }

    fn set_geologic_index(&mut self, gi: i32, depth: i32) {
        self.geologic_index = Some(gi);
        let el = (gi + depth) % 20183;
        self.erosion_level = Some(el);
        self.t = match el % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct CaveSystem {
    vec: Vec<Region>,
    width: usize,
    height: usize,
    depth: i32,
    target: (i32, i32),
}

impl std::fmt::Display for CaveSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printing_grid = self.vec.iter().map(|b| match b.t {
                                                Rocky => '.',
                                                Narrow => '|',
                                                Wet => '=',
                                                Unknown => '?',
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

impl CaveSystem {
    fn new(width: i32, height: i32,
           depth: i32, target: (i32, i32)) -> CaveSystem {

        let mut vec = vec![];
        for y in 0..height {
            for x in 0..width {
                vec.push(Region::new(x, y, depth, target));
            }
        }

        let width = width as usize;
        let  height = height as usize;
        let mut cs = CaveSystem {
            vec,
            width,
            height,
            depth,
            target,
        };

        let err_str = "Unexpectedly unknown erosion level!";
        for x in 1..width  {
            for y in 1..height {
                // Don't overrite the RegionType for the target
                if (x as i32, y as i32) == target {
                    continue;
                }

                let x_minus =  cs.get(x - 1, y).erosion_level.expect(err_str);
                let y_minus =  cs.get(x, y - 1).erosion_level.expect(err_str);
                let current = cs.get_mut_ref(x, y);
                current.set_geologic_index(x_minus * y_minus, depth);
            }
        }

        cs
    }


    fn get_mut_ref(&mut self, x: usize, y: usize) -> &mut Region {
        &mut self.vec[x + (self.width * y)]
    }

    fn get(&self, x: usize, y: usize) -> Region {
        self.vec[x + (self.width * y)]
    }

    fn get_possible_moves(&self, css: &CaveSystemState) -> Moves {
        let x = css.x;
        let y = css.y;
        let ref current_tool = css.tool;
        let ref current_region_type = css.region_type;


        //### Consider moves into neighbouring regions ###//
        let mut next_move_region_types = vec![
            (x + 1, y, self.get(x + 1, y).t),
            (x, y + 1, self.get(x, y + 1).t),
        ];

        // Can't go into negative x or y regions
        if x > 0 {
            next_move_region_types.push((x - 1, y, self.get(x - 1, y).t));
        }
        if y > 0 {
            next_move_region_types.push((x, y - 1, self.get(x, y - 1).t));
        }

        let mut next_moves = vec![];
        for n in next_move_region_types {
            let (n_x, n_y, ref n_region_type) = n;

            // If the current tool is valid for the neighbouring square then
            // we can just move in at a cost of one minute
            if VALID_GEAR.get(n_region_type).unwrap().contains(current_tool) {
                let next_state = CaveSystemState {
                    x: n_x,
                    y: n_y,
                    tool: current_tool.clone(),
                    region_type: n_region_type.clone(),
                };
                next_moves.push((next_state, 1));
            }
        }


        //### Consider swapping the current tool at a cost of 7 minutes ###//
        let new_tool: Vec<_> =
            TOOLS.iter().filter(
                |t| *t != current_tool &&
                VALID_GEAR.get(current_region_type).unwrap().contains(t)
            ).collect();
        assert!(new_tool.len() == 1);

        let new_tool_state = CaveSystemState {
            x,
            y,
            tool: new_tool[0].clone(),
            region_type: current_region_type.clone(),
        };
        next_moves.push((new_tool_state, 7));


        next_moves
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}
use self::Tool::*;

lazy_static! {
    static ref TOOLS: Vec<Tool> = vec![Torch, ClimbingGear, Neither];
}


lazy_static! {
    static ref VALID_GEAR: HashMap<RegionType, HashSet<Tool>> = {
        let mut map = HashMap::new();
        map.insert(Rocky, vec![ClimbingGear, Torch].iter().cloned().collect());
        map.insert(Wet, vec![ClimbingGear, Neither].iter().cloned().collect());
        map.insert(Narrow, vec![Torch, Neither].iter().cloned().collect());
        map
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CaveSystemState {
    x: usize,
    y: usize,
    tool: Tool,
    region_type: RegionType,
}

impl CaveSystemState {
    fn distance(&self, other: &CaveSystemState) -> usize {
        ((self.x as i32 - other.x as i32).abs()
            + (self.y as i32 - other.y as i32).abs()) as usize
    }
}

fn s_to_i(s: &str) -> i32 {
    s.parse().expect("Failed to parse str as i32")
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    let mut depth = None;
    let mut target = None;

    for line in input.lines() {
        if line.contains("depth") {
            let d = line.split(" ").collect::<Vec<_>>()[1];
            depth = Some(s_to_i(d));
        }
        if line.contains("target") {
            let t = line.split(" ").collect::<Vec<_>>()[1];
            let t_split = t.split(",").map(s_to_i).collect::<Vec<_>>();
            target = Some((t_split[0], t_split[1]));
        }
    }

    let target = target.expect("Failed to find target in the input");
    let depth = depth.expect("Failed to find depth in the input");

    part1(target, depth);
    part2(target, depth);

    Ok(())
}

fn part1(target: (i32, i32), depth: i32) {
    let (x_max, y_max) = target;
    let cs = CaveSystem::new(x_max + 1, y_max + 1, depth, target);
    println!("{}", cs);

    let danger_index: i32 = cs.vec.iter().map(|r| match r.t {
                                                    Rocky => 0,
                                                    Wet => 1,
                                                    Narrow => 2,
                                                    Unknown => unreachable!(),
                                                }).sum();
    println!("The danger index is {}.\n", danger_index);
}

fn part2(target: (i32, i32), depth: i32) {
    let (x_max, y_max) = target;
    // Allow 20 squares extra beyond the target in x and y since the fastest
    // route may involve some squares beyond the target x and y values.
    let cs = CaveSystem::new(x_max + 21, y_max + 21, depth, target);

    let start = CaveSystemState {
        x: 0,
        y: 0,
        tool: Torch,
        region_type: Rocky,
    };

    let dest = CaveSystemState {
        x: target.0 as usize,
        y: target.1 as usize,
        tool: Torch,
        region_type: Rocky,
    };

    let quickest_path = astar(
                            &start,
                            |s| cs.get_possible_moves(s),
                            |s| s.distance(&dest),
                            |s| s == &dest
                        );
    println!("The quickest path to save Santa's friend takes {} minutes.",
             quickest_path.expect("Failed to find a path").1);
}
