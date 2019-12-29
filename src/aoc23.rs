use crate::intcode::{Machine, State};
use std::collections::{HashMap, VecDeque, HashSet};

pub fn advent() {
    let image = Machine::from_file("data/day23.txt");
    let mut machines: Vec<_> = (0..50).map(|i| {
        let mut machine = image.clone();
        machine.send_input(i);
        machine
    }).collect();

    let mut queues: HashMap<usize, VecDeque<i64>> = (0..50).map(|i| (i, VecDeque::new())).collect();
    let mut seen_nat = false;
    let mut nat: Option<(i64, i64)> = None;
    let mut seen_idle_ys = HashSet::new();

    loop {
        let mut idle = true;
        for (i, machine) in machines.iter_mut().enumerate() {
            match machine.run_until(|o| o.len() >= 3) {
                State::OUTPUT => {
                    idle = false;
                    let packet = machine.read_output();
                    assert_eq!(packet.len(), 3);
                    if packet[0] == 255 {
                        if !seen_nat {
                            println!("First NAT Packet Y:   {}", &packet[2]);
                            seen_nat = true;
                        }
                        nat = Some((packet[1], packet[2]));

                    } else {
                        let queue = queues.get_mut(&(packet[0] as usize)).expect("No queue");
                        queue.push_back(packet[1]);
                        queue.push_back(packet[2]);
                    }
                },
                State::INPUT => {
                    let queue = queues.get_mut(&i).expect("No queue");
                    match queue.pop_front() {
                        Some(x) => {
                            idle = false;
                            machine.send_input(x);
                            machine.send_input(queue.pop_front().expect("No second part?"));
                        },
                        None => {
                            machine.send_input(-1);
                        }
                    }
                },
                _ => panic!(),
            }
        }

        if idle {
            if let Some((x, y))= nat.take() {
                if !seen_idle_ys.insert(y) {
                    println!("First repeated NAT Y: {}", y);
                    break;
                }
                machines[0].send_input(x);
                machines[0].send_input(y);
            }
        }
    }

}