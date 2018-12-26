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

    Ok(())
}
