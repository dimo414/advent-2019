use std::fs;

pub fn advent() {
    let input = read_data();
    println!("Alarm State Pos 0: {}", run_program(&input, 12, 2)[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            let output = run_program(&input, noun, verb)[0];
            if output == 19690720 {
                println!("Found NounVerb: {}", noun * 100 + verb);
                return;
            }
        }
    }
    println!("No match!");
}

fn read_data() -> Vec<usize> {
    parse_program(fs::read_to_string("data/day2.txt").expect("Cannot open").trim())
}

fn parse_program(input: &str) -> Vec<usize> {
    input.split(",").map(|n| n.parse::<usize>().unwrap()).collect()
}

fn run_program(input: &Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    let mut output = input.clone();
    output[1] = noun;
    output[2] = verb;
    let mut pos = 0;

    loop {
        let opcode = output[pos];
        match opcode {
            1 | 2 => {

                let a = output[output[pos+1]];
                let b = output[output[pos+2]];
                let c_pos = output[pos+3];
                output[c_pos] = match opcode {
                    1 => a + b,
                    2 => a * b,
                    _ => panic!("Impossible"),
                };
            },
            99 => break,
            _ => panic!("Unexpected opcode {} at pos {} of {:?}", output[pos], pos, output),
        }
        pos += 4;
        assert!(pos < output.len());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test!{ examples, (input, expected), {
            let input = parse_program(input);
            let expected = parse_program(expected);
            assert_eq!(run_program(&input, input[1], input[2]), expected);
        }}
    examples!{
            a: ("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50"),
            b: ("1,0,0,0,99", "2,0,0,0,99"),
            c: ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            d: ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        }

    #[test]
    fn read_file() {
        assert!(read_data().len() > 0);
    }
}
