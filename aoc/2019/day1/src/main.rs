fn calculate_fuel_requirement(mass: isize) -> isize {
    std::cmp::max((mass / 3) - 2, 0)
}

// part 1
fn sum_fuel_requirements(input: String) -> isize {
    input.lines()
        .map(|line| line.parse::<isize>())
        .map(|mass| calculate_fuel_requirement(mass.unwrap()))
        .sum()
}

// part 2
fn sum_fuel_requirements2(input: String) -> isize {
    input.lines()
        .map(|line| line.parse::<isize>())
        .map(|mass| {
            let mut total_fuel = 0;
            let mut next_mass = mass.unwrap();
            loop {
                next_mass = calculate_fuel_requirement(next_mass);
                if next_mass == 0 {
                    break;
                }
                total_fuel += next_mass;
            }
            total_fuel
        })
        .sum()
}

fn main() {
    println!("{}", sum_fuel_requirements2(std::fs::read_to_string("input.txt").unwrap()));
}
