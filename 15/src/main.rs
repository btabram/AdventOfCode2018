use std::fs;
use std::cmp::Ordering;

extern crate pathfinding;
use pathfinding::prelude::astar;

extern crate failure;
use failure::Error;

type Path = (Vec<Square>, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct UnitData {
    hp: i32,
    attack: i32,
    had_turn: bool,
}

impl UnitData {
    fn new() -> UnitData {
        UnitData { hp: 200, attack: 3, had_turn: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SquareType {
    Open,
    Wall,
    Elf,
    Goblin
}
use self::SquareType::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Square {
    x: usize,
    y: usize,
    t: SquareType,
    data: Option<UnitData>,
}

impl Square {
    fn new(x: usize, y: usize, c: char) -> Square {
        match c {
            '.' => {
                Square { x, y, t: Open, data: None }
            },
            '#' => {
                Square { x, y, t: Wall, data: None }
            },
            'E' => {
                Square { x, y, t: Elf, data: Some(UnitData::new()) }
            },
            'G' => {
                Square { x, y, t: Goblin, data: Some(UnitData::new()) }
            },
            _ => unreachable!(),
        }
    }

    fn distance(&self, other: &Square) -> usize {
        ((self.x as i32 - other.x as i32).abs()
            + (self.y as i32 - other.y as i32).abs()) as usize
    }
}

impl PartialOrd for Square {
    // Order squares by reading order
    fn partial_cmp(&self, other: &Square) -> Option<Ordering> {
        let s = self.x + (self.y * 10000);
        let o = other.x + (other.y * 10000);
        s.partial_cmp(&o)
    }
}

impl Ord for Square {
    // Order squares by reading order
    fn cmp(&self, other: &Square) -> Ordering {
        let s = self.x + (self.y * 10000);
        let o = other.x + (other.y * 10000);
        s.cmp(&o)
    }
}

#[derive(Debug, Clone)]
struct Map {
    map_vec: Vec<Square>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y :usize) -> Square {
        self.map_vec[x + (self.width * y)].clone()
    }

    fn get_mut_ref(&mut self, x: usize, y: usize) -> &mut Square {
        &mut self.map_vec[x + (self.width * y)]
    }

    fn get_neighbours(&self, square: &Square) -> Vec<Square> {
        let x = square.x;
        let y = square.y;
        vec![
            // Order the neighbours by reading order
            self.get(x, y - 1),
            self.get(x - 1, y),
            self.get(x + 1, y),
            self.get(x, y + 1),
        ]
    }

    fn get_possible_moves(&self, square: &Square) -> Vec<(Square, usize)> {
        let mut neighbours = self.get_neighbours(square);
        // Can only move into open squares
        neighbours.retain(|s| s.t == Open);
        // Map into a vector of possible moves and their distance costs
        neighbours.iter().map(|s| (s.clone(), 1)).collect()
    }

    fn get_targets_in_range(&self, square: &Square) -> Vec<Square> {
        let enemy = match square.t {
            Elf => Goblin,
            Goblin => Elf,
            _ => unreachable!(),
        };
        let mut neighbours = self.get_neighbours(square);
        neighbours.retain(|s| s.t == enemy);
        neighbours
    }

    // The returned units vector is in reading order
    fn get_units_mut(&mut self) -> Vec<&mut Square> {
        let mut units_mut = vec![];
        for s in self.map_vec.iter_mut() {
            match s.t {
                Open | Wall => {},
                _ => units_mut.push(s),
            }
        }
        units_mut
    }

    // The returned units vector is in reading order
    fn get_units(&self) -> Vec<Square> {
        let mut units = vec![];
        for s in &self.map_vec {
            match s.t {
                Open | Wall => {},
                _ => units.push(s.clone()),
            }
        }
        units
    }

    // The returned elves vector is in reading order
    fn get_elves(&self) -> Vec<Square> {
        let mut elves = self.get_units();
        elves.retain(|s| s.t == Elf);
        elves
    }

    // The returned elves vector is in reading order
    fn get_elves_mut(&mut self) -> Vec<&mut Square> {
        let mut elves_mut = self.get_units_mut();
        elves_mut.retain(|s| s.t == Elf);
        elves_mut
    }

    // The returned goblins vector is in reading order
    fn get_goblins(&self) -> Vec<Square> {
        let mut goblins = self.get_units();
        goblins.retain(|s| s.t == Goblin);
        goblins
    }

    fn find_path(&self, start: &Square, dest: &Square) -> Option<Path> {
        astar(
            start,
            |s| self.get_possible_moves(s),
            |s| s.distance(dest),
            |s| s.x == dest.x && s.y == dest.y
        )
    }

    // Returns true if the battle finished during the turn, otherwise false
    fn take_turn(&mut self) -> bool {
        for u in self.get_units_mut() {
            u.data.as_mut().expect("Unit doesn't have data").had_turn = false;
        }


        // MOVEMENT
        loop {
            let mut next_square = None;
            let mut x = None;
            let mut y = None;

            // Iterate to find a unit that hasn't moved yet
            for unit in self.get_units() {
                if unit.data.clone().expect("Unit doesn't have data").had_turn {
                    continue;
                }

                // This is needed for attacking etc. later
                x = Some(unit.x);
                y = Some(unit.y);

                // Don't move if already in range of a target
                if !self.get_targets_in_range(&unit).is_empty() {
                    break;
                }

                // Get a list of potential targets
                let potential_targets = match unit.t {
                    Elf => self.get_goblins(),
                    Goblin => self.get_elves(),
                    _ => unreachable!(),
                };

                // The battle ends if there's no potential targets left
                if potential_targets.is_empty() {
                    return true;
                }

                // Work out which squares are in range of a potential target
                let mut in_range: Vec<_>
                    = potential_targets.iter()
                                       .flat_map(|s| self.get_possible_moves(s))
                                       .map(|(s, _)| s)
                                       .collect();
                // Sort by reading order and remove duplicates
                in_range.sort_unstable();
                in_range.dedup();

                // Refine to reachable squares
                let mut targets: Vec<_> = in_range
                                            .iter()
                                            .map(|s| self.find_path(&unit, s))
                                            .filter(|path| *path != None)
                                            .map(|x| x.unwrap())
                                            .collect();

                // Bail out if there's no reachable targets
                if targets.is_empty() {
                    break;
                }

                // Refine to the (joint) closest target squares
                let min_dist = match targets.iter().min_by_key(|(_, d)| d) {
                    Some((_, d)) => *d,
                    None => unreachable!(),
                };
                targets.retain(|(_, d)| *d == min_dist);

                // Sort targets by reading order (all paths end in the target so
                // we can look at the end of a path to find the target)
                targets.sort_unstable_by_key(
                    |(path, _)| path[path.len() - 1].clone());

                // Work out the target square we're trying to move towards
                let (ref target_path, _) = targets[0];
                let target_square = target_path.last().unwrap().clone();

                // Now we've got a target square we consider each of the (up to)
                // 4 possible next steps from our current position. We eliminate
                // all squares except those satisfying the minimum distance to
                // our target which we found earlier. The vector is sorted by
                // reading order so we can then just take the first element as
                // our next square to move to.
                let possible_next_squares: Vec<_>
                    = self.get_possible_moves(&unit)
                        .iter()
                        .map(|(s, _)| s.clone())
                        .filter(|s| match self.find_path(s, &target_square) {
                            None => false,
                            Some((_, dist)) => dist + 1 == min_dist,
                        })
                        .collect();

                next_square = Some(possible_next_squares[0].clone());
                break;
            }

            // All units have taken turns so return
            if x == None {
                return false;
            }

            let unit = self.get_mut_ref(x.unwrap(), y.unwrap());
            unit.data.as_mut().unwrap().had_turn = true;

            // Actually move the unit on the map if appropriate
            if next_square != None {
                let unit_type = unit.t.clone();
                let unit_data = unit.data.clone();

                unit.t = Open;
                unit.data = None;

                // Explicitly drop [unit] since we're done with it now and need
                // to take a new mut reference to self for [moved_unit]
                std::mem::drop(unit);

                let ns = next_square.unwrap();
                let moved_unit = self.get_mut_ref(ns.x, ns.y);
                moved_unit.t = unit_type;
                moved_unit.data = unit_data;

                // Update x and y, they might be needed during attacking
                x = Some(ns.x);
                y = Some(ns.y);
            }
            else {
                std::mem::drop(unit);
            }


            // ATTACK
            let u = self.get(x.unwrap(), y.unwrap());
            let targets_in_range = self.get_targets_in_range(&u);

            if targets_in_range.is_empty() {
                continue;
            }

            let mut min_hp = 201;
            let mut best_target = None;
            for mut t in targets_in_range {
                let hp = t.data.as_mut().expect("Unit has no data").hp;
                if hp < min_hp {
                    min_hp = hp;
                    best_target = Some(t.clone());
                }
                else if hp == min_hp {
                    // Should order by reading order because of Ord impl
                    if t < best_target.clone().unwrap() {
                        best_target = Some(t.clone());
                    }
                }
            }

            let target = best_target.expect("Error finding best target");
            let target_mut = self.get_mut_ref(target.x, target.y);

            // Do the attack
            target_mut.data.as_mut().unwrap().hp
                -= u.data.as_ref().unwrap().attack;

            // Remove the victim if it has died
            if target_mut.data.as_mut().unwrap().hp <= 0 {
                target_mut.data = None;
                target_mut.t = Open;
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printing_map_vec = self.map_vec.iter().map(|s| match s.t {
                                                            Open => '.',
                                                            Wall => '#',
                                                            Elf => 'E',
                                                            Goblin => 'G',
                                                        }).collect::<Vec<_>>();
        let mut map_string = String::new();
        for row_index in 0..self.height {
            let mut row: String = printing_map_vec.iter()
                                    .skip(row_index * self.width)
                                    .take(self.width).collect();
            row.push('\n');
            map_string.push_str(&row);
        }
        write!(f, "{}", map_string)
    }
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut map_vec = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map_vec.push(Square::new(x, y, c));
        }
    }
    let starting_map = Map { map_vec, width, height };


    // Part 1
    let mut part1_map = starting_map.clone();
    println!("Initially:\n{}", part1_map);

    let mut complete_rounds = 0;
    loop {
        if part1_map.take_turn() {
            break;
        };
        complete_rounds += 1;
        println!("After {} rounds:\n{}", complete_rounds, part1_map);
    }

    let mut units = part1_map.get_units_mut();
    let total_hp: i32 = units.iter_mut()
                             .map(|u| u.data.as_mut().unwrap().hp)
                             .sum();
    println!("The outcome of the battle for Part 1 is: {} * {} = {}\n",
             complete_rounds, total_hp, complete_rounds * total_hp);


    // Part 2
    let inital_elves = starting_map.get_elves().len();
    for boost in 1..200 {
        println!("Simulating a battle with attack boost {} for the elves...",
                 boost);
        let mut part2_map = starting_map.clone();

        part2_map.get_elves_mut().iter_mut().for_each(|e| {
            e.data.as_mut().unwrap().attack += boost;
        });

        let (outcome, remaining_elves) = resolve_battle(&part2_map);
        if inital_elves == remaining_elves {
            println!("With an attack boost of {} the elves win without losses. \
                        The outcome of this battle is {}.", boost, outcome);
            break;
        }
    }

    Ok(())
}

// Resolve a battle. Returning the outcome and the number of remaining elves
fn resolve_battle(starting_map: &Map) -> (i32, usize) {
    let mut map = starting_map.clone();
    let mut complete_rounds = 0;
    loop {
        if map.take_turn() {
            break;
        };
        complete_rounds += 1;
    }

    let mut units = map.get_units_mut();
    let total_hp: i32 = units.iter_mut()
                             .map(|u| u.data.as_mut().unwrap().hp)
                             .sum();

    (total_hp * complete_rounds, map.get_elves().len())
}
