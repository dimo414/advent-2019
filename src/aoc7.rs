use crate::intcode::{Machine, State};
use permutohedron::LexicalPermutation;

pub fn advent() {
    let image = read_data();
    let max = find_maximum_signal(&image);
    println!("         Sequence: {:?} - Signal: {:8}", max.0, max.1);
    let max = find_maximum_feedback_signal(&image);
    println!("Feedback Sequence: {:?} - Signal: {:8}", max.0, max.1);
}

fn read_data() -> Machine {
    Machine::from_file("data/day7.txt")
}

fn compute_signal(image: &Machine, sequence: &[i64]) -> i64 {
    let mut last_output = 0;
    for &phase in sequence {
        let mut machine: Machine = image.clone();
        machine.send_input(phase);
        machine.send_input(last_output);
        machine.run();
        let output = machine.read_output();
        assert_eq!(output.len(), 1);
        last_output = output[0];
    }
    last_output
}

fn compute_feedback_signal(image: &Machine, sequence: &[i64]) -> i64 {
    let mut machines: Vec<_> = (0..5).map(|_| image.clone()).collect();
    for i in 0..machines.len() {
        machines[i].send_input(sequence[i]);
    }

    let mut last_output = 0;
    for i in (0..machines.len()).cycle() {
        machines[i].send_input(last_output);
        let state = machines[i].run();
        let output = machines[i].read_output();
        if state == State::HALT && output.is_empty() {
            assert_eq!(i, 0); break;
        }
        assert_eq!(output.len(), 1);
        last_output = output[0];
    }
    last_output
}

fn find_maximum_signal(image: &Machine) -> (Vec<i64>, i64) {
    let mut sequence = [0, 1, 2, 3, 4];
    let mut max = (sequence.to_vec(), 0);
    loop {
        let signal = compute_signal(image, &sequence);
        if signal > max.1 {
            max = (sequence.to_vec(), signal);
        }
        if !sequence.next_permutation() {
            break;
        }
    }
    max
}

fn find_maximum_feedback_signal(image: &Machine) -> (Vec<i64>, i64) {
    let mut sequence = [5, 6, 7, 8, 9];
    let mut max = (sequence.to_vec(), 0);
    loop {
        let signal = compute_feedback_signal(image, &sequence);
        if signal > max.1 {
            max = (sequence.to_vec(), signal);
        }
        if !sequence.next_permutation() {
            break;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{ signal, (program, max_sequence, max_signal), {
        let image: Machine = program.parse().unwrap();
        assert_eq!(compute_signal(&image, &max_sequence), max_signal);
        assert_eq!(find_maximum_signal(&image), (max_sequence, max_signal));
    }}
    signal!{
        a: ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", vec!(4,3,2,1,0), 43210),
        b: ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", vec!(0,1,2,3,4), 54321),
        c: ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", vec!(1,0,4,3,2), 65210),
    }

    parameterized_test::create!{ feedback_signal, (program, max_sequence, max_signal), {
        let image: Machine = program.parse().unwrap();
        assert_eq!(compute_feedback_signal(&image, &max_sequence), max_signal);
        assert_eq!(find_maximum_feedback_signal(&image), (max_sequence, max_signal));
    }}
    feedback_signal!{
        a: ("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", vec!(9,8,7,6,5), 139629729),
        b: ("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", vec!(9,7,8,5,6), 18216),
    }

    #[test]
    fn read_file() {
        read_data();
    }
}
