#![ allow( dead_code ) ]
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Opcode {
    ADD,
    MUL,
    EXIT,
}

impl Opcode {
    fn opcode(&self) -> usize {
        use Opcode::*;
        match *self {
            ADD => 1,
            MUL => 2,
            EXIT => 99,
        }
    }

    fn lookup(code: usize) -> Option<Opcode> {
        use Opcode::*;
        match code {
            1 => Some(ADD),
            2 => Some(MUL),
            99 => Some(EXIT),
            _ => None,
        }
    }

    fn parameters(&self) -> usize {
        use Opcode::*;
        match *self {
            EXIT => 0,
            ADD | MUL => 3,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({})", self, self.opcode())
    }
}

pub struct Machine {
    state: Vec<usize>,
    pointer: usize,
}

impl Machine {
    pub fn new(state: &[usize]) -> Machine {
        Machine { state: state.to_vec(), pointer: 0 }
    }

    pub fn read_state(&self, address: usize) -> usize {
        self.state[address]
    }

    pub fn set_state(&mut self, address: usize, value: usize) {
        self.state[address] = value;
    }

    fn set_pointer(&mut self, pointer: usize) {
        self.pointer = pointer;
    }

    pub fn run(&mut self) {
        self.debug();
    }

    pub fn debug(&mut self) {
        loop {
            let opcode = Opcode::lookup(self.state[self.pointer])
                .expect(&format!("Invalid opcode {} at {}", self.state[self.pointer], self.pointer));
            let params = self.state[self.pointer+1..self.pointer+1+opcode.parameters()].to_vec();
            match opcode {
                Opcode::ADD => self.add(&params),
                Opcode::MUL => self.mul(&params),
                Opcode::EXIT => return,
            }
            self.pointer += 1 + opcode.parameters();
        }
    }

    fn add(&mut self, params: &[usize]) {
        let a = self.state[params[0]];
        let b = self.state[params[1]];
        self.state[params[2]] = a + b;
    }

    fn mul(&mut self, params: &[usize]) {
        let a = self.state[params[0]];
        let b = self.state[params[1]];
        self.state[params[2]] = a * b;
    }
}

impl FromStr for Machine {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(",").map(|n| n.parse::<usize>()).collect::<Result<Vec<usize>, _>>()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test! { add_mul, (input, expected), {
        let mut machine: Machine = input.parse().expect("Invalid");
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