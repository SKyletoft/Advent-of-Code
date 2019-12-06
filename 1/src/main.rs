fn main() {
    let mass_string = include_str!("input.txt");
    let sum: i32 = mass_string
        .lines()
        .map(|line| fuel(line.parse::<i32>().unwrap()))
        .sum();
    let recursive_sum: i32 = mass_string
        .lines()
        .map(|line| recursive_fuel(line.parse::<i32>().unwrap()))
        .sum();
    println!("{}\n{}\n{}", sum, recursive_sum, sum + recursive_sum);
}

fn fuel(mass: i32) -> i32 {
    let req = (mass / 3) - 2;
    if req <= 0 {
        return 0;
    }
    req
}

fn recursive_fuel(mass: i32) -> i32 {
    let mut last = fuel(mass);
    let mut total = 0;
    loop {
        let next = fuel(last);
        total += next;
        last = next;
        if next == 0 {
            break;
        }
    }
    total
}
