fn main() {
    let input = 402328..864247;
    let mut possibilities = 0;
    for i in input {
        if ok(i) {
            possibilities += 1;
        }
    }
    println!("{}", possibilities);
}

fn ok(number: i32) -> bool {
    let digits: Vec<u32> = number
        .to_string()
        .chars()
        .map(|letter| letter.to_digit(10).unwrap())
        .collect();
    if digits.len() != 6 {
        return false;
    }
    let mut min = 0;
    let mut same = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for digit in digits.iter() {
        if *digit < min {
            return false;
        }
        if *digit == min {
            same[*digit as usize] += 1;
        }
        min = *digit;
    }
    let mut pairs = 0;
    for number in same.iter() {
        if *number == 1 {
            pairs += 1;
        }
    }
    return pairs > 0;
}
