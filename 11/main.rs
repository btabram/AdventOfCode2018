use std::fs;

type ErrorHolder = Box<std::error::Error>;

#[derive(Debug, Clone)]
struct FuelCellGrid {
    fuel_cells: Vec<i32>,
    size: usize,
}

impl FuelCellGrid {
    fn get_power_level(&self, x: usize, y :usize) -> i32 {
        // Fuel cells are index from 1 not 0
        let x_i = x - 1;
        let y_i = y - 1;
        // Hard coding the 300x300 grid size
        self.fuel_cells[self.size*x_i + y_i]
    }

    fn set_power_level(&mut self, x: usize, y :usize, power: i32) {
        // Fuel cells are index from 1 not 0
        let x_i = x - 1;
        let y_i = y - 1;
        // Hard coding the 300x300 grid size
        self.fuel_cells[self.size*x_i + y_i] = power;
    }

    fn sum_square_power(&self, size: usize, x_c: usize, y_c: usize) -> i32 {
        let mut power_sum = 0;
        for x in x_c..x_c+size {
            for y in y_c..y_c+size {
                power_sum += self.get_power_level(x, y);
            }
        }
        power_sum
    }
}

#[derive(Debug)]
struct SummedGrid {
    grid: FuelCellGrid,
    summed_square_size: usize,
}

impl SummedGrid {
    // Transform a grid of (size-1)x(size-1) sums to a grid of size x size sums
    fn calculate_next_square_sum(&mut self, fcg: &FuelCellGrid,
                                 size: usize, x_c: usize, y_c: usize) {
        assert!(self.summed_square_size + 1 == size);

        let mut power_sum = self.grid.get_power_level(x_c, y_c);

        for x in x_c..(x_c + size) {
            power_sum += fcg.get_power_level(x, y_c + size - 1);
        }

        // Remember to only sum the corner cell once!
        for y in y_c..(y_c + size - 1) {
            power_sum += fcg.get_power_level(x_c + size - 1, y);
        }

        self.grid.set_power_level(x_c, y_c, power_sum);
    }
}

fn get_hundred_digit(i: i32) -> i32 {
    let s = i.to_string();

    // Return 0 if there's no hundred digit
    if s.len() <= 3 {
        return 0;
    }

    s.chars().rev().skip(2).next().expect("Didn't find expected 3rd ditgit")
        .to_digit(10).expect("Faild to parse char as digit") as i32
}

fn calculate_power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    let rack_id = x + 10;

    let mut power = rack_id * y;

    power += grid_serial_number;

    power *= rack_id;

    power = get_hundred_digit(power);

    power -= 5;

    power
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    let gsn: i32
        = input.trim().parse().expect("Failed to parse grid serial number");
    println!("The grid serial number is {}\n", gsn);

    let mut fuel_cells = vec![];
    for x in 1..301 {
        for y in 1..301 {
            fuel_cells.push(calculate_power_level(x, y, gsn));
        }
    }

    let fcg = FuelCellGrid { fuel_cells, size: 300 };

    // Part 2
    let mut max_3x3_power = 0;
    let mut max_3x3_power_square = (0, 0);
    for x in 1..299 {
        for y in 1..299 {
            let power = fcg.sum_square_power(3, x, y);
            if power > max_3x3_power {
                max_3x3_power = power;
                max_3x3_power_square = (x, y);
            }
        }
    }

    // Part 2
    // Use the previous (smaller) square power sums to speed up calculation of
    // later (larger) square power sums
    let mut summed_grid = SummedGrid {
                              grid: fcg.clone(),
                              summed_square_size: 1
                          };

    let mut max_power = 0;
    let mut max_power_square = (0, 0);
    let mut max_power_size = 0;

    // size == 1 case
    for x in 1..301 {
        for y in 1..301 {
            let power = fcg.get_power_level(x, y);
            if power > max_power {
                max_power = power;
                max_power_square = (x, y);
                max_power_size = 1;
            }
        }
    }


    for size in 2..301 {
        println!("Checking squares of size {}...", size);
        for x in 1..(301 - size + 1) {
            for y in 1..(301 - size + 1) {

                summed_grid.calculate_next_square_sum(&fcg, size, x, y);

                let power = summed_grid.grid.get_power_level(x, y);
                if power > max_power {
                    max_power = power;
                    max_power_square = (x, y);
                    max_power_size = size;
                }
            }
        }
        summed_grid.summed_square_size += 1;
    }

    println!("\nThe max 3x3 power is {} with top left corner {:?}",
             max_3x3_power, max_3x3_power_square);

    println!("The max power is {} with top left corner {:?} and square size {}",
             max_power, max_power_square, max_power_size);

    Ok(())
}

