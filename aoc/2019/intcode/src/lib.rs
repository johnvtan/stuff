use std::io::{BufRead, Write};

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

pub struct Intcode<R: BufRead, W: Write> {
    pc: usize,
    memory: Vec<isize>,
    input: R,
    output: W,
}

impl<R: BufRead, W: Write> Intcode<R, W> {
    pub fn new(program: Vec<isize>, input: R, output: W) -> Self {
        Intcode {
            pc: 0,
            memory: program,
            input,
            output,
        }
    }

    pub fn read_memory(&self, addr: usize) -> isize {
        self.memory[addr]
    }

    pub fn write_memory(&mut self, addr: usize, val: isize) {
        self.memory[addr] = val
    }

    fn parameter(&self, offset: usize, opcode: &Opcode) -> isize {
        let param = self.memory[self.pc + offset];
        match opcode.mode(offset) {
            Mode::Immediate => {
                debug_println!("got immediate {}", param);
                param
            }
            Mode::Position => {
                debug_println!("got position: addr {} val {}", param, self.memory[param as usize]);
                self.memory[param as usize]
            }
        }
    }

    /// Runs the program until it halts (executs opcode 99).
    pub fn run(&mut self) {
        loop {
            debug_println!("pc = [{}], mem = [{:?}]", self.pc, self.memory);
            let opcode = Opcode::new(self.read_memory(self.pc as usize));
            match opcode.operation() {
                Operation::Add => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;

                    //println/g!("mem[{}] = mem[{}] + mem[{}]", dst_addr, p1, p2);
                    self.memory[dst_addr] = p1 + p2;
                    self.pc += 4;
                }
                Operation::Mul => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;

                    //println/g!("mem[{}] = mem[{}] + mem[{}]", dst_addr, p1, p2);
                    self.memory[dst_addr] = p1 * p2;
                    self.pc += 4;
                }
                Operation::Input => {
                    let mut input_line = String::new();
                    self.input
                        .read_line(&mut input_line)
                        .expect("Failed to read line");

                    //println/g!("");

                    let input: isize = input_line.trim().parse().expect("Input not an integer");
                    //println/g!("input: {}", input);
                    let dst_addr = self.memory[self.pc + 1] as usize;
                    self.memory[dst_addr] = input;
                    self.pc += 2;
                }
                Operation::Output => {
                    let p1 = self.parameter(1, &opcode);
                    write!(&mut self.output, "OUTPUT> {}\n", p1).expect("could not write");
                    self.pc += 2;
                }
                Operation::JumpIfTrue => {
                    let cond = self.parameter(1, &opcode);
                    let jmp_addr = self.parameter(2, &opcode);
                    if cond != 0 {
                        self.pc = jmp_addr as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::JumpIfFalse => {
                    let cond = self.parameter(1, &opcode);
                    let jmp_addr = self.parameter(2, &opcode);
                    if cond == 0 {
                        self.pc = jmp_addr as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::LessThan => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;
                    self.memory[dst_addr] = if p1 < p2 { 1 } else { 0 };
                    self.pc += 4;
                }
                Operation::Equals => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;
                    self.memory[dst_addr] = if p1 == p2 { 1 } else { 0 };
                    self.pc += 4;
                }
                Operation::Halt => break,
            }
        }
    }
}


pub fn csv_to_vec(input: String) -> Vec<isize> {
    input
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(PartialEq, Debug)]
enum Mode {
    Position,
    Immediate,
}

#[derive(PartialEq, Debug)]
struct Opcode {
    orig: isize,
}

impl Opcode {
    fn new(instruction: isize) -> Self {
        Self { orig: instruction }
    }

    fn operation(&self) -> Operation {
        match self.orig % 100 {
            1 => Operation::Add,
            2 => Operation::Mul,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpIfTrue,
            6 => Operation::JumpIfFalse,
            7 => Operation::LessThan,
            8 => Operation::Equals,
            99 => Operation::Halt,
            _ => unreachable!("{}", self.orig % 100),
        }
    }

    fn mode(&self, n: usize) -> Mode {
        assert!(n >= 1);
        let shifted = self.orig / (10 as isize).pow(1 + n as u32);
        match shifted % 10 {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    type TestIntcode = Intcode<Cursor<String>, Vec<u8>>;

    fn intcode_no_input(program: Vec<isize>) -> TestIntcode {
        Intcode::new(program, Cursor::new(String::new()), Vec::new())
    }

    fn intcode_with_input(program: Vec<isize>, input: String) -> TestIntcode {
        Intcode::new(program, Cursor::new(input), Vec::new())
    }

    #[test]
    fn test_csv_to_vec() {
        assert_eq!(csv_to_vec("1,2,3".to_string()), vec![1, 2, 3]);
    }

    fn assert_memory(intcode: &TestIntcode, expected: Vec<isize>) {
        for (addr, val) in expected.iter().enumerate() {
            assert_eq!(intcode.read_memory(addr), *val);
        }
    }

    #[test]
    fn intcode_add() {
        let mut intcode = intcode_no_input(vec![1, 0, 0, 0, 99]);
        intcode.run();
        assert_memory(&intcode, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn intcode_mul() {
        {
            let mut intcode = intcode_no_input(vec![2, 3, 0, 3, 99]);
            intcode.run();
            assert_memory(&intcode, vec![2, 3, 0, 6, 99]);
        }
        {
            let mut intcode = intcode_no_input(vec![2, 4, 4, 5, 99, 0]);
            intcode.run();
            assert_memory(&intcode, vec![2, 4, 4, 5, 99, 9801]);
        }
    }

    #[test]
    fn intcode_add_and_mul() {
        let mut intcode = intcode_no_input(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        intcode.run();
        assert_memory(&intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn intcode_parameters() {
        let mut intcode = intcode_no_input(vec![1002,4,3,4,33]);
        intcode.run();
        assert_memory(&intcode, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn intcode_input() {
        let mut intcode = intcode_with_input(vec![3, 0, 99], "1234\n".to_string());
        intcode.run();
        assert_memory(&intcode, vec![1234, 0, 99]);
    }

    #[test]
    fn intcode_input_output() {
        let mut intcode = intcode_with_input(vec![3, 0, 4, 0, 99], "1234\n".to_string());
        intcode.run();
        assert_memory(&intcode, vec![1234, 0, 4, 0, 99]);
        assert_eq!(String::from_utf8(intcode.output).expect("fail"), "OUTPUT> 1234\n");
    }

    #[test]
    fn opcode_operation() {
        assert_eq!(Opcode { orig: 1 }.operation(), Operation::Add);
    }

    #[test]
    fn opcode_mode() {
        let opcode = Opcode::new(1002);
        assert_eq!(opcode.mode(1), Mode::Position);
        assert_eq!(opcode.mode(2), Mode::Immediate);
        assert_eq!(opcode.mode(3), Mode::Position);
    }
}
