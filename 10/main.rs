use std::fs;

type ErrorHolder = Box<std::error::Error>;

#[derive(Debug)]
struct GridSpec {
    x_min: i32,
    y_min: i32,
    x_len: usize,
    y_len: usize,
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(v: &Vec<i32>) -> Coord {
        Coord { x: v[0], y: v[1] }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl std::ops::AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        *self = *self + other
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl std::ops::SubAssign for Coord {
    fn sub_assign(&mut self, other: Coord) {
        *self = *self - other
    }
}

#[derive(Debug)]
struct Star {
    pos: Coord,
    vel: Coord,
}

impl Star {
    fn new(s: &str) -> Star {
        let pos_start = s.find('<').expect("Failed to find postion");
        let pos_finish = s.find('>').expect("Failed to find position");
        let pos_skip = pos_start + 1;
        let pos_len = pos_finish - pos_skip;
        let pos_str: String = s.chars().skip(pos_skip).take(pos_len).collect();
        let position: Vec<_> = pos_str.split(',').map(s_to_i).collect();

        let vel_start = s.rfind('<').expect("Failed to find velocity");
        let vel_finish = s.rfind('>').expect("Failed to find velocity");
        let vel_skip = vel_start + 1;
        let vel_len = vel_finish - vel_skip;
        let vel_str: String = s.chars().skip(vel_skip).take(vel_len).collect();
        let velocity: Vec<_> = vel_str.split(',').map(s_to_i).collect();

        Star { pos: Coord::new(&position), vel: Coord::new(&velocity) }
    }

    fn advance(&mut self) {
        self.pos += self.vel
    }

    fn reverse(&mut self) {
        self.pos -= self.vel
    }
}

fn s_to_i(s: &str) -> i32 {
    s.to_string().trim().parse().expect("Failed to parse char as i32")
}

fn print_stars(grid_spec: &GridSpec, stars: &Vec<Star>) {
    // Offset grid so that it starts at (0,0)
    let x_offset = -grid_spec.x_min;
    let y_offset = -grid_spec.y_min;

    let mut grid = vec![vec![' '; grid_spec.x_len]; grid_spec.y_len];

    for s in stars {
        let x = (s.pos.x + x_offset) as usize;
        let y = (s.pos.y + y_offset) as usize;
        grid[y][x] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn calculate_grid_spec(stars: &Vec<Star>) -> GridSpec {
    let msg = "Failed to find the star with max/min x/y value";
    let x_max = stars.iter().max_by_key(|s| s.pos.x).expect(msg).pos.x;
    let x_min = stars.iter().min_by_key(|s| s.pos.x).expect(msg).pos.x;
    let y_min = stars.iter().min_by_key(|s| s.pos.y).expect(msg).pos.y;
    let y_max = stars.iter().max_by_key(|s| s.pos.y).expect(msg).pos.y;

    let x_len = (x_max - x_min + 1) as usize;
    let y_len = (y_max - y_min + 1) as usize;

    GridSpec { x_min, y_min, x_len, y_len }
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    let mut stars = vec![];

    for line in input.lines() {
        stars.push(Star::new(line));
    }

    // Calculate an inital bounding box for all the stars
    let mut grid_spec = calculate_grid_spec(&stars);
    println!("{:?}", grid_spec);

    // Assume that bounding box decreases monotonically in size until the
    // letters appear, and then the box starts to grow
    let mut last_grid_len = grid_spec.x_len;

    let mut t = 0;
    loop {
        // Advance the position of all the stars by 1s
        stars.iter_mut().for_each(|s| s.advance());

        // Update the bounding box
        grid_spec = calculate_grid_spec(&stars);

        if last_grid_len < grid_spec.x_len {
            break;
        }

        last_grid_len = grid_spec.x_len;
        t += 1;

        println!("{:?} after {}s", grid_spec, t);
    }

    // In our loop we have overshot the minimum bounding box size, so rewind
    stars.iter_mut().for_each(|s| s.reverse());
    print_stars(&grid_spec, &stars);

    println!("It took {} for this pattern to appear!", t);

    Ok(())
}

