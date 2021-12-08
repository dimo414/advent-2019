use crate::intcode::{Machine, State};
use std::collections::HashMap;
use crate::euclid::{point, Point, vector, Vector};

pub fn advent() {
    // Don't provide a hint, thereby disabling the interactive display, since it's slow and typically
    // larger than the shell window, which messes up the rendering.
    let painted_tiles = paint(false, None).len();
    println!("(Mis)painted Tiles: {}\n", painted_tiles);

    println!("Registration:");
    let hull = paint(true, Some((point(0, 0), point(42, 5))));
    println!("{}", render(&hull));
}

fn read_data() -> Machine {
    Machine::from_file("data/day11.txt")
}

fn paint(paint_origin: bool, bounds_hint: Option<(Point, Point)>) -> HashMap<Point, i64> {
    let mut machine = read_data();
    let mut hull = HashMap::new();
    let mut dir = Dir::Up;
    let mut pos = point(0, 0);
    if paint_origin {
        hull.insert(pos, 1);
    }
    loop {
        if interactive!() && bounds_hint.is_some() {
            let image = render_debug(&hull, bounds_hint);
            println!("{}\u{001B}[{}A", image, image.chars().filter(|&c| c=='\n').count()+1);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        machine.send_input(*hull.get(&pos).unwrap_or(&0));
        let state = machine.run();
        let output = machine.read_output();
        assert!(output[0] == 0 || output[0] == 1);
        hull.insert(pos, output[0]);
        assert!(output[1] == 0 || output[1] == 1);
        dir = dir.rotate(output[1] == 1);
        pos += dir.vec();
        match state {
            State::Input => {}
            State::Halt => { break; }
            _ => panic!(),
        }

    }
    hull
}

fn render(painted: &HashMap<Point, i64>) -> String {
    render_debug(painted,Point::bounding_box(painted.keys().cloned()))
}

fn render_debug(painted: &HashMap<Point, i64>, bounds: Option<(Point, Point)>) -> String {
    let mut out = String::new();
    let bounds = bounds.or_else(|| Point::bounding_box(painted.keys().cloned())).unwrap_or((Point::ORIGIN, Point::ORIGIN));
    for y in bounds.0.y..bounds.1.y+1 {
        for x in bounds.0.x..bounds.1.x+1 {
            let coord = point(x, y);
            let paint = match painted.get(&coord) {
                Some(0) => ' ',
                Some(1) => '█',
                None => '▒',
                _ => panic!(),
            };
            out.push(paint);
        }
        out.push('\n');
    }
    out.pop();
    out
}

#[derive(Copy, Clone)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn rotate(&self, rotate_right: bool) -> Dir {
        use Dir::*;
        match self {
            Up => { if rotate_right { Right } else { Left } },
            Down => { if rotate_right { Left } else { Right } },
            Left => { if rotate_right { Up } else { Down } },
            Right => { if rotate_right { Down } else { Up } },
        }
    }

    fn vec(&self) -> Vector {
        use Dir::*;
        match self {
            Up => vector(0, -1),
            Down => vector(0, 1),
            Left => vector(-1, 0),
            Right => vector(1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute() {
        let hull = paint(true, None);
        let bounding_box = Point::bounding_box(hull.keys().cloned()).expect("No points");
        assert_eq!(bounding_box, (point(0, 0), point(42, 5)));
    }
}
