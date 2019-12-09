use std::fs;
use crate::intcode::Machine;

pub fn advent() {
    let mut machine: Machine = read_data();
    machine.send_input(1);
    machine.run();
    println!("Keycode: {}", machine.read_output()[0]);

    let mut machine: Machine = read_data();
    machine.send_input(2);
    machine.run();
    println!("Coordinates: {}", machine.read_output()[0]);
}

fn read_data() -> Machine {
    fs::read_to_string("data/day9.txt").expect("Cannot open").trim().parse().expect("Invalid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        read_data();
    }
}