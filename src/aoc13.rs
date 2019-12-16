use crate::intcode::Machine;
use std::collections::HashMap;
use crate::euclid::{Point, point};
use std::fmt;

pub fn advent() {
    let mut machine = Machine::from_file("data/day13.txt");
    let mut state = State::new();
    machine.run();
    state.update(&machine.read_output());
    let blocks = state.find_tiles(Tile::BLOCK).len();

    let score = play_game(true);
    println!("Initial Blocks: {}\nFinal Score: {}", blocks, score);
}

fn play_game(print_display: bool) -> u32 {
    let mut machine = Machine::from_file("data/day13.txt");
    machine.set_state(0, 2);
    let mut state = State::new();
    if cfg!(debug_assertions) && print_display {
        print!("\u{001B}[?25l"); // hide cursor
    }
    loop {
        machine.run_until_input();
        let output = machine.read_output();
        if output.is_empty() { break; }
        state.update(&output);
        machine.send_input(state.find_move());
        if cfg!(debug_assertions) && print_display {
            let display = state.to_string();
            println!("{}\u{001B}[{}A", display, display.chars().filter(|&c| c == '\n').count() + 1);
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
    if cfg!(debug_assertions) && print_display {
        println!("{}", state);
        print!("\u{001B}[?25h"); // restore cursor
    }
    state.score
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

impl Tile {
    fn from_id(id: i64) -> Tile {
        use Tile::*;
        match id {
            0 => EMPTY,
            1 => WALL,
            2 => BLOCK,
            3 => PADDLE,
            4 => BALL,
            _ => panic!(),
        }
    }

    fn char(&self) -> char {
        use Tile::*;
        match self {
            EMPTY => ' ',
            WALL => '█',
            BLOCK => '#',
            PADDLE => '▔',
            BALL => 'O',
        }
    }
}

struct State {
    steps: u32,
    tiles: HashMap<Point, Tile>,
    score: u32,
}

impl State {
    const SCORE_COORD: Point = point(-1, 0);

    fn new() -> State { State { steps: 0, tiles: HashMap::new(), score: 0 } }

    fn update(&mut self, output: &[i64]) {
        assert_eq!(output.len() % 3, 0);
        let mut updates = HashMap::new();
        let mut score = None;

        for i in (0..output.len()).step_by(3) {
            let coord = point(output[i] as i32, output[i+1] as i32);
            if coord == State::SCORE_COORD {
                score = Some(output[i + 2] as u32);
            } else {
                let tile = Tile::from_id(output[i + 2]);
                updates.insert(coord, tile);
            }
        }

        self.steps += 1;
        self.tiles.extend(updates);
        if let Some(score) = score {
            self.score = score;
        }
    }

    fn find_tiles(&self, tile: Tile) -> Vec<Point> {
        self.tiles.iter().filter(|(_, &v)| v == tile).map(|(&k, _)| k).collect()
    }

    fn find_move(&self) -> i64 {
        let ball = self.find_tiles(Tile::BALL);
        assert_eq!(ball.len(), 1);
        let ball = ball[0];
        let paddle = self.find_tiles(Tile::PADDLE);
        assert_eq!(paddle.len(), 1);
        let paddle = paddle[0];
        (ball - paddle).x.signum() as i64
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        let bounds = Point::bounding_box(self.tiles.keys().cloned()).expect("No points");
        for y in bounds.0.y..bounds.1.y+1 {
            for x in bounds.0.x..bounds.1.x + 1 {
                let coord = point(x, y);
                let tile = self.tiles.get(&coord).expect("Unknown tile");
                out.push(tile.char());
            }
            out.push('\n');
        }
        write!(f, "{}Step: {:<5}  Blocks Left: {:<5}  Score: {:<8}",
               out, self.steps, self.find_tiles(Tile::BLOCK).len(), self.score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic change-detector; problem statement doesn't offer any meaningful test cases
    #[test] fn check_score() { assert_eq!(play_game(false), 11140); }

    #[test]
    fn state_updates() {
        let mut state = State::new();
        state.update(&[1,2,3,6,5,4]);
        state.update(&[-1,0,12345]);
        let tiles: HashMap<_, _> =
            vec!((point(1,2), Tile::PADDLE), (point(6,5), Tile::BALL)).iter().cloned().collect();
        assert_eq!(state.tiles, tiles);
        assert_eq!(state.score, 12345);
    }
}