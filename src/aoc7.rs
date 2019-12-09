use std::fs;
use crate::intcode::{Debugger, Machine, Opcode, Address};
use permutohedron::LexicalPermutation;

pub fn advent() {
    let program = read_data();
    let max = find_maximum_signal(&program);
    println!("         Sequence: {:?} - Signal: {:8}", max.0, max.1);
    let max = find_maximum_feedback_signal(&program);
    println!("Feedback Sequence: {:?} - Signal: {:8}", max.0, max.1);
}

fn read_data() -> String {
    fs::read_to_string("data/day7.txt").expect("Cannot open").trim().into()
}

fn compute_signal(program: &str, sequence: &[i64]) -> i64 {
    let mut last_output = 0;
    for &phase in sequence {
        let mut machine: Machine = program.parse().expect("Invalid");
        machine.send_input(phase);
        machine.send_input(last_output);
        machine.run();
        let output = machine.read_output();
        assert_eq!(output.len(), 1);
        last_output = output[0];
    }
    last_output
}

fn compute_feedback_signal(program: &str, sequence: &[i64]) -> i64 {
    let mut machines: Vec<Machine> = vec!(
        program.parse().unwrap(),
        program.parse().unwrap(),
        program.parse().unwrap(),
        program.parse().unwrap(),
        program.parse().unwrap()
    );
    for i in {0..machines.len()} {
        machines[i].send_input(sequence[i]);
    }

    let mut last_output = 0;
    for i in (0..machines.len()).cycle() {
        machines[i].send_input(last_output);
        machines[i].debug(&mut BreakOnOutput::new());
        let output = machines[i].read_output();
        assert!(output.len() < 2);
        if output.is_empty() { assert_eq!(i, 0); break; }
        last_output = output[0];
    }
    last_output
}

fn find_maximum_signal(program: &str) -> (Vec<i64>, i64) {
    let mut sequence = [0, 1, 2, 3, 4];
    let mut max = (sequence.to_vec(), 0);
    loop {
        let signal = compute_signal(program, &sequence);
        if signal > max.1 {
            max = (sequence.to_vec(), signal);
        }
        if !sequence.next_permutation() {
            break;
        }
    }
    max
}

fn find_maximum_feedback_signal(program: &str) -> (Vec<i64>, i64) {
    let mut sequence = [5, 6, 7, 8, 9];
    let mut max = (sequence.to_vec(), 0);
    loop {
        let signal = compute_feedback_signal(program, &sequence);
        if signal > max.1 {
            max = (sequence.to_vec(), signal);
        }
        if !sequence.next_permutation() {
            break;
        }
    }
    max
}

pub struct BreakOnOutput {
    seen_output: bool,
}

#[allow(dead_code)]
impl BreakOnOutput {
    pub fn new() -> BreakOnOutput {
        BreakOnOutput{ seen_output: false }
    }
}

impl Debugger for BreakOnOutput {
    fn on_exec(&mut self, opcode: Opcode, _: &[Address], _: &[i64], _: usize, _: isize) -> bool {
        if self.seen_output && opcode != Opcode::EXIT {
            self.seen_output = false;
            return false;
        }
        if opcode == Opcode::OUTPUT {
            self.seen_output = true;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test!{ signal, (program, max_sequence, max_signal), {
        assert_eq!(compute_signal(program, &max_sequence), max_signal);
        assert_eq!(find_maximum_signal(program), (max_sequence, max_signal));
    }}
    signal!{
        a: ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", vec!(4,3,2,1,0), 43210),
        b: ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", vec!(0,1,2,3,4), 54321),
        c: ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", vec!(1,0,4,3,2), 65210),
    }

    parameterized_test!{ feedback_signal, (program, max_sequence, max_signal), {
        assert_eq!(compute_feedback_signal(program, &max_sequence), max_signal);
        assert_eq!(find_maximum_feedback_signal(program), (max_sequence, max_signal));
    }}
    feedback_signal!{
        a: ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", vec!(9,8,7,6,5), 139629729),
        b: ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", vec!(9,7,8,5,6), 18216),
    }

    #[test]
    fn read_file() {
        read_data().parse::<Machine>().unwrap();
    }
}