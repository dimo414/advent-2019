use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use std::collections::{VecDeque, BTreeMap};

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Opcode {
    ADD,
    MUL,
    INPUT,
    OUTPUT,
    JIT,
    JIF,
    LT,
    EQ,
    EXIT,
}

impl Opcode {
    fn code(&self) -> i32 {
        use Opcode::*;
        match *self {
            ADD => 1,
            MUL => 2,
            INPUT => 3,
            OUTPUT => 4,
            JIT => 5,
            JIF => 6,
            LT => 7,
            EQ => 8,
            EXIT => 99,
        }
    }

    fn lookup(code: i32) -> Option<Opcode> {
        use Opcode::*;
        match code % 100 {
            1 => Some(ADD),
            2 => Some(MUL),
            3 => Some(INPUT),
            4 => Some(OUTPUT),
            5 => Some(JIT),
            6 => Some(JIF),
            7 => Some(LT),
            8 => Some(EQ),
            99 => Some(EXIT),
            _ => None,
        }
    }

    fn parameters(&self) -> usize {
        use Opcode::*;
        match *self {
            EXIT => 0,
            INPUT | OUTPUT => 1,
            JIT | JIF => 2,
            ADD | MUL | LT | EQ => 3,
        }
    }

    fn modes(&self, code: i32) -> Vec<bool> {
        let mut modes = code / 100;
        let mut ret = Vec::new();
        for _ in {0..self.parameters()} {
            let mode = modes % 10;
            assert!(mode == 0 || mode == 1);
            ret.push(mode == 1);
            modes /= 10;
        }
        ret
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({})", self, self.code())
    }
}

pub struct Machine {
    state: Vec<i32>,
    pointer: usize,
    pointer_moved: bool,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

impl Machine {
    pub fn new(state: &[i32]) -> Machine {
        Machine { state: state.to_vec(), pointer: 0, pointer_moved: false, input: VecDeque::new(), output: VecDeque::new() }
    }

    pub fn send_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    pub fn read_output(&mut self) -> Vec<i32> {
        self.output.drain(..).collect()
    }

    pub fn read_state(&self, address: usize) -> i32 {
        self.state[address]
    }

    pub fn set_state(&mut self, address: usize, value: i32) {
        self.state[address] = value;
    }

    #[cfg(test)]
    fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }

    pub fn run(&mut self) {
        self.debug(&mut NoopDebugger);
    }

    pub fn debug(&mut self, debugger: &mut impl Debugger) {
        loop {
            let code = self.state[self.pointer];
            let opcode = Opcode::lookup(code)
                .expect(&format!("Invalid opcode {} at {}", self.state[self.pointer], self.pointer));
            let params = self.state[self.pointer+1..self.pointer+1+opcode.parameters()].to_vec();
            let modes = opcode.modes(code);

            let proceed = debugger.on_exec(self.pointer, opcode, &params, &modes, &self.state);
            if !proceed { break; }

            match opcode {
                Opcode::ADD => self.add(&params, &modes),
                Opcode::MUL => self.mul(&params, &modes),
                Opcode::INPUT => self.input(&params, &modes),
                Opcode::OUTPUT => self.output(&params, &modes),
                Opcode::JIT => self.jump_if_true(&params, &modes),
                Opcode::JIF => self.jump_if_false(&params, &modes),
                Opcode::LT => self.less_than(&params, &modes),
                Opcode::EQ => self.equals(&params, &modes),
                Opcode::EXIT => break,
            }

            if ! self.pointer_moved {
                self.pointer += 1 + opcode.parameters();
            }
            self.pointer_moved = false;
        }
        debugger.on_halt(self.pointer);
    }

    fn read(&mut self, params: &[i32], modes: &[bool], index: usize) -> i32 {
        if modes[index] {
            params[index]
        } else {
            self.state[params[index] as usize]
        }
    }

    fn write(&mut self, params: &[i32], modes: &[bool], index: usize, value: i32) {
        assert!(!modes[index]); // can't write-to-value...
        self.state[params[index] as usize] = value;
    }

    fn move_pointer(&mut self, new_pointer: usize) {
        self.pointer = new_pointer;
        self.pointer_moved = true;
    }

    fn add(&mut self, params: &[i32], modes: &[bool]) {
        let a = self.read(params, modes, 0);
        let b = self.read(params, modes, 1);
        self.write(params, modes, 2, a + b);
    }

