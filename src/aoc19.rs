use crate::intcode::Machine;
use crate::euclid::{point,Point};
use std::collections::HashSet;

pub fn advent() {
    let mut traction = HashSet::new();
    for y in {0..50} {
        for x in {0..50} {
            let coord = point(x,y);
            if in_traction(coord) {
                traction.insert(coord);
            }
        }
    }
    println!("Coords in traction within 50x50: {}", traction.len());

    // The tractor beam is tricksy; it has no traction, other than at the origin, within the first
    // several squares of the beam. The beam only gets "wide" enough to be detected further away.
    let non_origin_coord = traction.iter().filter(|&&p| p != Point::ORIGIN).min().unwrap();

    let mut widths: Vec<_> = (0..non_origin_coord.y).map(|_| (0,0)).collect();
    widths.push((non_origin_coord.x as usize, non_origin_coord.x as usize));

    let target_width = 100;
    loop {
        widths.push(width_for(widths.len(), widths[widths.len()-1]));
        let lower = widths[widths.len()-1];
        if lower.1-lower.0+1 >= target_width {
            let upper = widths[widths.len()-target_width];
            if upper.1-lower.0+1 >= target_width {
                let bounds = (
                    point(lower.0 as i32, (widths.len()-target_width) as i32),
                    point(upper.1 as i32, (widths.len()-1) as i32));
                println!("{}x{0} rectangle found at {} -> {}", target_width, bounds.0, bounds.1);
                println!("Identifier for nearest coord: {}", (bounds.0.x*10000)+bounds.0.y);
                break;
            }
        }
    }
}

fn width_for(y: usize, prior: (usize, usize)) -> (usize, usize) {
    //println!("Checking {} in range {:?}", y, prior);
    let mut min_x = None;
    for x in (0..prior.0+2).rev() {
        if in_traction(point(x as i32, y as i32)) {
            min_x = Some(x);
        } else if min_x.is_some() { break; }
    }
    let min_x = min_x.unwrap();

    let mut max_x = None;
    for x in {prior.1-1..} {
        if in_traction(point(x as i32, y as i32)) {
            max_x = Some(x);
        } else if max_x.is_some() { break; }
    }
    let max_x = max_x.unwrap();

    (min_x, max_x)
}

fn in_traction(coord: Point) -> bool {
    lazy_static!{
        static ref SOURCE: Vec<i64> = Machine::from_file("data/day19.txt").state;
    }
    let mut machine = Machine::new(&SOURCE);
    // TODO the algorithm above is sensitive to the fact that our beam is narrow and pointed
    // downward (i.e. expands slowly in the x relative to the y); flipping the x/y here ought to
    // be OK, but it causes this algorithm to crash.
    machine.send_input(coord.x as i64);
    machine.send_input(coord.y as i64);
    let output = machine.run_until(1).expect("No output");
    output[0] == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn machine() {
        assert!(in_traction(Point::ORIGIN));
    }
}