use std::collections::{HashMap, HashSet};
use crate::euclid::{point,Point,vector};
use crate::pathfinding::{Graph, Edge};
use std::str::FromStr;
use crate::error::ParseError;
use std::collections::hash_map::Entry;

pub fn advent() {
    let maze = read_data("data/day20.txt");
    println!("Distance: {}", maze.route().len());

    let rec_maze = RecursiveMaze::new(&maze);
    println!("Recursive Distance: {}", rec_maze.route().len());
}

fn read_data(file: &str) -> Maze {
    std::fs::read_to_string(file).expect("Cannot open").parse().expect("Invalid maze")
}

#[derive(Debug)]
struct Maze {
    points: HashSet<Point>,
    portals: HashMap<Point, Point>,
    center: Point,
    start: Point,
    goal: Point,
}

impl Maze {
    fn portal(&self, source: &Point) -> Option<(Point, i32)> {
        match self.portals.get(source) {
            Some(dest) => {
                let source_dist = (*source - self.center).len();
                let dest_dist = (*dest - self.center).len();
                Some((*dest, if source_dist < dest_dist { 1 } else { -1 }))
            },
            None => None,
        }
    }

    fn route(&self) -> Vec<Edge<Point>> {
        self.dijkstras(&self.start, |n| n == &self.goal).expect("No path!")
    }
}

impl FromStr for Maze {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let chars: HashMap<Point, char> = s.lines().enumerate()
            .flat_map(|(y, l)| l.chars().enumerate()
                .map(move |(x, c)| (point(x as i32, y as i32), c)).collect::<Vec<_>>())
            .collect();

        let bounds = Point::bounding_box(chars.keys().cloned()).expect("No points");
        assert_eq!(bounds.0, Point::ORIGIN);
        let center = point(bounds.1.x / 2, bounds.1.y / 2);

        let mut points = HashSet::new();
        let mut labels: HashMap<_, Point> = HashMap::new();
        let mut portals = HashMap::new();

        for (coord, c) in chars.iter() {
            match c {
                ' '|'#' => {},
                '.' => { points.insert(*coord); },
                'A'..='Z' => {
                    for dir in [vector(1,0), vector(0,1)].iter() {
                        let other = coord + dir;
                        if let Some(c2) = chars.get(&other) {
                            if (&'A'..=&'Z').contains(&c2) {
                                let key = (*c, *c2);
                                let source =
                                    if let Some('.') = chars.get(&(other + dir)) { other + dir }
                                    else if let Some('.') = chars.get(&(coord + (*dir * -1))) { *coord + (*dir * -1) }
                                    else { panic!() };

                                //points.insert(source);
                                match labels.entry(key) {
                                    Entry::Occupied(e) => {
                                        let dest = e.remove();
                                        portals.insert(source, dest);
                                        portals.insert(dest, source);
                                    },
                                    Entry::Vacant(e) => {
                                        e.insert(source);
                                    }
                                }
                            }
                        }
                    }
                },
                _ => panic!(),
            }
        }

        let start = labels.remove(&('A','A')).expect("No start found");
        let goal = labels.remove(&('Z','Z')).expect("No goal found");
        assert!(labels.is_empty());

        Ok(Maze { points, portals, center, start, goal })
    }
}

impl Graph for Maze {
    type Node = Point;

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        let direct: Vec<_> = [vector(0, 1), vector(1, 0), vector(0, -1), vector(-1, 0)].iter()
            .map(|v| source + v)
            .filter(|p| self.points.get(p).is_some())
            .collect();

        let portal = self.portals.get(source);

        direct.iter().chain(portal.iter().cloned())
            .map(|p| Edge::new(1, *source, *p))
            .collect()
    }
}

struct RecursiveMaze<'a> {
    maze: &'a Maze,
}

impl<'a> RecursiveMaze<'a> {
    fn new(maze: &'a Maze) -> RecursiveMaze<'a> {
        RecursiveMaze { maze }
    }

    fn route(&self) -> Vec<Edge<(Point, i32)>> {
        self.dijkstras(&(self.maze.start, 0), |n| n == &(self.maze.goal, 0)).expect("No path!")
    }
}

impl<'a> Graph for RecursiveMaze<'a> {
    type Node = (Point, i32);

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        assert!(source.1 < 500, "No path within 500 layers");

        let portal = self.maze.portal(&source.0)
            .filter(|(_, d)| source.1 + *d >= 0)
            .map(|(dest, depth)| Edge::new(1, *source, (dest, source.1 + depth)));

        [vector(0, 1), vector(1, 0), vector(0, -1), vector(-1, 0)].iter()
            .map(|v| source.0 + v)
            .filter(|p| self.maze.points.get(p).is_some())
            .map(|p| Edge::new(1, *source, (p, source.1)))
            .chain(portal.into_iter())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let maze = read_data("data/day20-example1.txt");
        assert_eq!(maze.route().len(), 23);

        let rec_maze = RecursiveMaze::new(&maze);
        assert_eq!(rec_maze.route().len(), 26);
    }

    #[test]
    fn example2() {
        let maze = read_data("data/day20-example2.txt");
        assert_eq!(maze.route().len(), 58);

        // No recursive route
    }

    #[test]
    fn example3() {
        let maze = read_data("data/day20-example3.txt");
        assert_eq!(maze.route().len(), 77); // not provided by example

        let rec_maze = RecursiveMaze::new(&maze);
        assert_eq!(rec_maze.route().len(), 396);
    }
}