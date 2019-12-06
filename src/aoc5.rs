use std::fs;
use crate::intcode::Machine;

pub fn advent() {
    let mut machine = read_data();
    machine.send_input(1);
    machine.run();
    let diagnostic = machine.read_output();
    println!("AC Diagnostic: {}", diagnostic[diagnostic.len() - 1]);

    let mut machine = read_data();
    machine.send_input(5);
    machine.run();
    println!("Radiator Diagnostic: {}", machine.read_output()[0]);
}

fn read_data() -> Machine {
    fs::read_to_string("data/day5.txt").expect("Cannot open").trim().parse().expect("Invalid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        read_data();
    }
}
