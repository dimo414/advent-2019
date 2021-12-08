use std::collections::HashMap;
use std::ops::{AddAssign, Mul};
use std::str::FromStr;
use crate::error::ParseError;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

//
// # Command to rename all the intermediate products to (maybe) make it easier to read
// $ grep -o '[A-Z]*' data/day14.txt | sort -u | grep -v -e FUEL -e ORE \
//   | ( letters=({A..Z}{A..Z}); n=0
//       while read line; do printf 's/%s/%s/g\n' "$line" "${letters[$n]}"; ((n++)); done; ) \
//   | sed --file=- data/day14.txt > /tmp/day14-simplified.txt
//

pub fn advent() {
    let fuel = Ingredient { id: Ingredient::FUEL, amount: 1 };
    let recipes = read_data("data/day14.txt");

    println!("1 FUEL: {} ORE", resolve(&recipes, &mut HashMap::new(), &fuel).amount);

    let fuel_created = maximize(&recipes, &Ingredient { id: Ingredient::ORE, amount: 1000000000000u64 }).amount;
    println!("10^12 ORE: {} FUEL", fuel_created);
}

fn read_data(file: &str) -> HashMap<u32, Recipe> {
    let reader = BufReader::new(File::open(file).expect("Cannot open"));
    reader.lines()
        .map(|l| l.unwrap().parse::<Recipe>().unwrap())
        .map(|r| (r.output.id, r))
        .collect()
}

fn resolve(recipes: &HashMap<u32, Recipe>, extras: &mut HashMap<u32, u64>, result: &Ingredient) -> Ingredient {
    if result.id == Ingredient::ORE {
        return *result;
    }

    let recipe = recipes.get(&result.id).expect("Missing");
    let mut ores = Ingredient { id: Ingredient::ORE, amount: 0 };
    let mut amount_created = extras.remove(&result.id).unwrap_or(0);
    if amount_created < result.amount {
        let batches = div_ceil(result.amount-amount_created, recipe.output.amount);
        while amount_created < result.amount {
            for input in recipe.inputs.iter() {
                ores += resolve(recipes, extras, &(*input * batches));
            }
            amount_created += recipe.output.amount * batches;
        }
    }
    if amount_created > result.amount {
        extras.insert(result.id, amount_created - result.amount);
    }
    ores
}

fn maximize(recipes: &HashMap<u32, Recipe>, max_input: &Ingredient) -> Ingredient {
    assert_eq!(max_input.id, Ingredient::ORE);

    let mut ore_left = max_input.amount as i64;
    let mut batch_size = 100000;
    let mut fuel_created = 0;
    let mut extras = HashMap::new();
    loop {
        // TODO this is wrong! we mutate extras even if we throw out the computation
        // can increase the batch size once this is fixed
        let ore = resolve(recipes, &mut extras, &Ingredient { id: Ingredient::FUEL, amount: batch_size }).amount as i64;
        if ore > ore_left {
            if batch_size > 1 {
                batch_size /= 10;
                continue;
            } else {
                break;
            }
        }
        ore_left -= ore;
        fuel_created += batch_size;
    }

    Ingredient { id: Ingredient::FUEL, amount: fuel_created }
}

// https://users.rust-lang.org/t/ceiling-of-isize-isize/23285/4
fn div_ceil(n: u64, d: u64) -> u64 {
    n/d + if n % d != 0 { 1 } else { 0 }
}

#[derive(Clone, Copy, Debug)]
struct Ingredient {
    //name: String,
    id: u32,
    amount: u64,
}

impl Ingredient {
    // Kind of weird way to get Copy semantics for Ingredient; map the name to an integer
    // This is a _huge_ performance savings vs. dealing with string copying over and over
    fn encode_name(s: &str) -> u32 {
        s.char_indices()
            .map(|(i, c)| c.to_digit(36).unwrap() * 10u32.pow((i*2) as u32))
            .sum()
    }
    const ORE: u32 = 142724;//encode_name("ORE");
    const FUEL: u32 = 21143015;//encode_name("FUEL");
}

impl AddAssign for Ingredient {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.id, other.id);
        *self = Self { id: self.id, amount: self.amount + other.amount };
    }
}

impl Mul<u64> for Ingredient {
    type Output = Ingredient;

    fn mul(self, m: u64) -> Ingredient {
        Ingredient { id: self.id, amount: self.amount * m }
    }
}

impl FromStr for Ingredient {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        lazy_static! {
                static ref RE: Regex = Regex::new(r"^(\d+) (\w+)$").unwrap();
            }

        let caps = regex_captures!(RE, s)?;
        let amount: u64 = capture_group!(caps, 1).trim().parse()?;
        let id = Ingredient::encode_name(capture_group!(caps, 2));
        Ok(Ingredient { id, amount })
    }
}

#[derive(Debug)]
struct Recipe {
    output: Ingredient,
    inputs: Vec<Ingredient>,
}

impl FromStr for Recipe {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let halves: Vec<_> = s.split(" => ").collect();
        assert_eq!(halves.len(), 2);
        let output: Ingredient = halves[1].parse()?;
        let inputs: Result<Vec<Ingredient>, _> = halves[0].split(", ").map(|i| i.parse()).collect();
        let inputs = inputs?;
        Ok(Recipe { output, inputs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create! { encode_name, (name, expected), {
        assert_eq!(Ingredient::encode_name(name), expected);
    }}
    encode_name! {
        ore: ("ORE", Ingredient::ORE),
        fuel: ("FUEL", Ingredient::FUEL),
        abcd: ("ABCD", 13121110),
    }

    parameterized_test::create! { process_recipes, (file, ore_expected, fuel_expected), {
        let recipes = read_data(&format!("data/day14-example{}.txt", file));
        let fuel = Ingredient { id: Ingredient::FUEL, amount: 1 };
        let ore = Ingredient { id: Ingredient::ORE, amount: 1000000000000u64 };

        assert_eq!(resolve(&recipes, &mut HashMap::new(), &fuel).amount, ore_expected);
        assert_eq!(maximize(&recipes, &ore).amount, fuel_expected);
    }}
    process_recipes! {
        a: (1, 31, 34482758620), // fuel amount is not given
        b: (2, 165, 6323777403), // fuel amount is not given
        c: (3, 13312, 82892753),
        d: (4, 180697, 5586022),
        e: (5, 2210736, 460664),
    }
}