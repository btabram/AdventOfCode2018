use std::fs;

type ErrorHolder = Box<std::error::Error>;

#[derive(Debug, Clone, Copy)]
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
                if (x, y) == (width - 1, height - 1) {
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

    let (x_max, y_max) = target;
    let cs = CaveSystem::new(x_max + 1, y_max + 1, depth, target);
    println!("{}", cs);

    let danger_index: i32 = cs.vec.iter().map(|r| match r.t {
                                                    Rocky => 0,
                                                    Wet => 1,
                                                    Narrow => 2,
                                                    Unknown => unreachable!(),
                                                }).sum();
    println!("The danger index is {}.", danger_index);

    Ok(())
}
