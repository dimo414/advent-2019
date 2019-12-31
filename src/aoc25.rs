use crate::intcode::{Machine, State};
use std::io::{stdin, stdout, Write};

pub fn advent() {
    let image = read_data();
    if interactive!() {
        let mut input = String::new();
        print!("Explore manually? [y/N]: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).expect("Failed");
        if input.trim().to_ascii_lowercase() == "y" {
            interactive(&image);
            return;
        }
    }
    let santas_greeting = pre_explored(&image);
    println!("Santa says: {}", santas_greeting);
}

fn read_data() -> Machine {
    Machine::from_file("data/day25.txt")
}

fn pre_explored(image: &Machine) -> String {
    // Q       H-G
    // |         |
    // P-O-A-B-C |
    //   | |   | |
    //   | N   | |
    //   |   I-D-E
    //   R-T |   |
    //   |   |   F
    //   S K-J
    //     |
    //     L
    //     |
    //     M
    //
    // A: Hull Breach
    // B: Engineering - mug
    // C: Sick Bay - escape pod(!)
    // D: Crew Quarters
    // E: Hot Chocolate Fountain - photons(!)
    // F: Corridor - easter egg
    // G: Stables - molten lava(?)
    // H: Kitchen
    // I: Warp Drive Maintenance
    // J: Observatory - infinite loop(?)
    // K: Gift Wrapping Center - shell
    // L: Security Checkpoint
    // M: Pressure-Sensitive Floor = Requires: easter egg(F), mug(B), sand(S), space heater(Q)
    // N: Science Lab - weather machine
    // O: Hallway
    // P: Holodeck - giant electromagnet(!)
    // Q: Passages - space heater
    // R: Navigation - festive hat
    // S: Storage - sand
    // T: Arcade - whirled peas

    let mut machine = image.clone();
    let instructions = vec!(
        "west", "west", "north", "take space heater",
        "south", "east", "south", "south", "take sand",
        "north", "north", "east", "east", "take mug",
        "east", "south", "east", "south", "take easter egg",
        "north", "west", "west", "south", "west", "south", "south",
    );
    machine.send_input_ascii(&instructions.join("\n"));
    machine.send_input_ascii("\n");
    machine.run().assert_halt();
    let output = machine.read_output_ascii();
    let output: Vec<_> = output.lines().collect();
    output[output.len()-1].into()
}

fn interactive(image: &Machine) {
    let mut machine = image.clone();
    loop {
        let state = machine.run();
        println!("\n{}", machine.read_output_ascii());

        match state {
            State::HALT => { break; },
            State::INPUT => {},
            _ => panic!(),
        }

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed");
        machine.send_input_ascii(&input);
    }
}