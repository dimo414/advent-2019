use crate::intcode::Machine;

pub fn advent() {
    let image = read_data();
    println!("Alarm State Pos 0: {}", run_program(image.clone(), 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            let output = run_program(image.clone(), noun, verb);
            if output == 19690720 {
                println!("Found NounVerb: {}", noun * 100 + verb);
                return;
            }
        }
    }
    println!("No match!");
}

fn read_data() -> Machine {
    Machine::from_file("data/day2.txt")
}

fn run_program(mut machine: Machine, noun: i64, verb: i64) -> i64 {
    machine.set_state(1, noun);
    machine.set_state(2, verb);
    machine.run();
    machine.read_state(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let machine = "1,0,0,3,2,3,11,0,99,30,40,50".parse().unwrap();
        assert_eq!(run_program(machine, 9, 10), 3500);
    }

    #[test]
    fn read_file() {
        read_data();
    }
}
