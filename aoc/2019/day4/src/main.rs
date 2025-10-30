fn is_valid_password(mut candidate: usize) -> bool {
    let mut has_double = false;
    let mut last_digit = 10;
    let mut digit_count = 0;
    loop {
        let curr = (candidate % 10) as usize;
        if curr == last_digit {
            digit_count += 1;
            // println!("curr: {}, last_digit: {}, digit_count: {}, has_double: {}", curr, last_digit, digit_count, has_double);
        } else {
            has_double = has_double || (digit_count == 2);
            digit_count = 1;
            // println!("curr: {}, last_digit: {}, digit_count: {}, has_double: {}", curr, last_digit, digit_count, has_double);
            if curr > last_digit {
                return false;
            }
        }

        if candidate == 0 {
            break;
        }

        last_digit = curr;
        candidate /= 10;
    }

    has_double
}

fn main() {
    // println!("{}", is_valid_password(111111));
    // println!("{}", is_valid_password(223450));
    // println!("{}", is_valid_password(1233789));
    // println!("{}", is_valid_password(123444));
    // println!("{}", is_valid_password(11122));
    // println!("{}", is_valid_password(22111));
    // println!("{}", is_valid_password(11222));

    println!("{}", (347312..805915).filter(|n| is_valid_password(*n as usize)).count());
}
