use crate::intcode::{Machine, State};
use crate::euclid::{point, Point, vector, Vector};
use std::collections::HashSet;
use std::fmt::Write;
use regex::Regex;

pub fn advent() {
    let image = read_data();
    let mut machine = image.clone();
    machine.run().assert_halt();
    let display = machine.read_output_ascii();

    let (start, points) = plot_map(&display);
    println!("Alignment Parameters: {}", intersection_points_sum(&points));

    let path = gen_path(start, &points);

    let (comp,a,b,c) = compress(&path).expect("Encoding not found");

    let mut machine = image.clone();
    machine.set_state(0, 2);

    machine.run().assert_input();
    let output = machine.read_output_ascii();
    assert_eq!(output[0..display.len()], display);
    assert_eq!(&output[display.len()..], "Main:\n");
    machine.send_input_ascii(&format!("{}\n", comp));

    machine.run().assert_input();
    assert_eq!(machine.read_output_ascii(), "Function A:\n");
    machine.send_input_ascii(&format!("{}\n", a));

    machine.run().assert_input();
    assert_eq!(machine.read_output_ascii(), "Function B:\n");
    machine.send_input_ascii(&format!("{}\n", b));

    machine.run().assert_input();
    assert_eq!(machine.read_output_ascii(), "Function C:\n");
    machine.send_input_ascii(&format!("{}\n", c));

    machine.run().assert_input();
    assert_eq!(machine.read_output_ascii(), "Continuous video feed?\n");
    let debug = if cfg!(debug_assertions) { 'y' } else { 'n' };
    machine.send_input_ascii(&format!("{}\n", debug));

    if debug == 'y' {
        // skip the preceeding whitespace
        assert_eq!(machine.run_until(|o| o == &['\n' as i64]), State::OUTPUT);
        assert_eq!(machine.read_output_ascii(), "\n");

        let mut display = display;
        loop {
            match machine.run_until(|o| o.len() >= 2 && &o[o.len()-2..] == &['\n' as i64, '\n' as i64]) {
                State::OUTPUT => {
                    display = machine.read_output_ascii();
                    let display = &display[..display.len()-1]; // remove the extra trailing newline
                    print!("{}\u{001B}[{}A", display.replace('.', " "), display.chars().filter(|&c| c=='\n').count());
                },
                State::HALT => { println!("{}", display.replace('.', " ")); break; },
                _ => panic!(),
            }
        }
    } else {
        machine.run().assert_halt();
    }
    println!("Dust collected: {}", machine.read_output().last().expect("No output remaining"));
}

fn read_data() -> Machine {
    Machine::from_file("data/day17.txt")
}

fn plot_map(input: &str) -> (Point,HashSet<Point>) {
    let mut pos = Point::ORIGIN;
    let mut start = None;
    let mut points = HashSet::new();
    for c in input.chars() {
        match c {
            '#'|'^'|'v'|'<'|'>' => { points.insert(pos); if c != '#' { start = Some(pos) } },
            '.' => {},
            '\n' => { pos = point(-1, pos.y+1); },
            _ => panic!(),
        };
        pos += vector(1, 0);
    }
    (start.expect("Robot not found"), points)
}

fn intersection_points_sum(points: &HashSet<Point>) -> i32 {
    points.iter().filter(|&p|
            // Only count four-way intersections, since we know the scaffold forms a path there
            // shouldn't be any T-junctions or other intersection types
            points.contains(&(p + vector(-1, 0))) &&
            points.contains(&(p + vector(1, 0))) &&
            points.contains(&(p + vector(0, 1))) &&
            points.contains(&(p + vector(0, -1))))
        .map(|&p| p.x * p.y)
        .sum()
}

fn gen_path(start: Point, points: &HashSet<Point>) -> String {
    let mut out = String::new();
    let mut pos = start;
    // TODO this assumes the robot starts facing-up, which may not always be true
    let mut dir = vector(0, -1);
    // TODO this assumes the robot starts facing sideways to the path, which may not always be true
    loop {
        if let Some(next) = find_dir(pos, dir, &points) {
            write!(out, "{},", to_letter(dir, next)).unwrap();
            dir = next;
            let mut dist = 0;
            loop {
                let next = pos + dir;
                if points.contains(&next) {
                    dist += 1;
                    pos = next;
                } else {
                    write!(out, "{},", dist).unwrap();
                    break;
                }
            }
        } else {
            break;
        }
    }
    out.pop();
    out
}

fn find_dir(pos: Point, cur_dir: Vector, points: &HashSet<Point>) -> Option<Vector> {
    let ew = &[vector(-1,0), vector(1, 0)];
    let ns = &[vector(0, -1), vector(0, 1)];

    for v in if cur_dir.x != 0 {ns} else {ew} {
        if points.contains(&(pos + v)) { return Some(*v); }

    }
    None
}

