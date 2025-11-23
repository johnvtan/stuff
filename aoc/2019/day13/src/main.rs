use intcode::*;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl Tile {
    fn from(n: i64) -> Self {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => unreachable!(),
        }
    }

    fn render(&self) -> &str {
        match self {
            Self::Empty => " ",
            Self::Wall => "|",
            Self::Block => "#",
            Self::HorizontalPaddle => "_",
            Self::Ball => "o",
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

struct Game {
    cpu: Intcode,
    tiles: HashMap<Coord, Tile>,
    score: i64,
    ball_x: i64,
    paddle_x: i64,
}

enum JoystickState {
    Neutral,
    Left,
    Right,
}

impl JoystickState {
    fn to(&self) -> i64 {
        match self {
            JoystickState::Neutral => 0,
            JoystickState::Left => -1,
            JoystickState::Right => 1,
        }
    }
}

impl Game {
    fn new() -> Self {
        let prog = std::fs::read_to_string("program.txt").expect("couldn't read program");
        let cpu = Intcode::new(csv_to_vec(prog));
        Self {
            cpu,
            tiles: HashMap::new(),
            score: 0,
            ball_x: 0,
            paddle_x: 0,
        }
    }

    fn advance(&mut self, joystick: JoystickState) {
        self.cpu.input.push_back(joystick.to());
        self.cpu.run();

        // update screen/score from output
        while self.cpu.output.len() > 0 {
            assert!(self.cpu.output.len() >= 3);
            let x = self.cpu.output.pop_front().unwrap();
            let y = self.cpu.output.pop_front().unwrap();
            if x == -1 && y == 0 {
                self.score = self.cpu.output.pop_front().unwrap();
            } else {
                let tile = Tile::from(self.cpu.output.pop_front().unwrap());
                match tile {
                    Tile::Ball => self.ball_x = x,
                    Tile::HorizontalPaddle => self.paddle_x = x,
                    _ => {}
                }
                self.tiles.insert(Coord { x, y }, tile);
            }
        }
    }

    fn done(&self) -> bool {
        self.cpu.is_halted()
    }

    fn render(&self) {
        let min_x = self.tiles.keys().min_by_key(|k| k.x).unwrap().x;
        let max_x = self.tiles.keys().max_by_key(|k| k.x).unwrap().x;

        let min_y = self.tiles.keys().min_by_key(|k| k.y).unwrap().y;
        let max_y = self.tiles.keys().max_by_key(|k| k.y).unwrap().y;

        println!("score: {}", self.score);
        for row in min_y..=max_y {
            for col in min_x..=max_x {
                print!(
                    "{}",
                    self.tiles.get(&Coord { x: col, y: row }).unwrap().render()
                );
            }
            print!("\n");
        }
    }
}

use std::io;
fn main() {
    {
        let mut game = Game::new();
        while !game.done() {
            game.advance(JoystickState::Neutral);
        }
        let num_blocks = game
            .tiles
            .values()
            .filter(|tile| **tile == Tile::Block)
            .count();
        println!("part1: {}", num_blocks);
    }
    {
        let mut game = Game::new();
        game.cpu.write_memory(0, 2);
        while !game.done() {
            let js = if game.paddle_x < game.ball_x {
                JoystickState::Right
            } else if game.paddle_x > game.ball_x {
                JoystickState::Left
            } else {
                JoystickState::Neutral
            };
            game.advance(js);
            game.render();
        }
    }
}
