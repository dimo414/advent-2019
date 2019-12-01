use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn advent() {
    let data = read_data();
    let base_fuel: u32 = data.iter().map(|&m| base_fuel_required(m)).sum();
    println!("Base Fuel Required: {}", base_fuel);
    let full_fuel: u32 = data.iter().map(|&m| full_fuel_required(m)).sum();
    println!("Full Fuel Required: {}", full_fuel);
}

fn read_data() -> Vec<u32> {
    let reader = BufReader::new(File::open("data/day1.txt").expect("Cannot open"));

    return reader.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect();
}

fn base_fuel_required(mass: u32) -> u32 {
    u32::saturating_sub(mass / 3, 2)
}

fn full_fuel_required(mass: u32) -> u32 {
    let mut unfueled_mass = mass;
    let mut total_fuel = 0;
    loop {
        let fuel = base_fuel_required(unfueled_mass);
        if fuel == 0 {
            return total_fuel;
        }
        unfueled_mass = fuel;
        total_fuel += fuel;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test!{ base_fuel, (mass, expected), {
            assert_eq!(base_fuel_required(mass), expected);
        }}
    base_fuel!{
            m12: (12, 2),
            m14: (14, 2),
            m1969: (1969, 654),
            m100756: (100756, 33583),
        }

    parameterized_test!{ full_fuel, (mass, expected), {
            assert_eq!(full_fuel_required(mass), expected);
        }}
    full_fuel! {
            m14: (14, 2),
            m1969: (1969, 966),
            m100756: (100756, 50346),
        }

    #[test]
    fn read_file() {
        assert!(read_data().len() > 0);
    }
}