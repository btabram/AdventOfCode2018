use std::fs;
use std::collections::HashMap;

type ErrorHolder = Box<std::error::Error>;
type OpcodeFn = Fn(&mut Processor, i32, i32, i32);
type Instructions = HashMap<i32, &'static OpcodeFn>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Registers(i32, i32, i32, i32, i32, i32);

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}, {}, {}]",
               self.0, self.1, self.2, self.3, self.4, self.5)
    }
}

#[derive(Debug)]
struct Processor{
    registers: Registers,
    ip_register: i32,
}

// Macros for simple binary operations like add
macro_rules! binaryr {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, a: i32, b: i32, c: i32) {
            self.write(c, self.read(a) $op self.read(b));
        }
    }
}
macro_rules! binaryi {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, a: i32, b: i32, c: i32) {
            self.write(c, self.read(a) $op b);
        }
    }
}

// Macros for testing functions like equality testing
macro_rules! testingir {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, a: i32, b: i32, c: i32) {
            self.write(c, if a $op self.read(b) { 1 } else { 0 });
        }
    }
}
macro_rules! testingri {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, a: i32, b: i32, c: i32) {
            self.write(c, if self.read(a) $op b { 1 } else { 0 });
        }
    }
}
macro_rules! testingrr {
    ($name:ident, $op:tt) => {
        fn $name(&mut self, a: i32, b: i32, c: i32) {
            self.write(c, if self.read(a) $op self.read(b) { 1 } else { 0 });
        }
    }
}

impl std::fmt::Display for Processor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.registers)
    }
}

