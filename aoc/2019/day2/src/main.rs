use intcode::{Intcode, csv_to_vec};

const input: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,5,23,1,23,9,27,2,27,6,31,1,31,6,35,2,35,9,39,1,6,39,43,2,10,43,47,1,47,9,51,1,51,6,55,1,55,6,59,2,59,10,63,1,6,63,67,2,6,67,71,1,71,5,75,2,13,75,79,1,10,79,83,1,5,83,87,2,87,10,91,1,5,91,95,2,95,6,99,1,99,6,103,2,103,6,107,2,107,9,111,1,111,5,115,1,115,6,119,2,6,119,123,1,5,123,127,1,127,13,131,1,2,131,135,1,135,10,0,99,2,14,0,0";

fn main() {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut intcode = Intcode::new(
                csv_to_vec(input.to_string()),
                std::io::stdin().lock(),
                std::io::stdout(),
            );
            intcode.write_memory(1, noun);
            intcode.write_memory(2, verb);
            intcode.run();
            if intcode.read_memory(0) == 19690720 {
                println!(
                    "noun = {}, verb = {}, output = {}",
                    noun,
                    verb,
                    100 * noun + verb
                );
                return;
            }
        }
    }
}
