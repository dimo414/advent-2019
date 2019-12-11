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
    RELBASE,
    EXIT,
}

impl Opcode {
    fn code(&self) -> i64 {
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
            RELBASE => 9,
            EXIT => 99,
        }
    }

    fn lookup(code: i64) -> Option<Opcode> {
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
            9 => Some(RELBASE),
            99 => Some(EXIT),
            _ => None,
        }
    }

    fn parameters(&self) -> usize {
        use Opcode::*;
        match *self {
            EXIT => 0,
            INPUT | OUTPUT | RELBASE => 1,
            JIT | JIF => 2,
            ADD | MUL | LT | EQ => 3,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // https://github.com/rust-lang/rust/issues/67162
        fmt::Display::fmt(&format!("{:?}({})", self, self.code()), f)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Address {
    Reference(usize),
    Immediate(i64),
    Relative(isize),
}

pub struct Machine {
    state: Vec<i64>,
    pointer: usize,
    relative_base: isize,
    pointer_moved: bool,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    debugger: Box<Debugger>,
}

impl Machine {
    pub fn new(state: &[i64]) -> Machine {
        Machine {
            state: state.to_vec(),
            pointer: 0,
            relative_base: 0,
            pointer_moved: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
            debugger: Box::new(NoopDebugger)
        }
    }

    pub fn send_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn read_output(&mut self) -> Vec<i64> {
        self.output.drain(..).collect()
    }

    pub fn read_state(&self, address: usize) -> i64 {
        self.state[address]
    }

    pub fn set_state(&mut self, address: usize, value: i64) {
        self.state[address] = value;
    }

    // Not sure if there's a good way to hide the Box from the caller; impl Debugger triggers a
    // 'static lifetime requirement
    #[allow(dead_code)]
    pub fn set_debugger(&mut self, debugger: Box<Debugger>) {
        self.debugger = debugger;
    }

    #[cfg(test)]
    fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }

    pub fn run(&mut self) {
        self.run_internal(None);
    }

    pub fn run_until(&mut self, output: usize) -> Option<Vec<i64>> {
        self.run_internal(Some(output))
    }

    fn run_internal(&mut self, output: Option<usize>) -> Option<Vec<i64>> {
        loop {
            if let Some(o) = output {
                if self.output.len() == o {
                    return Some(self.read_output());
                }
                assert!(self.output.len() < o);
            }
            let code = self.state[self.pointer];
            let opcode = Opcode::lookup(code)
                .expect(&format!("Invalid opcode {} at {}", self.state[self.pointer], self.pointer));
            let params = self.compute_params(opcode, code / 100);

            let proceed = self.debugger.on_exec(opcode, &params, &self.state, self.pointer, self.relative_base);
            if !proceed { break; }

            match opcode {
                Opcode::ADD => self.add(&params),
                Opcode::MUL => self.mul(&params),
                Opcode::INPUT => self.input(&params),
                Opcode::OUTPUT => self.output(&params),
                Opcode::JIT => self.jump_if_true(&params),
                Opcode::JIF => self.jump_if_false(&params),
                Opcode::LT => self.less_than(&params),
                Opcode::EQ => self.equals(&params),
                Opcode::RELBASE => self.update_relative_base(&params),
                Opcode::EXIT => break,
            }

            if ! self.pointer_moved {
                self.pointer += 1 + opcode.parameters();
            }
            self.pointer_moved = false;
        }
        self.debugger.on_halt(self.pointer);
        None
    }

    fn compute_params(&self, opcode: Opcode, modes_mask: i64) -> Vec<Address> {
        let params = self.state[self.pointer+1..self.pointer+1+opcode.parameters()].to_vec();
        let mut modes_mask = modes_mask;

        let mut ret = Vec::new();
        for i in 0..opcode.parameters() {
            let address = match modes_mask % 10 {
                0 => Address::Reference(params[i] as usize),
                1 => Address::Immediate(params[i]),
                2 => Address::Relative(params[i] as isize),
                _ => panic!(format!("Invalid mode: {}", modes_mask % 10)),
            };
            ret.push(address);
            modes_mask /= 10;
        }
        ret
    }

    fn read(&self, param: Address) -> i64 {
        match param {
            Address::Reference(a) => *self.state.get(a).unwrap_or(&0),
            Address::Immediate(v) => v,
            Address::Relative(r) => *self.state.get((self.relative_base + r) as usize).unwrap_or(&0),
        }
    }

    fn write(&mut self, param: Address, value: i64) {
        let address = match param {
            Address::Reference(a) => a,
            Address::Immediate(_) => panic!("Can't write in immediate mode"),
            Address::Relative(r) => (self.relative_base + r) as usize,
        };
        if self.state.len() <= address {
            let len = self.state.len();
            self.state.extend(vec![0; address - len + 1]);
        }
        self.state[address] = value;
    }

    fn move_pointer(&mut self, new_pointer: usize) {
        self.pointer = new_pointer;
        self.pointer_moved = true;
    }

    fn add(&mut self, params: &[Address]) {
        self.write(params[2], self.read(params[0]) + self.read(params[1]));
    }

    fn mul(&mut self, params: &[Address]) {
        self.write(params[2], self.read(params[0]) * self.read(params[1]));
    }

    fn input(&mut self, params: &[Address]) {
        let input = self.input.pop_front().expect("No input left");
        self.write(params[0], input);
    }

    fn output(&mut self, params: &[Address]) {
        self.output.push_back(self.read(params[0]));
    }

    fn jump_if_true(&mut self, params: &[Address]) {
        if self.read(params[0]) != 0 {
            self.move_pointer(self.read(params[1]) as usize);
        }
    }

    fn jump_if_false(&mut self, params: &[Address]) {
        if self.read(params[0]) == 0 {
            self.move_pointer(self.read(params[1]) as usize);
        }
    }

    fn less_than(&mut self, params: &[Address]) {
        let value = if self.read(params[0]) < self.read(params[1]) { 1 } else { 0 };
        self.write(params[2], value);
    }

    fn equals(&mut self, params: &[Address]) {
        let value = if self.read(params[0]) == self.read(params[1]) { 1 } else { 0 };
        self.write(params[2], value);
    }

    fn update_relative_base(&mut self, params: &[Address]) {
        self.relative_base += self.read(params[0]) as isize;
    }
}

impl FromStr for Machine {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(",").map(|n| n.parse::<i64>()).collect::<Result<Vec<i64>, _>>()?;
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
    fn on_exec(&mut self, opcode: Opcode, params: &[Address], state: &[i64], pointer: usize, relative_base: isize) -> bool;

    fn on_halt(&mut self, pointer: usize) { let _=pointer; }
}

struct NoopDebugger;
impl Debugger for NoopDebugger {
    fn on_exec(&mut self, _: Opcode, _: &[Address], _: &[i64], _: usize, _: isize) -> bool { true }
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
    fn on_exec(&mut self, opcode: Opcode, _: &[Address], _: &[i64], _: usize, _: isize) -> bool {
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
    should_log: Box<dyn Fn(Opcode, usize) -> bool>,
}

#[allow(dead_code)]
impl ExecLogger {
    pub fn new(halt_after: usize, should_log: Box<dyn Fn(Opcode, usize) -> bool>) -> ExecLogger {
        ExecLogger{ steps: 0, halt_after, should_log }
    }

    pub fn halt_after(halt_after: usize) -> ExecLogger {
        ExecLogger::new(halt_after, Box::new(|_, _| true))
    }
}

impl Debugger for ExecLogger {
    fn on_exec(&mut self, opcode: Opcode, params: &[Address], state: &[i64], pointer: usize, relative_base: isize) -> bool {
        self.steps += 1;
        if (self.should_log)(opcode, self.steps) {
            let mut out = String::new();

            write!(&mut out, "{:5}:{:<5} {:>10}", pointer, self.steps, opcode).unwrap();
            for param in params {
                let formatted = match param {
                    Address::Reference(a) => format!("{}[{}]", a, state.get(*a).unwrap_or(&0)),
                    Address::Immediate(v) => format!("{}", v),
                    Address::Relative(r) => format!("{}{:+}[{}]", r, relative_base, state.get((r+relative_base) as usize).unwrap_or(&0)),
                };
                write!(&mut out, "\t{:>14}", formatted).unwrap();
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

    parameterized_test! { state, (program, expected), {
        let mut machine: Machine = program.parse().expect("Invalid");
        machine.run();
        let expected: Machine = expected.parse().expect("Invalid");
        assert_eq!(machine.state, expected.state);
    }}
    state! {
        d2_add_mul: ("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50"),
        d2_add: ("1,0,0,0,99", "2,0,0,0,99"),
        d2_mul: ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
        d2_add_mul_dynamic: ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        d5_immediate_mode: ("1002,4,3,4,33", "1002,4,3,4,99"),
        d5_negative_num: ("1101,100,-1,4,0", "1101,100,-1,4,99"),
    }

    parameterized_test!{ output, (program, expected), {
        let mut machine: Machine = program.parse().unwrap();
        machine.run();
        assert_eq!(machine.read_output(), expected);
    }}
    output!{
        d9_quine: ("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            vec!(109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99)),
        d9_large_add: ("1102,34915192,34915192,7,4,7,99,0", vec!(1219070632396864)),
        d9_large_value: ("104,1125899906842624,99", vec!(1125899906842624i64)),
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
        d5_identity: ("3,0,4,0,99", vec!(10), vec!(10)),
    }

    parameterized_test! { test_input, (program, true_input, false_input), {
        let mut machine: Machine = program.parse().expect("Invalid");
        machine.send_input(true_input);
        machine.run();
        assert_eq!(machine.read_output(), vec!(1));

        let mut machine: Machine = program.parse().expect("Invalid");
        machine.send_input(false_input);
        machine.run();
        assert_eq!(machine.read_output(), vec!(0));
    }}
    test_input! {
        d5_pos_eq: ("3,9,8,9,10,9,4,9,99,-1,8", 8, 12),
        d5_pos_lt: ("3,9,7,9,10,9,4,9,99,-1,8", 5, 11),
        d5_immed_eq: ("3,3,1108,-1,8,3,4,3,99", 8, 10),
        d5_immed_lt: ("3,3,1107,-1,8,3,4,3,99", 4, 9),
        d5_pos_jump: ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 5, 0),
        d5_immed_jump: ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 5, 0),
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