impl Processor {
    fn read(&self, register: i32) -> i32 {
        match register {
            0 => self.registers.0,
            1 => self.registers.1,
            2 => self.registers.2,
            3 => self.registers.3,
            4 => self.registers.4,
            5 => self.registers.5,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, register: i32, value: i32) {
        match register {
            0 => self.registers.0 = value,
            1 => self.registers.1 = value,
            2 => self.registers.2 = value,
            3 => self.registers.3 = value,
            4 => self.registers.4 = value,
            5 => self.registers.5 = value,
            _ => unreachable!(),
        }
    }

    binaryr!(addr, +);
    binaryi!(addi, +);

    binaryr!(mulr, *);
    binaryi!(muli, *);

    binaryr!(banr, &);
    binaryi!(bani, &);

    binaryr!(borr, |);
    binaryi!(bori, |);

    fn setr(&mut self, a: i32, _: i32, c: i32) {
        self.write(c, self.read(a));
    }
    fn seti(&mut self, a: i32, _: i32, c: i32) {
        self.write(c, a);
    }

    testingir!(gtir, >);
    testingri!(gtri, >);
    testingrr!(gtrr, >);

    testingir!(eqir, ==);
    testingri!(eqri, ==);
    testingrr!(eqrr, ==);

    fn run_command(&mut self, instructions: &Instructions, command: &Command) {
        let f = instructions.get(&command.opcode).expect("Unknown opcode");
        f(self, command.a, command.b, command.c);
    }

    fn ip(&self) -> i32 {
        self.read(self.ip_register)
    }

    fn run_program(&mut self, inst: &Instructions, commands: &Vec<Command>) {
        loop {
            // Run the command
            self.run_command(&inst, &commands[self.ip() as usize]);

            // Increment the instruction pointer
            self.write(self.ip_register, self.ip() + 1);

            // If the instruction pointer is now outside the program then end
            if self.ip() >= commands.len() as i32 {
                break;
            }
        }
    }
}

fn get_instructions() -> Instructions {
    let mut instructions: Instructions = HashMap::new();
    instructions.insert(0, &Processor::addr);
    instructions.insert(1, &Processor::addi);
    instructions.insert(2, &Processor::mulr);
    instructions.insert(3, &Processor::muli);
    instructions.insert(4, &Processor::banr);
    instructions.insert(5, &Processor::bani);
    instructions.insert(6, &Processor::borr);
    instructions.insert(7, &Processor::bori);
    instructions.insert(8, &Processor::setr);
    instructions.insert(9, &Processor::seti);
    instructions.insert(10, &Processor::gtir);
    instructions.insert(11, &Processor::gtri);
    instructions.insert(12, &Processor::gtrr);
    instructions.insert(13, &Processor::eqir);
    instructions.insert(14, &Processor::eqri);
    instructions.insert(15, &Processor::eqrr);
    instructions
}

#[derive(Debug)]
struct Command {
    opcode: i32,
    a: i32,
    b: i32,
    c: i32,
}

impl Command {
    fn new(opcode_name: &str, a: i32, b: i32, c: i32) -> Command {
        let opcode = match opcode_name {
            "addr" => 0,
            "addi" => 1,
            "mulr" => 2,
            "muli" => 3,
            "banr" => 4,
            "bani" => 5,
            "borr" => 6,
            "bori" => 7,
            "setr" => 8,
            "seti" => 9,
            "gtir" => 10,
            "gtri" => 11,
            "gtrr" => 12,
            "eqir" => 13,
            "eqri" => 14,
            "eqrr" => 15,
            _ => unreachable!(),
        };
        Command { opcode, a, b, c }
    }
}

fn s_to_i(s: &&str) -> i32 {
    s.parse().expect("Failed to parse str as i32")
}

fn parse_command(line: &str) -> Command {
    let split: Vec<_> = line.split(" ").collect();
    let opcode_str = split[0];

    let inputs: Vec<_> = split.iter().skip(1).map(s_to_i).collect();
    let a = inputs[0];
    let b = inputs[1];
    let c = inputs[2];

    Command::new(opcode_str, a, b, c)
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;

    let mut ip_register = None;
    let mut commands = vec![];
    for line in input.lines() {
        if line.contains("#ip ") {
            assert!(ip_register == None);
            ip_register = Some(s_to_i(&&line[4..]));
        }
        else {
            commands.push(parse_command(line));
        }
    }

    if ip_register == None {
        println!("Didn't find the instruction pointer register in the input");
        std::process::exit(1);
    }

    let instructions = get_instructions();

    // Part 1
    let mut part1_processor = Processor {
        registers: Registers(0, 0, 0, 0, 0, 0),
        ip_register: ip_register.unwrap(),
    };
    part1_processor.run_program(&instructions, &commands);
    println!("At the end of the program in, part 1, the register values are {}",
             part1_processor);

    // Part 2
    // The program loops, seemingly endlessly for Part 2. Try working through
    // the commands of the progam to see if we can work out when it will stop:

    // Running through the program from the start:
    //
    // IP is [1]
    // Add 16 to [1]
    // IP is now 17 so add 2 to [2]
    // IP is now 18 so multiply [2] by 2
    // IP is now 19 so multiply [2] by [1]
    // IP is now 20 so multiply [2] by 11
    // IP is now 21 so add 7 to [4]
    // IP is now 22 so multiply [4] by [1]
    // IP is now 23 so add 13 to [4]
    // IP is now 24 so add [4] to [2]
    //
    // Assuming [1, 0, 0, 0, 0, 0] start we now have [1, 25, 1003, 0, 167 , 0]
    //
    // IP is now 25 so add [0] to [1]
    // IP is now 27 so set [1] to [4]
    // IP is now 28 so multiply [4] by [1]
    // IP is now 29 so add [1] to [4]
    // IP is now 30 so multiply [4] by [1]
    // IP is now 31 so multiply [4] by 14
    // IP is now 32 so multiply [4] by [1]
    // IP is now 33 so add [4] to [2]
    // IP is now 34 so set [0] to 0
    // IP is now 35 so set [1] to 0
    //
    // We now have [0, 1, 10551403, 0, 10550400, 0]
    //
    // IP is now 1 so set [3] to 1
    // IP is now 2 so set [5] to 1
    //
    // >> LOOP <<
    // IP is now 3 so set [4] to [3]*[5]
    // IP is now 4 so set [4] to [4]==[2]
    //
    // We now have [0, 5, 10551403, 1, 0, 1]
    //
    // IP is now 5 so add [4] to [1]
    // IP is now 6 so add 1 to [1]
    // IP is now 8 so add 1 to [5]
    // IP is now 9 so set [4] to [5]>[2]
    // IP is now 10 so add [4] to [1]
    //
    // We now have [0, 11, 10551403, 1, 0, 2]
    //
    // IP is now 11 so set [1] to 2
    // >> LOOP <<
    // IP is now 3...

    // Loop found, consider its effects:
        // set [4] to ([3]*[5])==[2] (not true for the moment)
        // add [4] to [1] (it's zero since the test was false so no jumps)
        // add 1 to [5]
        // set [4] to [5]>[2] (not true for the moment)
        // add [4] to [1] (it's zero since the test was false so no jumps)
        // loop back to the top

    // The loop will only end if either of the testing operations succeeds.
    // The only change every iteration is [5]++.
    // At the moment [3] is 1 so ([3]*[5])==[2] will be satisfied first.

    // Lets go through that iteration of the loop:
    // We initially have [0, 3, 10551403, 1, 0, 10551403]
    //
    // IP is now 3 so set [4] to [3]*[5]
    // IP is now 4 so set [4] to [4]==[2] (true so [4] is 1)
    // IP is now 5 so add [4] to [1]
    // IP is now 7 so add [3] to [0]
    // IP is now 8 so add 1 to [5]
    // IP is now 9 so set [4] to [5]>[2] (true so [4] is 1)
    // IP is now 10 so add [4] to [1]
    // IP is now 12 so add 1 to [3]
    // IP is now 13 so set [4] to [3]>[2] (false so [4] is 0)
    // IP is now 14 so add [4] to [1]
    // IP is now 15 so set [1] to 1
    // IP is now 2 so set [5] to [1]
    // IP is now 3...
    //
    // The net result is [3]++, [0]=1 and [5]=1 so we're back to the original
    // loop but with [3]++ and [0]=1

    // The first test had the effect of adding [3] to [0] when true, but didn't
    // do anything else.
    //
    // The second test had the effect of [3]++, opening up a new test and
    // setting [5]=1. The new test is the only option to escape the loop (it's
    // also the final testing command in the program so better end the loop...)

    // This final test will be true when [3] is 10551404. While [3] is slowly
    // incrementing via the second test command the value of [0] (what we
    // actually care about) will be modified whenever the first test is true.

    // The first test will only be satisifed on 4 occasions, one of which we've
    // already dealt with above, since the only factors of 10551403 are 19 and
    // 555337. We will end up with the sum of all the factors of 10551403 in
    // [0]. This is 1 + 10551403 + 19 + 555337 = 11106760

    // At this point we enter the final iteration of the loop:
    // We initially have [11106760, 3, 10551403, 10551403, 0, 10551403]
    //
    // IP is now 3 so set [4] to [3]*[5]
    // IP is now 4 so set [4] to [4]==[2] (false so [4] is 0)
    // IP is now 5 so add [4] to [1]
    // IP is now 6 so add 1 to [1]
    // IP is now 8 so add 1 to [5]
    // IP is now 9 so set [4] to [5]>[2] (true so [4] is 1)
    // IP is now 10 so add [4] to [1]
    // IP is now 12 so add 1 to [3]
    // IP is now 13 so set [4] to [3]>[2] (true so [4] is 0)
    // IP is now 14 so add [4] to [1]
    // IP is now 16 so set multiply [1] by [1]
    // IP is now 257 -> out of range so program finishes!

    let part2_answer = 11106760;
    println!("The final value of the program, when starting with [0]=1 is {}.",
             part2_answer);

    Ok(())
}
