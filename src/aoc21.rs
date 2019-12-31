use crate::intcode::{Machine, State};

pub fn advent() {
    let image = read_data();

    let walk_program = vec!(
        "NOT A T", // T if A is a hole
        "NOT B J", // J if B is a hole
        "OR J T",  // T if A or B is a hole
        "NOT C J", // J if C is a hole
        "OR T J", // J if A, B, or C is a hole
        "AND D J", // J if D is not a hole, and A, B, or C are
    );
    let walk_damage = spring(&image, &[&walk_program[..], &["WALK"]].concat()).expect("WALK failed");
    println!("WALK hull damage: {}", walk_damage);

    let run_program = vec!(
        // if and only-if E and H are holes, don't jump
        "NOT H T", // T if H(8) is a hole
        "NOT T T", // T if H(8) is not a hole
        "OR E T", // T if H(8) is not a hole or E(5) is not a hole
        "AND T J", // J if D is jumpable (per walk_program) and doesn't trap us
    );
    let run_damage = spring(&image, &[&walk_program[..], &run_program[..], &["RUN"]].concat()).expect("RUN failed");
    println!("RUN hull damage:  {}", run_damage);
}

fn read_data() -> Machine {
    Machine::from_file("data/day21.txt")
}

fn spring(image: &Machine, program: &[&str]) -> Option<i64> {
    let mut machine = image.clone();

    machine.run().assert_input();
    assert_eq!(machine.read_output_ascii(), "Input instructions:\n");
    machine.send_input_ascii(&program.join("\n"));
    machine.send_input_ascii("\n");
    let n = '\n' as i64;
    machine.run_until(|o| o.len() > 2 && o[o.len()-2..] == [n, n]).assert_output();
    assert_eq!(&machine.read_output_ascii()[5..], "ing...\n\n"); // "[\nWalk]ing" or "[\nRunn]ing"
    match machine.run_until(|o| o.len() > 1) {
        State::HALT => Some(machine.read_output()[0]),
        State::OUTPUT => {
            machine.run();
            println!("{}", machine.read_output_ascii());
            None
        },
        _ => panic!(),
    }
}

// TODO (change detector) tests?