fn to_letter(vec: Vector, next: Vector) -> char {
    match vec {
        Vector{x:-1, y:0} => match next {
            Vector{x:0, y:-1} => 'R',
            Vector{x:0, y:1} => 'L',
            _ => panic!(),
        },
        Vector{x:1, y:0} => match next {
            Vector{x:0, y:-1} => 'L',
            Vector{x:0, y:1} => 'R',
            _ => panic!(),
        },
        Vector{x:0, y:-1} => match next {
            Vector{x:-1, y:0} => 'L',
            Vector{x:1, y:0} => 'R',
            _ => panic!(),
        },
        Vector{x:0, y:1} => match next {
            Vector{x:-1, y:0} => 'R',
            Vector{x:1, y:0} => 'L',
            _ => panic!(),
        },
        _ => panic!(),
    }
}

// Compresses a comma-separated string into groups of three substrings, labeled A, B, and C.
// The resulting string is fully compressed (only 'A', 'B', 'C', and ',' will be left), and the
// subsequent three returned values are the A, B, and C expansions.
//
// This algorithm works by treating A and B as the prefix and suffix of the string, and then
// expanding each until a suitable C is found. Although seemingly acceptable in practice, this is
// flawed as it assumes two separate substrings are needed for the prefix and suffix; if one string
// would suffice that compression will not be found.
//
// e.g. "1,2,3,1" _could_ become ("A,B,C,A", "1", "2", "3").
// TODO perhaps instead we could ignore the suffix and just search left-to-right
fn compress(s: &str) -> Option<(String, String, String, String)> {
    lazy_static! {
        // A subsequence without any A or B entries
        static ref UNCOMPRESSED: Regex = Regex::new(r",([^AB]+),").unwrap();
        // A fully compressed sequence with only A, B, and C entries
        static ref ALL_COMPRESSED: Regex = Regex::new(r"^[ABC,]+$").unwrap();
    }

    let parts = s.chars().filter(|&c| c == ',').count() + 1;

    // Iterate on the size of A+B, so that we start by looking at short prefixes and suffixes
    for ab_len in {2..parts} {
        for a_len in {1..ab_len} {
            let b_len = ab_len - a_len;
            assert!(b_len > 0);
            let a = nth_comma(s, a_len);
            let b = nth_comma_rev(s, b_len);
            if a.len() > 20 || b.len() > 20 { continue; }

            let mut compressed = s.replace(a, "A");
            compressed = compressed.replace(b, "B");

            if let Ok(candidate) = regex_captures!(UNCOMPRESSED, &compressed) {
                let c = capture_group!(candidate, 1);
                let compressed = compressed.replace(c, "C");
                if ALL_COMPRESSED.is_match(&compressed) {
                    if compressed.len() > 20 || c.len() > 20 { continue; }
                    return Some((compressed, a.into(), b.into(), c.into()));
                }
            } // else shouldn't happen, but either way we don't have a match
        }
    }
    None
}

fn nth_comma<'a>(s: &'a str, nth: usize) -> &'a str {
    let mut commas_seen = 0;
    let pos = s.chars()
        .position(|c| {if c == ',' { commas_seen+=1; if commas_seen == nth { return true; }}; false})
        .expect("Not enough commas");
    &s[..pos]
}

fn nth_comma_rev<'a>(s: &'a str, nth: usize) -> &'a str {
    let mut commas_seen = 0;
    let pos = s.chars().rev()
        .position(|c| {if c == ',' { commas_seen+=1; if commas_seen == nth { return true; }}; false})
        .expect("Not enough commas");
    &s[s.len()-pos..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment() {
        let image = "..#..........\n\
            ..#..........\n\
            #######...###\n\
            #.#...#...#.#\n\
            #############\n\
            ..#...#...#..\n\
            ..#####...^..";
        let (_, map) = plot_map(&image);
        assert_eq!(intersection_points_sum(&map), 76);
    }

    #[test]
    fn compression() {
        let image = "#######...#####\n\
            #.....#...#...#\n\
            #.....#...#...#\n\
            ......#...#...#\n\
            ......#...###.#\n\
            ......#.....#.#\n\
            ^########...#.#\n\
            ......#.#...#.#\n\
            ......#########\n\
            ........#...#..\n\
            ....#########..\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#...#......\n\
            ....#####......";
        let (start, map) = plot_map(&image);
        let path = gen_path(start, &map);
        let (main, a, b, c) = compress(&path).unwrap();
        // This isn't the same output as the example, though it appears to also be valid
        // It would be nice if the compression scheme prioritized smaller mains, rather than taking
        // the first valid match
        assert_eq!(main, "A,A,C,A,B,C,A,A,A,B");
        assert_eq!(a, "R,8");
        assert_eq!(b, "L,6,L,2");
        assert_eq!(c, "R,4,R,4");
    }
}
