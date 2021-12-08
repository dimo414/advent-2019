use crate::euclid3d::{point, Point, vector, Vector};
use std::fmt;
use std::collections::HashSet;
use num_integer::Integer;

const INPUT: [Point; 4] = [
    point(-10, -10,-13),
    point(5, 5, -9),
    point(3, 8, -16),
    point(1, 3, -3),
];

pub fn advent() {
    let (xs, ys, zs) = cycles(&INPUT);
    let energy = state_at(&xs, &ys, &zs, 1000).iter().map(|m|m.energy()).sum::<u32>();
    println!("After 1k steps, energy: {}", energy);
    // https://www.wolframalpha.com/input/?i=lcm+167624+231614+102356
    println!("Cycles after: {} steps", cycle_len(xs, ys, zs));
}

type Cycle = Vec<Vec<(i32, i32)>>;
type CycleSlice = [Vec<(i32, i32)>];

fn cycle_len(xs: Cycle, ys: Cycle, zs: Cycle) -> usize {
    xs.len().lcm(&ys.len()).lcm(&zs.len())
}

fn cycles(coords: &[Point]) -> (Cycle, Cycle, Cycle) {
    (cycle1d(&coords.iter().map(|&p|p.x).collect::<Vec<_>>()),
     cycle1d(&coords.iter().map(|&p|p.y).collect::<Vec<_>>()),
     cycle1d(&coords.iter().map(|&p|p.z).collect::<Vec<_>>()))
}

fn cycle1d(coords: &[i32]) -> Vec<Vec<(i32, i32)>> {
    let mut coords: Vec<_> = coords.iter().map(|&c| (c, 0)).collect();
    let mut seen = HashSet::new();
    let mut steps = Vec::new();

    loop {
        if !seen.insert(coords.clone()) { break; }
        steps.push(coords.clone());
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let ig = coords[i].0;
                let jg = coords[j].0;
                // v.x + (g.x - p.x).signum()
                coords[i].1 += (jg - ig).signum();
                coords[j].1 += (ig - jg).signum();
            }
        }
        for coord in &mut coords {
            let v = coord.1;
            coord.0 += v;
        }
    }

    steps
}

fn state_at(xs: &CycleSlice, ys: &CycleSlice, zs: &CycleSlice, step: usize) -> Vec<Moon> {
    let xs = xs[step % xs.len()].clone();
    let ys = ys[step % ys.len()].clone();
    let zs = zs[step % zs.len()].clone();

    assert_eq!(xs.len(), ys.len());
    assert_eq!(xs.len(), zs.len());

    let mut moons = Vec::new();
    for i in 0..xs.len() {
        let moon = Moon::new(point(xs[i].0, ys[i].0, zs[i].0), vector(xs[i].1, ys[i].1, zs[i].1));
        moons.push(moon);
    }
    moons
}

#[cfg(test)]
fn simulate(coords: &[Point], steps: usize) -> u32 {
    let mut moons: Vec<_> = coords.iter().map(|&p| Moon::new(p, vector(0,0,0))).collect();
    for _ in 0..steps {
        apply_gravity(&mut moons);
        for m in moons.iter_mut() {
            m.movement();
        }
    }
    moons.iter().map(|m|m.energy()).sum()
}

#[cfg(test)]
fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i+1..moons.len() {
            let ig = moons[i].pos;
            let jg = moons[j].pos;
            moons[i].gravity(jg);
            moons[j].gravity(ig);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Moon {
    pos: Point,
    velocity: Vector,
}

impl Moon {
    fn new(pos: Point, velocity: Vector) -> Moon {
        Moon { pos, velocity }
    }

    #[cfg(test)]
    fn gravity(&mut self, g: Point) {
        let p = self.pos;
        let v = self.velocity;
        self.velocity = vector(v.x + (g.x - p.x).signum(),
                            v.y + (g.y - p.y).signum(),
                            v.z + (g.z - p.z).signum());
    }

    #[cfg(test)]
    fn movement(&mut self) {
        self.pos += self.velocity;
    }

    fn energy(&self) -> u32 {
        let potential = (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()) as u32;
        let kinetic = self.velocity.grid_len();
        potential * kinetic
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pos:{} Vel:{}", self.pos, self.velocity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: [Point; 4] = [
        point(-1, 0, 2),
        point(2, -10, -7),
        point(4, -8, 8),
        point(3, 5, -1),
    ];

    const EXAMPLE2: [Point; 4] = [
        point(-8, -10, 0),
        point(5, 5, 10),
        point(2, -7, 3),
        point(9, -8, -3),
    ];

    parameterized_test::create!{ energy, (points, steps, energy), {
        assert_eq!(simulate(&points, steps), energy);
        let (xs, ys, zs) = cycles(&points);
        assert_eq!(state_at(&xs, &ys, &zs, steps).iter().map(|m|m.energy()).sum::<u32>(), energy);

    }}
    energy!{
        a: (EXAMPLE1, 10, 179),
        b: (EXAMPLE2, 100, 1940),
    }

    parameterized_test::create!{ cycle, (points, steps), {
        let (xs, ys, zs) = cycles(&points);
        assert_eq!(cycle_len(xs, ys, zs), steps);
    }}
    cycle!{
        a: (EXAMPLE1, 2772),
        b: (EXAMPLE2, 4686774924),
    }
}
