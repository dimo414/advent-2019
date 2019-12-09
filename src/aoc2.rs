use std::fs;
use crate::intcode::Machine;

pub fn advent() {
    let input = read_data();
    println!("Alarm State Pos 0: {}", run_program(&input, 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            let output = run_program(&input, noun, verb);
            if output == 19690720 {
                println!("Found NounVerb: {}", noun * 100 + verb);
                return;
            }
        }
    }
    println!("No match!");
}

fn read_data() -> String {
    fs::read_to_string("data/day2.txt").expect("Cannot open").trim().into()
}

fn run_program(input: &str, noun: i64, verb: i64) -> i64 {
    let mut machine: Machine = input.parse().expect("Invalid");
    machine.set_state(1, noun);
    machine.set_state(2, verb);
    machine.run();
    machine.read_state(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_state() {
        assert_eq!(run_program("1,0,0,3,2,3,11,0,99,30,40,50", 9, 10), 3500);
    }

    #[test]
    fn read_file() {
        read_data();
    }
}
