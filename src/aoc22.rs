use std::str::FromStr;
use regex::Regex;
use crate::error::ParseError;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SMALL_SIZE: i64 = 10007;
const LARGE_SIZE: i64 = 119315717514047;
const LARGE_REPEATS: i64 = 101741582076661;

pub fn advent() {
    let mut deck: Vec<_> = (0..SMALL_SIZE as usize).collect();
    let moves = read_data();
    for m in moves.iter() {
        deck = m.apply(deck);
    }
    let pos_2019 = deck.iter().position(|&v| v == 2019).unwrap();
    println!("Position of card 2019: {}", pos_2019);

    // Applying the same approach isn't feasible for a deck, or even a single index, of the larger size
    // Need to utilize modular arithmetic ¯\_(ツ)_/¯
    //
    // See https://old.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/
    // And https://old.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbpz92k/
    let mut repr = DeckRepr::new(SMALL_SIZE);
    for m in moves.iter() {
        m.apply_repr(&mut repr);
    }
    assert_eq!(repr.card_at(pos_2019 as u64), 2019);

    let mut repr = DeckRepr::new(LARGE_SIZE);
    for m in moves.iter() {
        m.apply_repr(&mut repr);
    }
    repr.repeat(LARGE_REPEATS);
    println!("Card at 2020: {}", repr.card_at(2020));
}

