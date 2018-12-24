use std::fs;

type ErrorHolder = Box<std::error::Error>;

macro_rules! unexpected {
    ($c:expr) => {{
        println!("Error! Unexpected char {}", $c);
        std::process::exit(1);
    }};
}

#[derive(Debug, Clone, Copy)]
enum Directions {
    Straight,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    symbol: char,
    next_turn: Directions,
    has_moved: bool,
}

impl Cart {
    fn new(x: usize, y: usize, symbol: char) -> Cart {
        // Carts always turn left first
        Cart { x, y, symbol, next_turn: Directions::Left, has_moved: false }
    }

    fn get_next_turn(&mut self) -> Directions {
        let next_turn = self.next_turn;
        match next_turn {
            Directions::Left => self.next_turn = Directions::Straight,
            Directions::Straight => self.next_turn = Directions::Right,
            Directions::Right => self.next_turn = Directions::Left,
        }
        next_turn
    }

    fn move_to_next_pos(&mut self) -> (usize, usize) {
        // Calculate next position
        let (x, y) = match self.symbol {
            '<' => (self.x - 1, self.y),
            '>' => (self.x + 1, self.y),
            '^' => (self.x, self.y - 1),
            'v' => (self.x, self.y + 1),
            c => unexpected!(c),
        };

        // Update internal position
        self.x = x;
        self.y = y;
        self.has_moved = true;

        // Return new position
        (x, y)
    }
}

#[derive(Debug, Clone)]
struct Map {
    map_vec: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y :usize) -> char {
        self.map_vec[x + (self.width * y)]
    }

    fn set(&mut self, x: usize, y :usize, value: char) {
        self.map_vec[x + (self.width * y)] = value;
    }
}

#[derive(Debug, Clone)]
struct Tracks {
    map: Map,
    carts: Vec<Cart>,
}

impl Tracks {
    // Sort the carts vector so that it's in the order the carts move in
    fn sort_carts(&mut self) {
        // Sort by row, with column only mattering for ties
        self.carts.sort_by_key(|c| c.x + (c.y * 1000000));
    }

    // Cleanup required at the end of each tick
    fn finish_tick(&mut self) {
        self.carts.iter_mut().for_each(|c| c.has_moved = false);
    }

    fn move_carts(&mut self) -> Option<(usize, usize)>  {
        self.sort_carts();
        let mut carts_mut = self.carts.clone();
        let mut collision_coord = None;

        for (i, cart) in self.carts.iter().enumerate() {
            // Skip any carts that have already moved this ticket
            if cart.has_moved {
                continue;
            }

            let mut c = cart.clone();

            let (x, y) = c.move_to_next_pos();
            let next_track = self.map.get(x, y);

            match next_track {
                '-' => {},
                '|' => {},
                '/' => {
                    // Make a turn
                    match c.symbol {
                        '>' => c.symbol = '^',
                        '<' => c.symbol = 'v',
                        '^' => c.symbol = '>',
                        'v' => c.symbol = '<',
                        c => unexpected!(c),
                    }
                },
                '\\' => {
                    // Make a turn
                    match c.symbol {
                        '>' => c.symbol = 'v',
                        '<' => c.symbol = '^',
                        '^' => c.symbol = '<',
                        'v' => c.symbol = '>',
                        c => unexpected!(c),
                    }
                },
                '+' => {
                    // Make an appropriate turn at the intersection
                    match c.get_next_turn() {
                        Directions::Left => {
                            match c.symbol {
                                '>' => c.symbol = '^',
                                '<' => c.symbol = 'v',
                                '^' => c.symbol = '<',
                                'v' => c.symbol = '>',
                                c => unexpected!(c),
                            }
                        },
                        Directions::Straight => {},
                        Directions::Right => {
                            match c.symbol {
                                '>' => c.symbol = 'v',
                                '<' => c.symbol = '^',
                                '^' => c.symbol = '>',
                                'v' => c.symbol = '<',
                                c => unexpected!(c),
                            }
                        },
                    }
                }
                c => unexpected!(c),
            }

            // Check for collisions
            for other_c in &carts_mut {
                if c.x == other_c.x && c.y == other_c.y {
                    collision_coord = Some((x, y));
                    break;
                }
            }

            // Update carts_mut
            carts_mut[i] = c;

            // Break if there's been a collision
            if collision_coord != None {
                break;
            }
        }

        self.carts = carts_mut;
        collision_coord
    }
}

impl std::fmt::Display for Tracks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut printing_map = self.map.clone();
        for c in &self.carts {
            printing_map.set(c.x, c.y, c.symbol);
        }
        let mut map_string = String::new();
        for row_index in 0..self.map.height {
            let mut row: String = printing_map.map_vec.iter()
                                    .skip(row_index * self.map.width)
                                    .take(self.map.width).collect();
            row.push('\n');
            map_string.push_str(&row);
        }
        write!(f, "{}", map_string)
    }
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut map_vec = vec![];
    let mut carts = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut map_c = c;
            match c {
                // Replace carts with their underlying track to get a complete
                // map of the track
                '<' => {
                    map_c = '-';
                    carts.push(Cart::new(x, y, c));
                },
                '>' => {
                    map_c = '-';
                    carts.push(Cart::new(x, y, c));
                },
                '^' => {
                    map_c = '|';
                    carts.push(Cart::new(x, y, c));
                },
                'v' => {
                    map_c = '|';
                    carts.push(Cart::new(x, y, c));
                },
                _ => {},
            }
            map_vec.push(map_c);
        }
    }

    assert!(map_vec.len() == height * width);

    let map = Map { map_vec, width, height };
    let tracks = Tracks { map, carts };
    println!("{}", tracks);

    // Part 1 where we stop on collisions
    let mut part1_tracks = tracks.clone();
    let part1_answer;
    loop {
        match part1_tracks.move_carts() {
            // A coord means that there was a collision while moving the carts
            Some((x, y)) => {
                part1_answer = Some((x,y));
                break;
            },
            None => {},
        }
        part1_tracks.finish_tick();

        //println!("{}", part1_tracks);
    }

    // Part 2 where we don't stop on collisions
    let mut part2_tracks = tracks.clone();
    let part2_answer;
    loop {
        // Complete one ticket inside this loop, we may need to call move_carts
        // multiple times since we have to break out when there's a collision to
        // remove the carts in question
        loop {
            let mut collision_coord = None;
            match part2_tracks.move_carts() {
                Some((x, y)) => collision_coord = Some((x, y)),
                None => {},
            }

            if collision_coord == None {
                // Finished ticket without collisions
                break;
            } else {
                // Resolve collision and keep going
                let (x, y) = collision_coord.unwrap();
                let len_before = part2_tracks.carts.len();
                part2_tracks.carts.retain(|c| c.x != x || c.y != y);
                assert!(part2_tracks.carts.len() + 2 == len_before);
            }
        }
        part2_tracks.finish_tick();
        if part2_tracks.carts.len() == 1 {
            let ref c = part2_tracks.carts[0];
            part2_answer = Some((c.x, c.y));
            break;
        }

        //println!("{}", part2_tracks);
    }

    println!("First collision at {:?}!", part1_answer);

    println!("The last remaining cart is located at {:?}!", part2_answer);

    Ok(())
}

