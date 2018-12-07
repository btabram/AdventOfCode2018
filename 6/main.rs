use std::fs;

#[derive(Debug)]
struct Grid {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    points: Vec<PointValue>,
}

impl Grid {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Grid {
        let mut points = vec![];
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                points.push(PointValue::new(x, y));
            }
        }
        Grid { min_x, min_y, max_x, max_y, points }
    }

    fn is_on_edge(&self, point: &Point) -> bool {
        point.x == self.min_x || point.y == self.min_y || point.x == self.max_x || point.y == self.max_y
    }

    fn iter(&self) -> GridIterator {
        GridIterator { iterator: self.points.iter() }
    }

    fn iter_mut(&mut self) -> GridIteratorMut {
        GridIteratorMut { iterator: self.points.iter_mut() }
    }
}

struct GridIterator<'a> {
    iterator: std::slice::Iter<'a, PointValue>,
}

impl <'a> Iterator for GridIterator<'a>  {
    type Item = &'a PointValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

struct GridIteratorMut<'a> {
    iterator: std::slice::IterMut<'a, PointValue>,
}

impl <'a> Iterator for GridIteratorMut<'a>  {
    type Item = &'a mut PointValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[derive(Debug)]
struct Coordinate {
    point: Point,
    area: Option<i32>, // None means infinte (and hence invalid) area
}


impl Coordinate {
    fn distance(&self, other: &Point) -> i32 {
        (self.point.x - other.x).abs() + (self.point.y - other.y).abs()
    }

    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { point: Point { x, y }, area: Some(0) }
    }
}

#[derive(Debug)]
struct PointValue {
    point: Point,
    value: i32
}

impl PointValue {
    fn new(x: i32, y: i32) -> PointValue {
        PointValue { point: Point { x, y}, value: 0 }
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut coords = vec![];
    let mut min_x = 1000;
    let mut min_y = 1000;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let split_line: Vec<_> = line.split(",").collect();
        let x: i32 = split_line[0].trim().to_string().parse().expect("Parse failed!");
        let y: i32 = split_line[1].trim().to_string().parse().expect("Parse failed!");

        if x < min_x { min_x = x; }
        if y < min_y { min_y = y; }

        if x > max_x { max_x = x; }
        if y > max_y { max_y = y; }

        coords.push(Coordinate::new(x, y)) ;
    }

    let mut grid = Grid::new(min_x, min_y, max_x, max_y);

    // Part 1
    for p in grid.iter() {

        let mut min_distance = 1000;
        let mut next_min_distance = 1000;
        let mut closest_coord_index = None;

        for (i, coord) in coords.iter().enumerate() {

            let distance = coord.distance(&p.point);
            if distance < min_distance {
                min_distance = distance;
                closest_coord_index = Some(i);
            }
            // Keep track of the next closest distance so we know if there's joint-closest coords
            else if distance < next_min_distance {
                next_min_distance = distance;
            }
        }

        let ref mut cc = coords[closest_coord_index.expect("Failed to find closest coord...")];
        match cc.area {
            Some(a) => {
                // If a coordinate is closest to a point on the edge it has infinte area
                if grid.is_on_edge(&p.point) {
                    cc.area = None
                }
                // Only counts for area if a coodinate is uniquely closest
                else if min_distance != next_min_distance {
                    cc.area = Some(a + 1);
                }
            },
            None => {},
        }
    }
    let largest_area = coords.iter().max_by_key( |c| match c.area { Some(a) => a, None => 0, });
    println!("The coordinate with the largest area is: {:?}", largest_area);

    // Part 2
    for p in grid.iter_mut() {
        for coord in &coords {
            p.value += coord.distance(&p.point);
        }
    }
    let safe_area = grid.iter().filter(|pv| pv.value < 10000).count();
    println!("The size of the safe area (total distance to all coords < 10000) is: {:?}", safe_area);

    Ok(())
}