    fn mul(&mut self, params: &[i32], modes: &[bool]) {
        let a = self.read(params, modes, 0);
        let b = self.read(params, modes, 1);
        self.write(params, modes, 2, a * b);
    }

    fn input(&mut self, params: &[i32], modes: &[bool]) {
        let input = self.input.pop_front().expect("No input left");
        self.write(params, modes, 0, input);
    }

    fn output(&mut self, params: &[i32], modes: &[bool]) {
        let value = self.read(params, modes, 0);
        self.output.push_back(value);
    }

    fn jump_if_true(&mut self, params: &[i32], modes: &[bool]) {
        if self.read(params, modes, 0) != 0 {
            let dest = self.read(params, modes, 1) as usize;
            self.move_pointer(dest);
        }
    }

    fn jump_if_false(&mut self, params: &[i32], modes: &[bool]) {
        if self.read(params, modes, 0) == 0 {
            let dest = self.read(params, modes, 1) as usize;
            self.move_pointer(dest);
        }
    }

    fn less_than(&mut self, params: &[i32], modes: &[bool]) {
        let value = if self.read(params, modes, 0) < self.read(params, modes, 1) {
            1
        } else {
            0
        };
        self.write(params, modes, 2, value);
    }

    fn equals(&mut self, params: &[i32], modes: &[bool]) {
        let value = if self.read(params, modes, 0) == self.read(params, modes, 1) {
            1
        } else {
            0
        };
        self.write(params, modes, 2, value);
    }
}

impl FromStr for Machine {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(",").map(|n| n.parse::<i32>()).collect::<Result<Vec<i32>, _>>()?;
        Ok(Machine::new(&v))
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();

        if self.pointer > 0 {
            for n in self.state[0..self.pointer-1].iter() {
                write!(&mut out, "{}\t", n)?
            }
            write!(&mut out, "{}\n", self.state[self.pointer-1])?;
        }

        let mut pointer = self.pointer;
        while pointer < self.state.len() {
            let rawcode = self.state[pointer];
            let opcode = Opcode::lookup(rawcode);
            if opcode.is_none() { break; }
            let opcode = opcode.expect("Cannot be none");
            write!(&mut out, "{}", opcode)?;
            for _ in {0..opcode.parameters()} {
                pointer += 1;
                if self.state.len() <= pointer {
                    write!(&mut out, "\n")?;
                    break;
                }
                write!(&mut out, "\t{}", self.state[pointer])?;
            }
            write!(&mut out, "\n")?;
            pointer += 1;
        }

        if pointer < self.state.len() {
            for n in self.state[pointer..self.state.len()-1].iter() {
                write!(&mut out, "{}\t", n)?
            }
            write!(&mut out, "{}\n", self.state[self.state.len()-1])?;
        }

        write!(f, "{}", out)
    }
}

pub trait Debugger {
    fn on_exec(&mut self, pointer: usize, opcode: Opcode, params: &[i32], modes: &[bool], state: &[i32]) -> bool;

    fn on_halt(&mut self, pointer: usize) { let _=pointer; }
}

struct NoopDebugger;
impl Debugger for NoopDebugger {
    fn on_exec(&mut self, _: usize, _: Opcode, _: &[i32], _: &[bool], _: &[i32]) -> bool { true }
}

pub struct ExecCounter {
    counts: BTreeMap<Opcode, usize>,
}

#[allow(dead_code)]
impl ExecCounter {
    pub fn new() -> ExecCounter {
        ExecCounter { counts: BTreeMap::new() }
    }

    pub fn counts(&self) -> &BTreeMap<Opcode, usize> {
        &self.counts
    }

    pub fn total(&self) -> usize {
        self.counts.values().sum()
    }
}

impl Debugger for ExecCounter {
    fn on_exec(&mut self, _: usize, opcode: Opcode, _: &[i32], _: &[bool], _: &[i32]) -> bool {
        let count = self.counts.entry(opcode).or_insert(0);
        *count += 1;
        true
    }

    fn on_halt(&mut self, _: usize) {
        self.counts.entry(Opcode::EXIT).or_insert(0);
    }
}

pub struct ExecLogger {
    steps: usize,
    halt_after: usize,
    should_log: Box<Fn(Opcode, usize) -> bool>,
}

#[allow(dead_code)]
impl ExecLogger {
    pub fn new(halt_after: usize, should_log: Box<Fn(Opcode, usize) -> bool>) -> ExecLogger {
        ExecLogger{ steps: 0, halt_after, should_log }
    }

