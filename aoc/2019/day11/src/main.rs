use intcode::{Intcode, csv_to_vec};
use std::collections::HashMap;

struct Robot {
    cpu: Intcode,
    orientation: Orientation,
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}

impl Color {
    fn from(n: i64) -> Self {
        match n {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        }
    }

    fn as_int(&self) -> i64 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn from(n: i64) -> Self {
        match n {
            0 => Turn::Left,
            1 => Turn::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn turn(&self, t: Turn) -> Self {
        match self {
            Orientation::Up => match t {
                Turn::Left => Orientation::Left,
                Turn::Right => Orientation::Right,
            },
            Orientation::Down => match t {
                Turn::Left => Orientation::Right,
                Turn::Right => Orientation::Left,
            },
            Orientation::Left => match t {
                Turn::Left => Orientation::Down,
                Turn::Right => Orientation::Up,
            },
            Orientation::Right => match t {
                Turn::Left => Orientation::Up,
                Turn::Right => Orientation::Down,
            },
        }
    }
}

impl Robot {
    fn new() -> Self {
        let prog = std::fs::read_to_string("program.txt").expect("couldn't read program");
        Self {
            cpu: Intcode::new(csv_to_vec(prog)),
            orientation: Orientation::Up,
            x: 0,
            y: 0,
        }
    }

    fn step(&mut self, panel_color: Color) -> Color {
        // println!("({},{}) {:?}, color={:?}", self.x, self.y, self.orientation, panel_color);
        self.cpu.input.push_back(panel_color.as_int());
        self.cpu.run();
        assert!(self.cpu.output.len() == 2);

        let new_color = self.cpu.output.pop_front().unwrap();
        let turn = self.cpu.output.pop_front().unwrap();
        // println!("\tnew_color={:?}, turn={:?}", Color::from(new_color), Turn::from(turn));

        self.update_pos(Turn::from(turn));

        Color::from(new_color)
    }

    fn update_pos(&mut self, turn: Turn) {
        self.orientation = self.orientation.turn(turn);
        match self.orientation {
            Orientation::Up => self.y += 1,
            Orientation::Down => self.y -= 1,
            Orientation::Left => self.x -= 1,
            Orientation::Right => self.x += 1,
        }
    }

    fn done(&self) -> bool {
        self.cpu.is_halted()
    }
}

struct Grid {
    robot: Robot,
    traversed: HashMap<(i64, i64), Color>,
}

impl Grid {
    fn new() -> Self {
        Self {
            robot: Robot::new(),
            traversed: HashMap::new(),
        }
    }

    fn run(&mut self) {
        while !self.robot.done() {
            let pos = (self.robot.x, self.robot.y);
            let color = *self.traversed.get(&pos).unwrap_or(&Color::Black);

            let new_color = self.robot.step(color);
            self.traversed.insert(pos, new_color);
        }
    }

    fn render(&self) {
        let min_x = self.traversed.keys().min_by_key(|k| k.0).unwrap().0;
        let max_x = self.traversed.keys().max_by_key(|k| k.0).unwrap().0;

        let min_y = self.traversed.keys().min_by_key(|k| k.1).unwrap().1;
        let max_y = self.traversed.keys().max_by_key(|k| k.1).unwrap().1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let color = self.traversed.get(&(x, y)).unwrap_or(&Color::Black);
                match color {
                    Color::White => print!("#"),
                    Color::Black => print!(" "),
                }
            }
            print!("\n");
        }
    }
}

fn main() {
    {
        let mut grid = Grid::new();
        grid.run();
        println!("part 1: {:?}", grid.traversed.len());
    }

    {
        let mut grid = Grid::new();
        grid.traversed.insert((0, 0), Color::White);
        grid.run();
        grid.render();
    }
}
