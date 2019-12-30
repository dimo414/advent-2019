use crate::intcode::{Machine, State};
use crate::euclid::{Point, point, Vector, vector};
use std::collections::HashMap;
use crate::pathfinding::{Graph, Edge};
use std::fmt;

pub fn advent() {
    let map = Map::explore(Machine::from_file("data/day15.txt"));
    println!("Distance to device: {}", map.distance_to_o2_system());
    println!("Minutes for Oxygen to travel: {}", map.time_for_o2_to_spread());
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Type {
    WALL,
    HALL,
    DEVICE,
}

struct Map {
    visited: HashMap<Point, Type>,
    pos: Point,
    dir: Dir,
    device: Option<Point>,
}

impl Map {
    fn explore(mut machine: Machine) -> Map {
        let mut map = Map { visited: HashMap::new(), pos: Point::ORIGIN, dir: Dir::NORTH, device: None };

        loop {
            machine.send_input(map.dir.command());
            let state = machine.run();
            let output = machine.read_output();
            assert_eq!(output.len(), 1);
            match output[0] {
                0 => {
                    map.visited.insert(map.pos + map.dir.vector(), Type::WALL);
                    if let Some(Type::WALL) = map.visited.get(&(map.pos + map.dir.right().vector())) {
                        if let Some(Type::WALL) = map.visited.get(&(map.pos + map.dir.left().vector())) {
                            map.dir = map.dir.left();
                        } else {
                            map.dir = map.dir.flip();
                        }
                    } else {
                        map.dir = map.dir.right();
                    }
                },
                1|2 => {
                    map.pos += map.dir.vector();
                    // keep-left
                    map.dir = map.dir.left();
                    let t = match output[0] {
                        1 => Type::HALL,
                        2 => Type::DEVICE,
                        _ => panic!(),
                    };
                    if t == Type::DEVICE {
                        map.device = Some(map.pos);
                    }
                    map.visited.insert(map.pos, t);
                    if map.pos == Point::ORIGIN { break; }
                },
                _ => panic!(),
            }
            if interactive!() {
                let image = map.to_string();
                println!("{}\u{001B}[{}A", image, image.chars().filter(|&c| c == '\n').count() + 1);
                //std::thread::sleep(std::time::Duration::from_millis(5));
            }
            match state {
                State::INPUT => {},
                State::HALT => { break; },
                _ => panic!(),
            }
        }
        if interactive!() {
            println!("{}", map);
        }

        map
    }

    fn distance_to_o2_system(&self) -> u32 {
        self.dijkstras(&Point::ORIGIN, &self.device.expect("Device not found"))
            .expect("No path").len() as u32
    }

    fn time_for_o2_to_spread(&self) -> u32 {
        self.bfs_all(&self.device.expect("Device not found"))
            .values().map(|v| v.len()).max().expect("No routes") as u32 - 1
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bounds = Point::bounding_box(self.visited.keys().cloned()).expect("No points");
        let mut out = String::new();
        for y in bounds.0.y..bounds.1.y+1 {
            for x in bounds.0.x..bounds.1.x + 1 {
                let coord = point(x, y);
                use Type::*;
                let c = match self.visited.get(&coord) {
                    Some(WALL) => '█',
                    Some(HALL) => ' ',
                    Some(DEVICE) => 'X',
                    None => '░',
                };
                let c = if coord == self.pos { '#' } else { c };
                out.push(c);
            }
            out.push('\n');
        }
        out.pop();
        write!(f, "{}", out)
    }
}

impl Graph for Map {
    type Node = Point;

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        vec!(vector(0, 1), vector(1, 0), vector(0, -1), vector(-1, 0)).iter()
            .map(|v| source + v)
            .filter(|p| self.visited.get(p).unwrap_or(&Type::WALL) != &Type::WALL)
            .map(|d| Edge::new(1, source.clone(), d.clone()))
            .collect()
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

impl Dir {
    fn command(&self) -> i64 {
        use Dir::*;
        match self {
            NORTH => 1,
            SOUTH => 2,
            WEST => 3,
            EAST => 4,
        }
    }

    fn vector(&self) -> Vector {
        use Dir::*;
        match self {
            NORTH => vector(0, -1),
            SOUTH => vector(0, 1),
            WEST => vector(-1, 0),
            EAST => vector(1, 0),
        }
    }

    fn right(&self) -> Dir {
        use Dir::*;
        match self {
            NORTH => EAST,
            SOUTH => WEST,
            WEST => NORTH,
            EAST => SOUTH,
        }
    }

    fn flip(&self) -> Dir {
        use Dir::*;
        match self {
            NORTH => SOUTH,
            SOUTH => NORTH,
            WEST => EAST,
            EAST => WEST,
        }
    }

    fn left(&self) -> Dir { self.flip().right() }
}