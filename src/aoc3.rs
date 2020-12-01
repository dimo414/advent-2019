use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::euclid::{Point, Vector, vector};
use crate::error::ParseError;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

pub fn advent() {
    let (one, two) = read_data();
    let (one, two) = (trace_wire(&one), trace_wire(&two));
    let nearest = nearest_intersection(&one, &two);
    println!("Nearest Crossing Dist: {}", (nearest.expect("No crossing found") - Point::ORIGIN).grid_len());
    let earliest = earliest_intersection_steps(&one, &two);
    println!("Earliest Crossing Steps: {}", earliest.expect("No crossing found"));
}

fn read_data() -> (Vec<Dir>,Vec<Dir>) {
    let reader = BufReader::new(File::open("data/day3.txt").expect("Cannot open"));

    let lines: Vec<_> = reader.lines().map(|l| l.unwrap()).collect();
    assert_eq!(lines.len(), 2);
    let one: Vec<_> = lines[0].trim().split(",").map(|v| v.parse::<Dir>().unwrap()).collect();
    let two: Vec<_> = lines[1].trim().split(",").map(|v| v.parse::<Dir>().unwrap()).collect();
    (one, two)
}

fn trace_wire(route: &Vec<Dir>) -> Vec<Point> {
    let mut cur = Point::ORIGIN;
    let mut points = Vec::new();
    for dir in route {
        for _ in 0..dir.1 {
            // mutate first, don't include (0, 0)
            cur += dir.0;
            points.push(cur.clone());
        }
    }
    points
}

fn intersects(one: &Vec<Point>, two: &Vec<Point>) -> HashSet<Point> {
    let one: HashSet<_> = one.iter().cloned().collect();
    let two: HashSet<_> = two.iter().cloned().collect();
    one.intersection(&two).cloned().collect()
}

fn nearest_intersection(one: &Vec<Point>, two: &Vec<Point>) -> Option<Point> {
    intersects(one, two).into_iter().min_by_key(|&p| (p - Point::ORIGIN).grid_len())
}

fn earliest_intersection_steps(one: &Vec<Point>, two: &Vec<Point>) -> Option<usize> {
    let intersects = intersects(one, two);
    intersects.into_iter()
        .map(|p| 2 + // add one for each path, step one is at index zero
            one.iter().position(|&pp| p == pp).unwrap() +
            two.iter().position(|&pp| p == pp).unwrap())
        .min()
}

#[derive(Debug, Eq, PartialEq)]
struct Dir(Vector, u32);

impl FromStr for Dir {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\D)(\d+)$").unwrap();
        }

        let caps = regex_captures!(RE, s)?;
        let dir = capture_group!(caps, 1);
        let vector = match dir {
            "R" => vector(1, 0),
            "L" => vector(-1, 0),
            "D" => vector(0, 1),
            "U" => vector(0, -1),
            _ => return Err(ParseError::Malformed(dir.to_string())),
        };
        let magnitude: u32 = capture_group!(caps, 2).parse()?;
        return Ok(Dir(vector, magnitude));
    }
}

#[cfg(test)]
mod tests {
    use crate::euclid::point;
    use super::*;

    parameterized_test!{ to_dir, (s, expected), {
            assert_eq!(s.parse::<Dir>(), expected);
        }}
    to_dir!{
            r8: ("R8", Ok(Dir(vector(1, 0), 8))),
            u5: ("U5", Ok(Dir(vector(0, -1), 5))),
            l5: ("L5", Ok(Dir(vector(-1, 0), 5))),
            d3: ("D3", Ok(Dir(vector(0, 1), 3))),
            bad: ("N2", Err(ParseError::Malformed("N".into()))),
        }

    #[test]
    fn wire() {
        let wire = trace_wire(&vec!(
            // R4,U2,L2,D1
            Dir(vector(1,0), 4),
            Dir(vector(0,-1), 2),
            Dir(vector(-1,0), 2),
            Dir(vector(0,1), 1)
        ));
        let expected = vec!(
            point(1, 0), point(2, 0), point(3, 0), point(4, 0), // R4
            point(4, -1), point(4, -2), // U2
            point(3, -2), point(2, -2), // L2
            point(2, -1) // D1
        );

        assert_eq!(wire, expected);
    }

    #[test]
    fn nearest() {
        let one: Vec<Dir> = vec!("R8","U5","L5","D3").iter().map(|s| s.parse().unwrap()).collect();
        let two: Vec<Dir> = vec!("U7","R6","D4","L4").iter().map(|s| s.parse().unwrap()).collect();

        assert_eq!(nearest_intersection(&trace_wire(&one), &trace_wire(&two)), Some(point(3, -3)));
    }

    #[test]
    fn earliest() {
        let one: Vec<Dir> = vec!("R8","U5","L5","D3").iter().map(|s| s.parse().unwrap()).collect();
        let two: Vec<Dir> = vec!("U7","R6","D4","L4").iter().map(|s| s.parse().unwrap()).collect();

        assert_eq!(earliest_intersection_steps(&trace_wire(&one), &trace_wire(&two)), Some(30));

    }

    #[test]
    fn read_file() {
        let data: (_, _) = read_data();
        assert!(data.0.len() > 0);
        assert!(data.1.len() > 0);
    }
}
