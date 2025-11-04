use std::collections::VecDeque;

macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

#[derive(Debug, PartialEq)]
enum ExecutionState {
    WaitForInput,
    Halted,
}

pub struct Intcode {
    pc: usize,
    memory: Vec<isize>,
    pub input: VecDeque<isize>,
    pub output: VecDeque<isize>,
    execution_state: Option<ExecutionState>,
}


impl Intcode {
    pub fn new(program: Vec<isize>) -> Self {
        Intcode {
            pc: 0,
            memory: program,
            input: VecDeque::new(),
            output: VecDeque::new(),
            execution_state: None,
        }
    }

    pub fn read_memory(&self, addr: usize) -> isize {
        self.memory[addr]
    }

    pub fn write_memory(&mut self, addr: usize, val: isize) {
        self.memory[addr] = val
    }

    pub fn is_halted(&self) -> bool {
        self.execution_state.as_ref().map_or(false, |state| match state {
            ExecutionState::Halted => true,
            _ => false,
        })
    }

    pub fn is_waiting(&self) -> bool {
        self.execution_state.as_ref().map_or(false, |state| match state {
            ExecutionState::WaitForInput => true,
            _ => false,
        })
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
        assert!(!self.is_halted());
        loop {
            debug_println!("pc = [{}], mem = [{:?}]", self.pc, self.memory);
            let opcode = Opcode::new(self.read_memory(self.pc as usize));
            match opcode.operation() {
                Operation::Add => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;

                    debug_println!("mem[{}] = mem[{}] + mem[{}]", dst_addr, p1, p2);
                    self.memory[dst_addr] = p1 + p2;
                    self.pc += 4;
                }
                Operation::Mul => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;

                    debug_println!("mem[{}] = mem[{}] + mem[{}]", dst_addr, p1, p2);
                    self.memory[dst_addr] = p1 * p2;
                    self.pc += 4;
                }
                Operation::Input => {
                    if self.input.is_empty() {
                        // assert!(!self.is_waiting());
                        debug_println!("Waiting for next input");
                        self.execution_state.replace(ExecutionState::WaitForInput);
                        return;
                    }
                    let input = self.input.pop_front().unwrap();
                    debug_println!("input: {}", input);
                    let dst_addr = self.memory[self.pc + 1] as usize;
                    self.memory[dst_addr] = input;
                    self.pc += 2;
                }
                Operation::Output => {
                    let p1 = self.parameter(1, &opcode);
                    debug_println!("output: {}", p1);
                    self.output.push_back(p1);
                    self.pc += 2;
                }
                Operation::JumpIfTrue => {
                    let cond = self.parameter(1, &opcode);
                    let jmp_addr = self.parameter(2, &opcode);
                    debug_println!("jump if true: cond {} target {}", cond, jmp_addr);
                    if cond != 0 {
                        self.pc = jmp_addr as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Operation::JumpIfFalse => {
                    let cond = self.parameter(1, &opcode);
                    let jmp_addr = self.parameter(2, &opcode);
                    debug_println!("jump if false: cond {} target {}", cond, jmp_addr);
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
                    debug_println!("mem[{}] = {} < {}", dst_addr, p1, p2);
                    self.memory[dst_addr] = if p1 < p2 { 1 } else { 0 };
                    self.pc += 4;
                }
                Operation::Equals => {
                    let p1 = self.parameter(1, &opcode);
                    let p2 = self.parameter(2, &opcode);
                    let dst_addr = self.memory[self.pc + 3] as usize;
                    debug_println!("mem[{}] = {} == {}", dst_addr, p1, p2);
                    self.memory[dst_addr] = if p1 == p2 { 1 } else { 0 };
                    self.pc += 4;
                }
                Operation::Halt => {
                    debug_println!("halt");
                    self.execution_state.replace(ExecutionState::Halted);
                    return;
                }
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

    #[test]
    fn test_csv_to_vec() {
        assert_eq!(csv_to_vec("1,2,3".to_string()), vec![1, 2, 3]);
    }

    fn assert_memory(intcode: &Intcode, expected: Vec<isize>) {
        for (addr, val) in expected.iter().enumerate() {
            assert_eq!(intcode.read_memory(addr), *val);
        }
    }

    #[test]
    fn intcode_add() {
        let mut intcode = Intcode::new(vec![1, 0, 0, 0, 99]);
        intcode.run();
        assert_memory(&intcode, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn intcode_mul() {
        {
            let mut intcode = Intcode::new(vec![2, 3, 0, 3, 99]);
            intcode.run();
            assert_memory(&intcode, vec![2, 3, 0, 6, 99]);
        }
        {
            let mut intcode = Intcode::new(vec![2, 4, 4, 5, 99, 0]);
            intcode.run();
            assert_memory(&intcode, vec![2, 4, 4, 5, 99, 9801]);
        }
    }

    #[test]
    fn intcode_add_and_mul() {
        let mut intcode = Intcode::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        intcode.run();
        assert_memory(&intcode, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn intcode_parameters() {
        let mut intcode = Intcode::new(vec![1002,4,3,4,33]);
        intcode.run();
        assert_memory(&intcode, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn intcode_input() {
        let mut intcode = Intcode::new(vec![3, 0, 99]);
        intcode.input.push_back(1234);
        intcode.run();
        assert_memory(&intcode, vec![1234, 0, 99]);
    }

    #[test]
    fn intcode_input_output() {
        let mut intcode = Intcode::new(vec![3, 0, 4, 0, 99]);
        intcode.input.push_back(1234);
        intcode.run();
        assert_memory(&intcode, vec![1234, 0, 4, 0, 99]);
        assert_eq!(intcode.output.pop_front().unwrap(), 1234);
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