fn read_data() -> Vec<Move> {
    let reader = BufReader::new(File::open("data/day22.txt").expect("Cannot open"));

    return reader.lines().map(|l| l.unwrap().parse().unwrap()).collect();
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Move {
    REVERSE,
    CUT(isize),
    DEAL(usize),
}

impl Move {
    fn apply(&self, mut deck: Vec<usize>) -> Vec<usize> {
        match self {
            Move::REVERSE => deck.reverse(),
            Move::CUT(cut) => {
                let n = deck.len() as isize;
                let cut = (((cut % n) + n) % n) as usize;
                let mut new_deck = vec![0; deck.len()];
                new_deck[deck.len()-cut..].clone_from_slice(&deck[..cut]);
                new_deck[..deck.len()-cut].clone_from_slice(&deck[cut..]);
                return new_deck;
            },
            Move::DEAL(deal) => {
                let mut new_deck = vec![0; deck.len()];
                let mut pos = 0;
                for n in deck {
                    assert_eq!(new_deck[pos], 0); // Collision!
                    new_deck[pos] = n;
                    pos = (pos + *deal) % new_deck.len();
                }
                return new_deck;
            },
        }

        deck
    }

    #[cfg(test)]
    fn undo(&self, index: u64, len: u64) -> u64 {
        match self {
            Move::REVERSE => len - index - 1,
            Move::CUT(cut) => {
                let cut = (((*cut as i64 % len as i64) + len as i64) % len as i64) as u64;
                (index + cut) % len
            },
            Move::DEAL(deal) => {
                // Forward is (index*deal)%len
                // Not sure how best to go backwards, but we can find the cycle
                let mut last_index = index;
                loop {
                    let next_index = last_index * *deal as u64 % len;
                    if next_index == index { return last_index; }
                    last_index = next_index;
                }
            },
        }
    }

    // https://old.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju/
    fn apply_repr(&self, repr: &mut DeckRepr) {
        match self {
            Move::REVERSE => {
                repr.increment = repr.mod_mul(repr.increment, -1);
                repr.offset = repr.mod_add(repr.offset, repr.increment);
            },
            Move::CUT(c) => {
                repr.offset = repr.mod_add(repr.offset, repr.mod_mul(repr.increment, *c as i64));
            },
            Move::DEAL(d) => {
                if repr.size == SMALL_SIZE || repr.size == LARGE_SIZE { // known to be prime
                    repr.increment = repr.mod_mul(repr.increment, repr.mod_exp(*d as i64, repr.size - 2));
                } else {
                    repr.increment = repr.mod_mul(repr.increment, modular_inverse(*d as u64, repr.size as u64) as i64);
                }
            }
        }
    }
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
fn modular_inverse(a: u64, n: u64) -> u64 {
    let a = a as i64;
    let n = n as i64;
    let (mut t, mut new_t, mut r, mut new_r) = (0, 1, n, a);
    while new_r != 0 {
        let q = r / new_r;
        let next_t = new_t;
        new_t = t - q * new_t;
        t = next_t;

        let next_r = new_r;
        new_r = r - q * new_r;
        r = next_r;
    }
    if r > 1 { panic!(format!("{} is not invertible", a)); }
    ((t + n) % n) as u64
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        lazy_static! {
                static ref RE: Regex =
                Regex::new(r"deal into new stack|cut (.+)|deal with increment (.+)").unwrap();
            }

        let caps: regex::Captures = regex_captures!(RE, s)?;
        if let Some(cut) = caps.get(1) {
            return Ok(Move::CUT(cut.as_str().parse()?));
        }
        if let Some(deal) = caps.get(2) {
            return Ok(Move::DEAL(deal.as_str().parse()?));
        }
        Ok(Move::REVERSE)
    }
}

#[derive(Copy, Clone, Debug)]
struct DeckRepr {
    offset: i64,
    increment: i64,
    size: i64,
}

impl DeckRepr {
    fn new(size: i64) -> DeckRepr { DeckRepr { offset: 0, increment: 1, size } }

    fn mod_add(&self, a: i64, b: i64) -> i64 {
        let m = self.size;
        (((a + b) % m) + m) % m
    }

    fn mod_mul(&self, a: i64, b: i64) -> i64 {
        let a = a as i128;
        let b = b as i128;
        let m = self.size as i128;
        ((((a * b) % m) + m) % m) as i64
    }

    fn mod_exp(&self, a: i64, b: i64) -> i64 {
        mod_exp::mod_exp(a as i128, b as i128, self.size as i128) as i64
    }


    fn repeat(&mut self, times: i64) {
        assert!(self.size == SMALL_SIZE || self.size == LARGE_SIZE); // only works for known primes
        let new_increment = self.mod_exp(self.increment, times);
        //offset = offset_diff * (1 - increment) * pow((1 - increment_mul) % size, size - 2, size)
        self.offset = self.mod_mul(self.offset,
                                   self.mod_mul(self.mod_add(1, 0-new_increment),
                                                self.mod_exp(self.mod_add(1, 0-self.increment), self.size - 2)));
        self.increment = new_increment;

    }

    fn card_at(&self, pos: u64) -> u64 {
        self.mod_add(self.offset, self.mod_mul(self.increment, pos as i64)) as u64
    }

    #[cfg(test)]
    fn apply_deck(&self, deck: &Vec<usize>) -> Vec<usize> {
        (0..deck.len()).map(|i| deck[self.card_at(i as u64) as usize]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{ mod_inv, (a, n, inv), {
        assert_eq!(modular_inverse(a, n), inv);
    }}
    mod_inv! {
        a: (3, 10, 7),
    }

    parameterized_test::create!{ parse, (input, expected), {
        let m: Move = input.parse().unwrap();
        assert_eq!(m, expected);
    }}
    parse! {
        reverse: ("deal into new stack", Move::REVERSE),
        cut: ("cut -2", Move::CUT(-2)),
        deal: ("deal with increment 7", Move::DEAL(7)),
    }

    parameterized_test::create!{ shuffle, (moves, expected, exp_repr), {
        let mut deck: Vec<_> = (0..10).collect();
        for m in moves.iter() {
            deck = m.apply(deck);
        }
        assert_eq!(deck, expected);

        // now verify undo()
        let len = expected.len() as u64;
        for i in 0..expected.len() {
            let mut index = i as u64;
            for m in moves.iter().rev() {
                index = m.undo(index, len);
            }
            assert_eq!(index, expected[i] as u64);
        }

        // now verify the DeckRepr behavior, which isn't documented by the problem
        let mut repr = DeckRepr::new(expected.len() as i64);
        for m in moves.iter() {
            m.apply_repr(&mut repr);
        }

        let repr_deck = repr.apply_deck(&(0..10).collect());
        assert_eq!(repr_deck, expected);
        assert_eq!((repr.offset, repr.increment), exp_repr);
    }}
    shuffle! {
        reverse: (vec!(Move::REVERSE), vec!(9,8,7,6,5,4,3,2,1,0), (9, 9)),
        cut_3: (vec!(Move::CUT(3)), vec!(3,4,5,6,7,8,9,0,1,2), (3, 1)),
        cut_n4: (vec!(Move::CUT(-4)), vec!(6,7,8,9,0,1,2,3,4,5), (6, 1)),
        deal_3: (vec!(Move::DEAL(3)), vec!(0,7,4,1,8,5,2,9,6,3), (0, 7)),
        a: (vec!(Move::DEAL(7), Move::REVERSE, Move::REVERSE), vec!(0,3,6,9,2,5,8,1,4,7), (0, 3)),
        b: (vec!(Move::CUT(6), Move::DEAL(7), Move::REVERSE), vec!(3,0,7,4,1,8,5,2,9,6), (3, 7)),
        c: (vec!(Move::DEAL(7), Move::DEAL(9), Move::CUT(-2)), vec!(6,3,0,7,4,1,8,5,2,9), (6, 7)),
        d: (vec!(
                Move::REVERSE, Move::CUT(-2), Move::DEAL(7), Move::CUT(8), Move::CUT(-4),
                Move::DEAL(7), Move::CUT(3), Move::DEAL(9), Move::DEAL(3), Move::CUT(-1)),
            vec!(9,2,5,8,1,4,7,0,3,6), (9, 3)),
    }
}