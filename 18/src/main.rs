use std::fs;

type ErrorHolder = Box<std::error::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Open,
    Wooded,
    LumberYard,
}
use self::TileType::*;

#[derive(Debug, Clone, PartialEq)]
struct Grid {
    vec: Vec<TileType>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get_mut_ref(&mut self, x: usize, y: usize) -> Option<&mut TileType> {
        if x >= self.width || y >= self.height {
            None
        }
        else {
            Some(&mut self.vec[x + (self.width * y)])
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<TileType> {
        if x >= self.width || y >= self.height {
            None
        }
        else {
            Some(self.vec[x + (self.width * y)])
        }
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<TileType> {
        let mut adjacent = vec![];

        // Be careful not to underflow!
        let x_min = if x == 0 { x } else { x - 1 };
        let x_max = x + 1;

        let y_min = if y == 0 { y } else { y - 1 };
        let y_max = y + 1;

        for i in x_min..=x_max {
            for j in y_min..=y_max {
                // Din't include the tile itself. Only the adjacent ones
                if i == x && j == y {
                    continue;
                }

                match self.get(i, j) {
                    None => {},
                    Some(value) => adjacent.push(value),
                }
            }
        }
        adjacent
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let printing_grid = self.vec.iter().map(|b| match b {
                                                Open => '.',
                                                Wooded => '|',
                                                LumberYard => '#',
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

#[derive(Debug)]
struct LumberArea {
    grid: Grid,
}

fn update_tile(tile: &mut TileType, adjacent: &Vec<TileType>) {
    match tile {
        Open => {
            if adjacent.iter().filter(|&&t| t == Wooded).count() >= 3 {
                *tile = Wooded;
            }
        },
        Wooded => {
            if adjacent.iter().filter(|&&t| t == LumberYard).count() >= 3 {
                *tile = LumberYard;
            }
        },
        LumberYard => {
            if adjacent.iter().filter(|&&t| t == LumberYard).count() == 0 ||
                    adjacent.iter().filter(|&&t| t == Wooded).count() == 0 {
                *tile = Open;
            }
        },
    }
}

impl LumberArea {
    fn advance(&mut self) {
        let mut grid_mut = self.grid.clone();
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                // Unwrap since we're definitely wihtin the grid bounds
                let mut tile = grid_mut.get_mut_ref(x ,y).unwrap();
                // Always get adjacents from the initial grid for this tick
                let adjacent = self.grid.get_adjacent(x, y);

                update_tile(&mut tile, &adjacent);
            }
        }
        self.grid = grid_mut;
    }

    fn get_resource_value(&self) -> usize {
        self.grid.vec.iter().filter(|&&t| t == LumberYard).count() *
            self.grid.vec.iter().filter(|&&t| t == Wooded).count()
    }
}

impl std::fmt::Display for LumberArea {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

fn parse_input(input: &String) -> LumberArea {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let mut vec = vec![];
    input.lines().flat_map(|l| l.chars())
        .for_each(|c|
            match c {
                '.' => vec.push(Open),
                '|' => vec.push(Wooded),
                '#' => vec.push(LumberYard),
                _ => unreachable!(),
            }
        );

    LumberArea { grid: Grid { vec, width, height } }
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    //let input = fs::read_to_string("test.txt")?;

    let mut area = parse_input(&input);
    println!("{}", area);

    // Part 1
    for _ in 0..10 {
        area.advance();
        println!("{}", area);
    }
    println!("The resource value after 10 minutes is {}",
             area.get_resource_value());

    // Part 2
    let goal_iterations = 1000000000;
    // Account for iterations made in part 1
    let mut remaining_iterations = goal_iterations - 10;

    // Simulate for a bit to see if we settle down to a steady state of
    // repated tile patterns
    for _ in 0..1000 {
        area.advance();
    }
    remaining_iterations -= 1000;

    let test_grid = area.grid.clone();

    // See if a given tile pattern is repeated
    let mut repeat_period: Option<usize> = None;
    for i in 1..10001 {
        area.advance();
        if area.grid == test_grid {
            repeat_period = Some(i);
            break;
        }
    }

    match repeat_period {
        None => {
            // This may take a long time...
            remaining_iterations -= 10000;
            for _ in 0..remaining_iterations {
                area.advance();
            }
        },
        Some(repeat_period) => {
            remaining_iterations -= repeat_period;
            let offset = remaining_iterations % repeat_period;
            for _ in 0..offset {
                area.advance();
            }
        },
    }

    println!("The resource value after {} minutes is {}",
             goal_iterations, area.get_resource_value());

    Ok(())
}
