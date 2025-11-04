use intcode::{Intcode, csv_to_vec};
use itertools::Itertools;
use std::collections::VecDeque;

const PROGRAM: &str = "3,8,1001,8,10,8,105,1,0,0,21,38,55,64,81,106,187,268,349,430,99999,3,9,101,2,9,9,1002,9,2,9,101,5,9,9,4,9,99,3,9,102,2,9,9,101,3,9,9,1002,9,4,9,4,9,99,3,9,102,2,9,9,4,9,99,3,9,1002,9,5,9,1001,9,4,9,102,4,9,9,4,9,99,3,9,102,2,9,9,1001,9,5,9,102,3,9,9,1001,9,4,9,102,5,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99";

// const PROGRAM: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
fn part1() {
    let program = csv_to_vec(PROGRAM.to_string());
    let settings = vec![0, 1, 2, 3, 4];
    let mut max_signal = 0;
    for perm in settings.iter().permutations(settings.len()).unique() {
        let mut input_signal = 0;
        for i in 0..perm.len() {
            let phase_setting = perm[i].clone();

            let mut intcode = Intcode::new(program.clone());
            intcode.input = VecDeque::from(vec![phase_setting, input_signal]);
            intcode.run();

            let output = intcode.output.pop_front().unwrap();
            if i == perm.len() - 1 {
                max_signal = std::cmp::max(max_signal, output);
            } else {
                input_signal = output;
            }
        }
    }

    println!("part 1: max_signal: {}", max_signal);
}

fn part2() {
    let program = csv_to_vec(PROGRAM.to_string());
    let settings = vec![5, 6, 7, 8, 9];
    let mut max_signal = 0;

    for perm in settings.iter().permutations(settings.len()).unique() {
        // Create new computers for each setting permuation with the phase setting as the first
        // input.
        let mut computers = vec![];
        let mut input_signal = 0;
        let mut last_thruster_signal = 0;

        // Do first iteration on creation so that we can set phase setting
        for phase_setting in perm.iter() {
            let mut computer = Intcode::new(program.clone());
            computer.input = VecDeque::from(vec![**phase_setting, input_signal.clone()]);

            computer.run();
            input_signal = computer.output.pop_front().unwrap();
            computers.push(computer);
        }

        'outer: loop {
            for i in 0..computers.len() {
                let computer = &mut computers[i];
                if computer.is_halted() {
                    // can't propagate any more signals on this run because the computer is halted.
                    break 'outer;
                }

                computer.input.push_back(input_signal);
                computer.output = VecDeque::new();
                computer.run();

                let output = computer.output.pop_front().unwrap();

                input_signal = output;
                if i == 4 {
                    last_thruster_signal = output;
                }
            }
        }

        max_signal = std::cmp::max(max_signal, last_thruster_signal);
    }

    println!("part 2: max signal {}", max_signal);
}

fn main() {
    part1();
    part2();
}
