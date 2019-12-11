use crate::intcode::Machine;
use std::fs;
use std::collections::HashMap;
use crate::euclid::{point, Point, vector, Vector};

pub fn advent() {
    let painted_tiles = paint(false, false).len();
    println!("(Mis)painted Tiles: {}\n", painted_tiles);

    println!("Registration:");
    let hull = paint(true, true);
    println!("{}", render(&hull));
}

fn read_data() -> Machine {
    fs::read_to_string("data/day11.txt").expect("Cannot open").trim().parse().expect("Invalid")
}

fn paint(paint_origin: bool, print_progress: bool) -> HashMap<Point, i64> {
    let mut machine = read_data();
    let mut hull = HashMap::new();
    let mut dir = Dir::UP;
    let mut pos = point(0, 0);
    if paint_origin {
        hull.insert(pos.clone(), 1);
    }
    if cfg!(debug_assertions) && print_progress {
        print!("\u{001B}[?25l"); // hide cursor
    }
    loop {
        if cfg!(debug_assertions) && print_progress {
            // Print the image as it's being drawn. This works for the !paint_origin case too, but
            // takes much longer and will overflow most terminal windows (breaking the ANSI cursor
            // movement), so we don't bother. The bounds are hard-coded to make the image look
            // better, but it will also work if the bounds are computed on each iteration.
            let image = render_debug(&hull, (point(0, 0), point(42, 5)));
            println!("{}\u{001B}[{}A", image, image.chars().filter(|&c| c=='\n').count()+1);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        machine.send_input(*hull.get(&pos).unwrap_or(&0));
        match machine.run_until(2) {
            Some(output) => {
                assert!(output[0] == 0 || output[0] == 1);
                hull.insert(pos, output[0]);
                assert!(output[1] == 0 || output[1] == 1);
                dir = dir.rotate(output[1] == 1);
                pos += dir.vec();
            },
            None => break,
        }

    }
    if cfg!(debug_assertions) && print_progress {
        print!("\u{001B}[?25h"); // restore cursor
    }
    hull
}

fn render(painted: &HashMap<Point, i64>) -> String {
    render_debug(painted,Point::bounding_box(painted.keys().cloned()).expect("No points"))
}

fn render_debug(painted: &HashMap<Point, i64>, bounds: (Point, Point)) -> String {
    let mut out = String::new();
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
enum Dir { UP, DOWN, LEFT, RIGHT }

impl Dir {
    fn rotate(&self, rotate_right: bool) -> Dir {
        use Dir::*;
        match self {
            UP => { if rotate_right { RIGHT } else { LEFT } },
            DOWN => { if rotate_right { LEFT } else { RIGHT } },
            LEFT => { if rotate_right { UP } else { DOWN } },
            RIGHT => { if rotate_right { DOWN } else { UP } },
        }
    }

    fn vec(&self) -> Vector {
        use Dir::*;
        match self {
            UP => vector(0, -1),
            DOWN => vector(0, 1),
            LEFT => vector(-1, 0),
            RIGHT => vector(1, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute() {
        let hull = paint(true, false);
        let bounding_box = Point::bounding_box(hull.keys().cloned()).expect("No points");
        assert_eq!(bounding_box, (point(0, 0), point(42, 5)));
    }
}
