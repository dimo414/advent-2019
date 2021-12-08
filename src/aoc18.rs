use std::collections::HashMap;
use crate::euclid::{point, Point, vector};
use std::str::FromStr;
use crate::error::ParseError;
use std::fmt;
use crate::pathfinding::{Graph, Edge};

pub fn advent() {
    // https://old.reddit.com/r/adventofcode/comments/ednz2o/2019_day_18_for_dummies/fbk1qg3/
    let map = read_data();
    if interactive!() {
        println!("{}", map);
    }

    println!("Scouted Route: {}", map.route_len());

    let robo_map = RoboMap::create(&map);
    println!("Robots' Route: {}", robo_map.route_len());
}

fn read_data() -> Map {
    std::fs::read_to_string("data/day18.txt").expect("Not found").parse().expect("Invalid")
}

#[derive(Debug, Copy, Clone)]
enum Type {
    Wall,
    Hall,
    Door(char),
    Key(char),
}

impl Type {
    fn lookup(c: char) -> Type {
        use Type::*;
        match c {
            '#' => Wall,
            '.'|'@' => Hall,
            'A'..='Z' => Door(c),
            'a'..='z' => Key(c),
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct ScanState {
    pos: Point,
    keys: CharSet,
}

impl ScanState {
    fn create(pos: Point, keys: CharSet) -> ScanState {
        ScanState { pos, keys }
    }

    fn moved_to(&self, pos: Point) -> ScanState {
        ScanState::create(pos, self.keys)
    }

    fn found_key(&self, key: char) -> ScanState {
        let mut keys = self.keys;
        keys.insert(key); // may be a no-op, that's OK
        ScanState::create(self.pos, keys)
    }
}

#[derive(Debug)]
struct Map {
    coords: HashMap<Point, Type>,
    entrance: Point,
    keys: HashMap<char, Point>,
}

impl Map {
    fn route_len(&self) -> usize {
        let goal = CharSet::create(&self.keys.keys().collect::<String>());
        self.bfs(&ScanState::create(self.entrance, CharSet::create("")), |n| n.keys == goal)
            .expect("No route").len() - 1
    }
}

impl Graph for Map {
    type Node = ScanState;

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        [vector(0, 1), vector(1, 0), vector(0, -1), vector(-1, 0)].iter()
            .map(|v| source.pos + v)
            .filter_map(|p| {
                let next = source.moved_to(p);
                match self.coords.get(&p) {
                    Some(Type::Key(k)) =>  Some(next.found_key(*k)),
                    Some(Type::Hall) => Some(next),
                    Some(Type::Door(d)) => if source.keys.contains(*d) { Some(next) } else { None },
                    _ => None,
                }
            })
            .map(|d| Edge::new(1, *source, d))
            .collect()
    }
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut coords = HashMap::new();
        let mut keys = HashMap::new();
        let mut entrance = None;

        let mut coord = Point::ORIGIN;
        for c in s.chars() {
            match c {
                '.'|'@'|'#'|'A'..='Z'|'a'..='z' => {
                    if c == '@' {
                        assert!(entrance.is_none(), "Multiple entrances!");
                        entrance = Some(coord);
                    }
                    let t = Type::lookup(c);
                    if let Type::Key(k) = t {
                        keys.insert(k, coord);
                    }
                    coords.insert(coord, t);
                }
                '\n' => { coord = point(-1, coord.y+1); },
                _ => panic!(),
            }
            coord += vector(1, 0);
        }

        Ok(Map { coords, entrance: entrance.expect("No entrance found."), keys, })
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let bounds = Point::bounding_box(self.coords.keys().cloned()).expect("No vault");
        for y in bounds.0.y..bounds.1.y+1 {
            for x in bounds.0.x..bounds.1.x+1 {
                let coord = point(x, y);
                if coord == self.entrance {
                    out.push('@');
                } else {
                    let c = match self.coords.get(&coord) {
                        Some(Type::Wall) => '█',
                        Some(Type::Hall) => ' ',
                        Some(Type::Door(d)) => *d,
                        Some(Type::Key(k)) => *k,
                        None => panic!(),
                    };
                    out.push(c);
                }
            }
            out.push('\n');
        }
        out.pop();
        write!(f, "{}", out)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct RoboState {
    pos: [Point; 4],
    active: Option<usize>,
    keys: CharSet,
}

impl RoboState {
    fn initial(pos: [Point; 4]) -> RoboState {
        RoboState { pos, active: None, keys: CharSet::create("") }
    }

    fn create(pos: [Point; 4], active: usize, keys: CharSet) -> RoboState {
        RoboState { pos, active: Some(active), keys }
    }

    fn moved_to(&self, point: Point) -> RoboState {
        let mut pos = self.pos;
        pos[self.active.expect("Must be active")] = point;
        RoboState::create(pos, self.active.expect("Must be active"), self.keys)
    }

    fn found_key(&self, key: char) -> Vec<RoboState> {
        let mut keys = self.keys;
        // not safe to no-op insert here, because we reactivate the other robots only when a _new_
        // key is found, not when we happen back across an already-found key's location
        assert!(keys.insert(key));
        (0..self.pos.len())
            .map(|i| RoboState::create(self.pos, i, keys))
            .collect()
    }
}

#[derive(Debug)]
struct RoboMap {
    coords: HashMap<Point, Type>,
    entrances: [Point; 4],
    keys: HashMap<char, Point>,
}

impl RoboMap {
    fn create(map: &Map) -> RoboMap {
        let mut coords = map.coords.clone();
        for v in [vector(0, 0), vector(0, 1), vector(1, 0), vector(0, -1), vector(-1, 0)].iter() {
            coords.insert(map.entrance + v, Type::Wall);
        }
        let e = map.entrance;
        let entrances = [e + vector(-1, -1), e + vector(1, -1), e + vector(-1, 1), e + vector(1, 1)];

        RoboMap { coords, entrances, keys: map.keys.clone() }
    }

    fn route_len(&self) -> i32 {
        let goal = CharSet::create(&self.keys.keys().collect::<String>());
        self.bfs(&RoboState::initial(self.entrances), |n| n.keys == goal)
            .expect("No route").len() as i32 - 2 // 2, start and "initial", see below
    }
}

impl Graph for RoboMap {
    type Node = RoboState;

    fn neighbors(&self, source: &Self::Node) -> Vec<Edge<Self::Node>> {
        if source.active.is_none() { // from RoboState::initial()
            return (0..source.pos.len())
                .map(|i| RoboState::create(source.pos, i, source.keys))
                // it would be more correct for these edges to be zero-weight and use dijkstras, but
                // doing so incurs a significant overhead. Since exactly one of these nodes will
                // appear at the start any valid path (and nowhere else) it's simple enough to just
                // exclude the extra node from the result.
                .map(|d| Edge::new(1, *source, d))
                .collect();
        }

        [vector(0, 1), vector(1, 0), vector(0, -1), vector(-1, 0)].iter()
            .map(|v| source.pos[source.active.expect("Must be active")] + v)
            .map(|p| (p, self.coords.get(&p)))
            .filter_map(|(p, t)|
                match t {
                    None|Some(Type::Wall) => None,
                    Some(t) => Some((p, *t))
                }
            )
            .flat_map(|(p, t)| {
                let next = source.moved_to(p);
                match t {
                    Type::Key(k) => if source.keys.contains(k) { vec!(next) } else { next.found_key(k) },
                    Type::Door(d) => if source.keys.contains(d) { vec!(next) } else { vec!() },
                    Type::Hall => vec!(next),
                    _ => unreachable!(),
                }
            })
            .map(|d| Edge::new(1, *source, d))
            .collect()
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct CharSet {
    bits: u32,
}

impl CharSet {
    fn create(s: &str) -> CharSet {
        let bits = s.chars().fold(0, |ac, c| ac | CharSet::to_mask(c));
        CharSet { bits }
    }

    fn insert(&mut self, c: char) -> bool {
        let orig = self.bits;
        self.bits |= CharSet::to_mask(c);
        orig != self.bits
    }

    #[cfg(test)]
    fn remove(&mut self, c: char) -> bool {
        let orig = self.bits;
        self.bits &= !CharSet::to_mask(c);
        orig != self.bits
    }

    fn contains(&self, c: char) -> bool {
        self.bits & CharSet::to_mask(c) != 0
    }

    fn to_mask(c: char) -> u32 {
        let c = c.to_ascii_uppercase();
        assert!(('A'..='Z').contains(&c));
        let idx = (c as u8 - b'A') as u32;
        1 << idx
    }
}

impl fmt::Debug for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for c in (b'A' ..= b'Z').map(char::from) {
            if self.contains(c) {
                out.push(c);
            }
        }
        write!(f, "{{{}}}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
        #########\n\
        #b.A.@.a#\n\
        #########";

    const EXAMPLE_2: &str = "\
        ########################\n\
        #f.D.E.e.C.b.A.@.a.B.c.#\n\
        ######################.#\n\
        #d.....................#\n\
        ########################";

    const EXAMPLE_3: &str = "\
        ########################\n\
        #...............b.C.D.f#\n\
        #.######################\n\
        #.....@.a.B.c.d.A.e.F.g#\n\
        ########################";

    const EXAMPLE_4: &str = "\
        #################\n\
        #i.G..c...e..H.p#\n\
        ########.########\n\
        #j.A..b...f..D.o#\n\
        ########@########\n\
        #k.E..a...g..B.n#\n\
        ########.########\n\
        #l.F..d...h..C.m#\n\
        #################";

    const EXAMPLE_5: &str = "\
        ########################\n\
        #@..............ac.GI.b#\n\
        ###d#e#f################\n\
        ###A#B#C################\n\
        ###g#h#i################\n\
        ########################";

    const EXAMPLE_6: &str = "\
        #######\n\
        #a.#Cd#\n\
        ##...##\n\
        ##.@.##\n\
        ##...##\n\
        #cB#Ab#\n\
        #######";

    const EXAMPLE_7: &str = "\
        ###############\n\
        #d.ABC.#.....a#\n\
        ######...######\n\
        ######.@.######\n\
        ######...######\n\
        #b.....#.....c#\n\
        ###############";

    const EXAMPLE_8: &str = "\
        #############\n\
        #DcBa.#.GhKl#\n\
        #.###...#I###\n\
        #e#d#.@.#j#k#\n\
        ###C#...###J#\n\
        #fEbA.#.FgHi#\n\
        #############";

    const EXAMPLE_9: &str = "\
    #############\n\
    #g#f.D#..h#l#\n\
    #F###e#E###.#\n\
    #dCba...BcIJ#\n\
    #####.@.#####\n\
    #nK.L...G...#\n\
    #M###N#H###.#\n\
    #o#m..#i#jk.#\n\
    #############";

    parameterized_test::create!{ shortest_path, (text, dist), {
        let map: Map = text.parse().unwrap();
        let map_str = map.to_string().replace(" ", ".").replace("█", "#");
        assert_eq!(map_str, text);

        assert_eq!(map.route_len(), dist);
    }}
    shortest_path!{
        a: (EXAMPLE_1, 8),
        b: (EXAMPLE_2, 86),
        c: (EXAMPLE_3, 132),
        d: (EXAMPLE_4, 136),
        e: (EXAMPLE_5, 81),
    }

    parameterized_test::create!{ shortest_robo_path, (text, dist), {
        let map = RoboMap::create(&text.parse().unwrap());

        assert_eq!(map.route_len(), dist);
    }}
    shortest_robo_path!{
        a: (EXAMPLE_6, 8),
        b: (EXAMPLE_7, 24),
        c: (EXAMPLE_8, 32),
        d: (EXAMPLE_9, 72),
    }

    const TWO: u32 = 2;

    parameterized_test::create!{ charsets, (letters, expected), {
        let mut set = CharSet::create(letters);
        assert_eq!(set.bits, expected);
        for c in letters.chars() {
            assert!(set.contains(c));
            assert!(!set.insert(c));
        }

        let mut set2 = CharSet::create("");
        for c in letters.chars().rev() {
            assert!(!set2.contains(c));
            assert!(set2.insert(c));
        }
        assert_eq!(set, set2);

        for c in letters.chars() {
            assert!(set2.remove(c));
            assert!(!set2.contains(c));
            assert!(!set2.remove(c));
        }
        assert_eq!(set2, CharSet::create(""));

        assert_eq!(format!("{:?}", set), format!("{{{}}}", letters));
    }}
    charsets!{
        a: ("A", TWO.pow(0)),
        ab: ("AB", TWO.pow(0) |  TWO.pow(1)),
        dgjsz: ("DGJSZ", TWO.pow(3) | TWO.pow(6) | TWO.pow(9) | TWO.pow(18) | TWO.pow(25)),
    }
}