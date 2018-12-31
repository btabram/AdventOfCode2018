use std::fs;
use std::collections::HashMap;

extern crate failure;
use failure::Error;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }
}

#[derive(Debug)]
struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    fn new(p: &Point)-> Constellation {
        Constellation { points: vec![*p] }
    }

    fn can_add_point(&self, new_point: &Point) -> bool {
        for p in &self.points {
            if new_point.distance(p) <= 3 {
                return true;
            }
        }
        false
    }
}

fn parse_point(s: &str) -> Result<Point, Error> {
    let split: Vec<_> = s.split(",").map(|s| s.trim()).collect();

    let x: i32 = split[0].parse()?;
    let y: i32 = split[1].parse()?;
    let z: i32 = split[2].parse()?;
    let t: i32 = split[3].parse()?;

    Ok(Point { x, y, z, t })
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;

    let mut points = vec![];
    for line in input.lines() {
        points.push(parse_point(&line)?);
    }

    let mut constellations = HashMap::new();
    constellations.insert(0, Constellation::new(&points[0]));
    let mut id = 1;

    for p in points.iter().skip(1) {
        // Work out which constellations this point is close enough to join
        let mut can_add = vec![];
        for (id, c) in constellations.iter() {
            if c.can_add_point(p) {
                can_add.push(*id);
            }
        }

        // If it's not close enough to any existsing constellations then start a
        // new one
        if can_add.is_empty() {
            constellations.insert(id, Constellation::new(&p));
            id += 1;
        }
        // If it can add to exactly one constellations then just add it
        else if can_add.len() == 1 {
            constellations.get_mut(&can_add[0]).unwrap().points.push(*p);
        }
        // If it can add to multiple constellations then merge those
        // constellations into a single large constellation
        else {
            let mut merged_constellation_points = vec![*p];
            for ref id in can_add {
                let removed_c = constellations.remove(id).unwrap();
                for removed_p in removed_c.points {
                    merged_constellation_points.push(removed_p);
                }
            }

            let new_c = Constellation { points: merged_constellation_points };
            constellations.insert(id, new_c);
            id += 1;
        }
    }

    println!("There are {} different constellations.", constellations.len());

    Ok(())
}