    pub fn halt_after(halt_after: usize) -> ExecLogger {
        ExecLogger::new(halt_after, Box::new(|_, _| true))
    }
}

impl Debugger for ExecLogger {
    fn on_exec(&mut self, pointer: usize, opcode: Opcode, params: &[i32], modes: &[bool], state: &[i32]) -> bool {
        self.steps += 1;
        if (self.should_log)(opcode, self.steps) {
            let mut out = String::new();

            write!(&mut out, "{:5}:{:<5} {:>10}", pointer, self.steps, opcode.to_string()).unwrap();
            for i in {0..params.len()} {
                let param = if modes[i] {
                    format!("{}", params[i])
                } else {
                    format!("{}[{}]", params[i], state[params[i] as usize])
                };
                write!(&mut out, "\t{:>14}", param).unwrap();
            }

            println!("{}", out);
        }
        self.steps <= self.halt_after
    }

    fn on_halt(&mut self, ip: usize) {
        println!("HALT: {}", ip);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test! { add_mul, (program, expected), {
        let mut machine: Machine = program.parse().expect("Invalid");
        machine.run();
        let expected: Machine = expected.parse().expect("Invalid");
        // TODO stop directly reading the state field
        assert_eq!(machine.state, expected.state);
    }}
    add_mul! {
        // From Day 2
        a: ("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50"),
        b: ("1,0,0,0,99", "2,0,0,0,99"),
        c: ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
        d: ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
    }

    parameterized_test! { io, (program, input, expected_output), {
        let mut machine: Machine = program.parse().expect("Invalid");
        for i in input {
            machine.send_input(i);
        }
        machine.run();
        assert_eq!(machine.read_output(), expected_output);
    }}
    io! {
        // From Day 5
        a: ("3,0,4,0,99", vec!(10), vec!(10)),
    }

    parameterized_test! { immediate_mode, (program, expected), {
        let mut machine: Machine = program.parse().expect("Invalid");
        machine.run();
        let expected: Machine = expected.parse().expect("Invalid");
        // TODO stop directly reading the state field
        assert_eq!(machine.state, expected.state);
    }}
    immediate_mode! {
        // From Day 5
        a: ("1002,4,3,4,33", "1002,4,3,4,99"),
        b: ("1101,100,-1,4,0", "1101,100,-1,4,99"),
    }

    parameterized_test! { lteq, (program, true_input, false_input), {
        let mut machine: Machine = program.parse().expect("Invalid");
        machine.send_input(true_input);
        machine.run();
        assert_eq!(machine.read_output(), vec!(1));

        let mut machine: Machine = program.parse().expect("Invalid");
        machine.send_input(false_input);
        machine.run();
        assert_eq!(machine.read_output(), vec!(0));
    }}
    lteq! {
        // From Day 5
        a: ("3,9,8,9,10,9,4,9,99,-1,8", 8, 12),
        b: ("3,9,7,9,10,9,4,9,99,-1,8", 5, 11),
        c: ("3,3,1108,-1,8,3,4,3,99", 8, 10),
        d: ("3,3,1107,-1,8,3,4,3,99", 4, 9),
    }

    parameterized_test! { jumps, program, {
        let mut machine: Machine = program.parse().expect("Invalid");
        println!("{}", machine);
        machine.send_input(0);
        machine.run();
        assert_eq!(machine.read_output(), vec!(0));

        let mut machine: Machine = program.parse().expect("Invalid");
        machine.send_input(5);
        machine.run();
        assert_eq!(machine.read_output(), vec!(1));
    }}
    jumps! {
        // From Day 5
        a: "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9",
        b: "3,3,1105,-1,9,1101,0,0,12,4,12,99,1",
    }


    parameterized_test! { display, (input, pointer, expected), {
        let mut machine = Machine::new(&input);
        machine.set_pointer(pointer);
        assert_eq!(machine.to_string(), expected);
    }}
    display! {
        a: (vec!(1,9,10,3,2,3,11,0,99,30,40,50), 0, "ADD(1)\t9\t10\t3\nMUL(2)\t3\t11\t0\nEXIT(99)\n30\t40\t50\n"),
        b: (vec!(1,9,10,3,2,3,11,0,99,30,40,50), 4, "1\t9\t10\t3\nMUL(2)\t3\t11\t0\nEXIT(99)\n30\t40\t50\n"),
    }
}