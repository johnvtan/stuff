use intcode::{Intcode, csv_to_vec};

fn main() {
    let prog = csv_to_vec(std::fs::read_to_string("input.txt").expect("Could not read input"));
    let mut intcode = Intcode::new(prog);
    intcode.input.push_front(/* part */ 2);
    intcode.run();

    while let Some(output) = intcode.output.pop_front() {
        println!("{}", output);
    }
}
