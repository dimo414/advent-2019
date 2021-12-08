use std::fs;
use std::collections::HashSet;
use crate::euclid::{Point, point, vector, Vector};
use num_integer::Integer;
use std::cmp::Ordering;

pub fn advent() {
    let coords = read_data("data/day10.txt");
    let max = find_best_location(&coords);
    println!("Best Station at {} can see: {} asteroids", max.1, max.0);

    let swept = sweep_all(max.1, &coords);
    println!("200th asteroid vaporized is: {} ({})", swept[199], swept[199].x * 100 + swept[199].y);
}

fn read_data(file: &str) -> HashSet<Point> {
    to_coords(fs::read_to_string(file).expect("Cannot open").trim())
}

fn to_coords(input: &str) -> HashSet<Point> {
    let mut ret = HashSet::new();
    let mut cur_point = point(0, 0);
    for char in input.chars() {
        match char {
            '.' => {},
            '\n' => cur_point = point(-1, cur_point.y+1),
            '#' => { ret.insert(cur_point); },
            _ => panic!("Unexpected char: {}", char),
        }
        cur_point += vector(1, 0);
    }
    ret
}

fn vector_between(p1: Point, p2: Point) -> Vector {
    let v = p2 - p1;
    match v {
        Vector { x: 0, .. } => vector(0, v.y.signum()),
        Vector { y: 0, ..} => vector(v.x.signum(), 0),
        _ => {
            let gcd = v.x.gcd(&v.y);
            vector(v.x / gcd, v.y / gcd)
        },
    }
}

fn vectors_for(asteroid: Point, asteroids: &HashSet<Point>) -> HashSet<Vector> {
    let mut ret = HashSet::new();
    for a in asteroids {
        if &asteroid == a { continue; }
        ret.insert(vector_between(asteroid, *a));
    }
    ret
}

fn find_best_location(asteroids: &HashSet<Point>) -> (usize, Point) {
    let mut max: Option<(usize, Point)> = None;
    for asteroid in asteroids {
        let visible = vectors_for(*asteroid, asteroids).len();
        if max.is_none() || max.unwrap().0 < visible {
            max = Some((visible, *asteroid));
        }
    }
    max.expect("No asteroids found?")
}

// https://old.reddit.com/r/adventofcode/comments/e8r1jx
fn compare_angle(a: Vector, b: Vector) -> Ordering {
    let da = a.x < 0;
    let db = b.x < 0;
    if da != db { return da.cmp(&db); }
    if a.x == 0 && b.x == 0 { return a.y.signum().cmp(&b.y.signum()); }
    0.cmp(&(a.x * b.y - a.y * b.x).signum())
}

fn sweep_order(vectors: &HashSet<Vector>) -> Vec<Vector> {
    let mut vec: Vec<_> = vectors.iter().cloned().collect();
    //vec.sort_by_key(|v| OrderedFloat(v.y as f64 / v.x as f64));
    vec.sort_by(|a, b| compare_angle(*a, *b));
    vec
}

fn sweep_all(laser: Point, asteroids: &HashSet<Point>) -> Vec<Point> {
    let mut asteroids = asteroids.clone();
    assert!(asteroids.remove(&laser)); // laser must be on an asteroid
    let mut ret = Vec::new();
    while !asteroids.is_empty() {
        for vector in sweep_order(&vectors_for(laser, &asteroids)) {
            for i in 1.. {
                let point = laser + (vector * i);
                if asteroids.remove(&point) {
                    ret.push(point);
                    break;
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    parameterized_test::create!{ reduced_vector, (p1, p2, expected), {
        assert_eq!(vector_between(p1, p2), expected);
    }}
    reduced_vector!{
        a: (point(5, 5), point(5, 5), vector(0, 0)),
        b: (point(10, 0), point(0, 0), vector(-1, 0)),
        c: (point(-2, -2), point(8, 0), vector(5, 1)),
        d: (point(0, 0), point(5, 4), vector(5, 4)),
        e: (point(0, 0), point(-4936, 6170), vector(-4, 5)),
    }

    parameterized_test::create!{ best_location, (file, count, coord), {
        let coords = read_data(&format!("data/day10-example{}.txt", file));
        assert_eq!(find_best_location(&coords), (count, coord));
    }}
    best_location!{
        a: (1, 8, point(3, 4)),
        b: (2, 33, point(5, 8)),
        c: (3, 35, point(1, 2)),
        d: (4, 41, point(6, 3)),
        e: (5, 210, point(11, 13)),
    }

    parameterized_test::create!{ vaporize, (file, count, points) ,{
        let coords = read_data(&format!("data/day10-example{}.txt", file));
        let max = find_best_location(&coords);
        let swept = sweep_all(max.1, &coords);
        assert_eq!(swept.len(), count);
        for (i, point) in points {
            assert_eq!(swept[i], point);
        }
    }}
    vaporize!{
        a: (6, 36,
            vec!(point(8, 1), point(9, 0), point(9, 1), point(10, 0), point(9, 2), point(11, 1),
                point(12, 1), point(11, 2), point(15, 1), point(12, 2), point(13, 2), point(14, 2),
                point(15, 2), point(12, 3), point(16, 4), point(15, 4), point(10, 4), point(4, 4),
                point(2, 4), point(2, 3), point(0, 2), point(1, 2), point(0, 1), point(1, 1),
                point(5, 2), point(1, 0), point(5, 1), point(6, 1), point(6, 0), point(7, 0),
                point(8, 0), point(10, 1), point(14, 0), point(16, 1), point(13, 3), point(14, 3))
            .into_iter().enumerate().collect::<Vec<_>>()),
        b: (5, 299,
            vec!((0, point(11, 12)), (1, point(12, 1)), (2, point(12, 2)), (9, point(12, 8)),
                (19, point(16, 0)), (49, point(16, 9)), (99, point(10, 16)), (198, point(9, 6)),
                (199, point(8, 2)), (200, point(10, 9)), (298, point(11, 1)))),
    }
}
