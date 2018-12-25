use std::fs;
use std::collections::{HashMap, HashSet};

type ErrorHolder = Box<std::error::Error>;
type OpcodeFn = Fn(&mut Processor, i32, i32, i32);
type Instructions = HashMap<i32, &'static OpcodeFn>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Registers(i32, i32, i32, i32);

#[derive(Debug)]
struct Processor{
    reg: Registers,
}

impl std::fmt::Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {}]",
               self.0, self.1, self.2, self.3)
    }
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

impl Processor {
    fn read(&self, register: i32) -> i32 {
        match register {
            0 => self.reg.0,
            1 => self.reg.1,
            2 => self.reg.2,
            3 => self.reg.3,
            _ => unreachable!(),
        }
    }

    fn write(&mut self, register: i32, value: i32) {
        match register {
            0 => self.reg.0 = value,
            1 => self.reg.1 = value,
            2 => self.reg.2 = value,
            3 => self.reg.3 = value,
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
}

#[derive(Debug)]
struct TestCase {
    before: Registers,
    opcode: i32,
    a: i32,
    b: i32,
    c: i32,
    after: Registers,
}

fn try_instruction(t: &TestCase, f: &OpcodeFn) -> bool {
    let mut p = Processor { reg: t.before };
    f(&mut p, t.a, t.b, t.c);
    p.reg == t.after
}

fn run_test_case(instructions: &Instructions, t: &TestCase) -> Vec<i32> {
    let mut matching = vec![];
    for (k, f) in instructions {
        if try_instruction(t, f) {
            matching.push(*k);
        }
    }
    matching
}

fn s_to_i(s: &str) -> i32 {
    s.parse().expect("Failed to parse str as i32")
}

fn parse_test_case(line0: &str, line1: &str, line2: &str) -> TestCase {
    let beforev: Vec<_> = line0[9..19].split(", ").map(s_to_i).collect();
    let before = Registers(beforev[0], beforev[1], beforev[2], beforev[3]);

    let input: Vec<_> = line1.split(" ").map(s_to_i).collect();
    let opcode = input[0];
    let a = input[1];
    let b = input[2];
    let c = input[3];

    let afterv: Vec<_> = line2[9..19].split(", ").map(s_to_i).collect();
    let after = Registers(afterv[0], afterv[1], afterv[2], afterv[3]);

    TestCase { before, opcode, a, b, c, after }
}

// Maintain my own map of IDs -> instructions
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

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<_> = input.lines().collect();

    let mut test_cases = vec![];
    let mut commands = vec![];
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];

        if line.contains("Before") {
            let tc = parse_test_case(lines[i], lines[i + 1], lines[i + 2]);
            test_cases.push(tc);

            i += 3;
            continue;
        }

        if !line.is_empty() {
            let command_values: Vec<_> = line.split(" ").map(s_to_i).collect();
            let opcode = command_values[0];
            let a = command_values[1];
            let b = command_values[2];
            let c = command_values[3];
            commands.push(Command { opcode, a, b, c });
        }

        i += 1;
    }

    let instructions = get_instructions();

    let test = TestCase {
        before: Registers(3, 2, 1, 1),
        opcode: 9,
        a: 2,
        b: 1,
        c: 2,
        after: Registers(3, 2, 2, 1),
    };
    assert_eq!(run_test_case(&instructions, &test).len(), 3);

    // Part 1
    let mut opcode_matches = HashMap::new();
    let mut gt3_count = 0;
    for t in &test_cases {
        let matches = run_test_case(&instructions, t);

        if matches.len() >= 3 {
            gt3_count += 1;
        }

        for m in matches {
            let entry = opcode_matches.entry(t.opcode).or_insert(HashSet::new());
            entry.insert(m);
        }
    }
    println!("There are {} samples which match 3 or more opcodes!", gt3_count);

    // Work out the mapping between the opcodes in the input and our internal
    // IDs for the different instructions
    let mut opcode_to_ids = HashMap::new();
    while opcode_to_ids.len() != 16 {

        let const_opcode_matches = opcode_matches.clone();
        let known_mappings: Vec<_> =
            const_opcode_matches.iter().filter(|(_, v)| v.len() == 1).collect();

        for (opcode, ids) in known_mappings {
            assert_eq!(ids.len(), 1);
            let known_id = ids.iter().next().unwrap();

            opcode_to_ids.insert(*opcode, *known_id);
            opcode_matches.values_mut().for_each(|v| { v.remove(known_id); });
        }
    }

    // Part 2
    let mut p = Processor { reg: Registers(0, 0, 0, 0) };
    for command in commands {
        let id = opcode_to_ids.get(&command.opcode).expect("Unexpected opcode");
        let f = instructions.get(id).expect("Unexpected instruction ID");
        f(&mut p, command.a, command.b, command.c);
    }
    println!("After executing the program the registers are {}", p.reg);

    Ok(())
}
