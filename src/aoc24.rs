use std::collections::{HashSet, VecDeque};
use crate::euclid::{point, Point, vector};
use crate::error::ParseError;
use std::str::FromStr;

const INPUT: &str = "###.#\n\
                     ..#..\n\
                     #..#.\n\
                     #....\n\
                     .#.#.";

pub fn advent() {
    let mut bio: Biosphere = INPUT.parse().unwrap();
    bio.step_until();
    println!("Single-layer Bio Rating: {}", bio.rating());

    let mut rec_bio = RecBiosphere::new(&INPUT.parse().unwrap());
    for _ in 0..200 {
        rec_bio.step();
    }
    println!("Recursive bug count after 200 minutes: {}", rec_bio.count());
}

struct Biosphere {
    bugs: HashSet<Point>,
}

impl Biosphere {
    fn step(&mut self) {
        let past_iter: HashSet<_> = self.bugs.drain().collect();
        for y in 0..5 {
            for x in 0..5 {
                let coord = point(x, y);
                let neighbors = Biosphere::neighbors(&past_iter, &coord);
                let live = match past_iter.get(&coord) {
                    Some(_) => neighbors == 1,
                    None => neighbors == 1 || neighbors == 2,
                };
                if live {
                    self.bugs.insert(coord);
                }
            }
        }
    }

    fn step_until(&mut self) {
        let mut seen = HashSet::new();
        while seen.insert(self.rating()) {
            self.step();
        }
    }

    fn neighbors(points: &HashSet<Point>, point: &Point) -> u32 {
        [vector(-1, 0), vector(1, 0), vector(0, -1), vector(0, 1)].iter()
            .filter(|&v| { let p = *point+v; points.contains(&p) })
            .count() as u32
    }

    fn rating(&self) -> u64 {
        self.bugs.iter().map(|p| 2_u64.pow((p.y*5+p.x) as u32)).sum()
    }
}

impl FromStr for Biosphere {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut bugs = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = point(x as i32, y as i32);
                match c {
                    '#' => { bugs.insert(coord); },
                    '.'|'?' => {},
                    _ => panic!(),
                }
            }
        }

        Ok(Biosphere { bugs })
    }
}

struct RecBiosphere {
    layers: VecDeque<HashSet<Point>>,
}

impl RecBiosphere {
    fn new(biosphere: &Biosphere) -> RecBiosphere {
        let layer = biosphere.bugs.clone();
        let layers: VecDeque<_> = vec!(HashSet::new(), layer, HashSet::new()).into_iter().collect();
        RecBiosphere { layers }
    }

    fn step(&mut self) {
        let past_iter: Vec<_> = self.layers.drain(..).collect();
        self.layers.push_back(HashSet::new());
        for (i, past_layer) in past_iter.iter().enumerate() {
            let mut layer = HashSet::new();
            for y in 0..5 {
                for x in 0..5 {
                    let coord = point(x, y);
                    if coord == point(2,2) { continue; }
                    let neighbors = RecBiosphere::neighbors(&past_iter, i, &coord);
                    let live = match past_layer.get(&coord) {
                        Some(_) => neighbors == 1,
                        None => neighbors == 1 || neighbors == 2,
                    };
                    if live {
                        layer.insert(coord);
                    }
                }
            }
            self.layers.push_back(layer);
        }
        self.layers.push_back(HashSet::new());
    }

    fn neighbors(layers: &[HashSet<Point>], layer: usize, p: &Point) -> u32 {
        let p = *p;
        let same = [vector(-1, 0), vector(1, 0), vector(0, -1), vector(0, 1)].iter()
            .filter(|&v| { let n = p+v; layers[layer].contains(&n) })
            .count() as u32;

        // TODO these nested and outer calculations feel inelegant...

        let nested =
            if layer < layers.len()-1 {
                if p == point(1, 2) {
                    layers[layer + 1].iter().filter(|p| p.x == 0).count()
                } else if p == point(2, 1) {
                    layers[layer + 1].iter().filter(|p| p.y == 0).count()
                } else if p == point(3, 2) {
                    layers[layer + 1].iter().filter(|p| p.x == 4).count()
                } else if p == point(2, 3) {
                    layers[layer + 1].iter().filter(|p| p.y == 4).count()
                } else { 0 }
            } else { 0 };

        let outer =
        if layer > 0 {
            (if p.x == 0 {
                if layers[layer - 1].contains(&point(1, 2)) { 1 } else { 0 }
            } else if p.x == 4 {
                if layers[layer - 1].contains(&point(3, 2)) { 1 } else { 0 }
            } else { 0 }) +
                (if p.y == 0 {
                if layers[layer - 1].contains(&point(2, 1)) { 1 } else { 0 }
            } else if p.y == 4 {
                if layers[layer - 1].contains(&point(2, 3)) { 1 } else { 0 }
            } else { 0 })
        } else { 0 };

        same + nested as u32 + outer
    }

    fn count(&self) -> usize {
        self.layers.iter().map(|l| l.len()).sum()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#\n\
                      #..#.\n\
                      #.?##\n\
                      ..#..\n\
                      #....";

    const SAMPLE_4MIN: &str = "####.\n\
                               ....#\n\
                               ##..#\n\
                               .....\n\
                               ##...";

    #[test]
    fn part1() {
        let mut sample: Biosphere = SAMPLE.parse().unwrap();
        for _ in 0..4 {
            sample.step();
        }
        let sample_4min: Biosphere = SAMPLE_4MIN.parse().unwrap();
        assert_eq!(sample.bugs, sample_4min.bugs);

        sample.step_until();
        assert_eq!(sample.rating(), 2129920);
    }

    #[test]
    fn part2() {
        let mut rec_sample = RecBiosphere::new(&SAMPLE.parse().unwrap());
        for _ in 0..10 {
            rec_sample.step();
        }
        assert_eq!(rec_sample.count(), 99);
    }
}