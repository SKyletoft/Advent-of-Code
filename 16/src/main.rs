use rayon::prelude::*;

fn main() {
    let input = //"03036732577212944063491565474664"
        include_str!("input.txt")
        .chars()
        .map(|digit| digit as i64 - 48)
        .collect::<Vec<i64>>();
    let base_pattern = [0, 1, 0, -1];
    
    let result = part_two(&input);
    println!("{:?}", result);
}

fn part_one(original: &[i64], pattern: &[i64]) -> Vec<i64> {
    let mut working_vec = Vec::from(original);
    for _ in 0..100 {
        working_vec = phase(&working_vec, pattern);
    }
    working_vec
}

fn part_two(original: &[i64]) -> Vec<i64> {
    let offset = (0..7)
        .map(|index| (10 as i64).pow(6 - index) * original[index as usize])
        .sum::<i64>();
    let mut working_vec = Vec::with_capacity(original.len() * 10000);
    for _ in 0..10000 {
        for value in original.iter() {
            working_vec.push(*value);
        }
    }
    println!("created initial vec");
    working_vec = Vec::from(&working_vec[(offset as usize)..working_vec.len()]);
    for i in 0..100 {
        working_vec = phase_by_reddit(&working_vec);
        println!("loop {}/100 done", i);
    }
    Vec::from(&working_vec[0..8])
}

fn phase(input: &[i64], pattern: &[i64]) -> Vec<i64> {
    (0..input.len())
        .into_par_iter()
        .map(|line| {
            let mut sum = 0;
            for index in line..input.len() {
                sum += pattern[((index + 1) / (line + 1)) % pattern.len()] * input[index];
            }
            sum %= 10;
            sum.abs()
        })
        .collect::<Vec<i64>>()
}

fn phase_by_reddit(input: &[i64]) -> Vec<i64> {
    (0..input.len())
        .into_par_iter()
        .map(|i| {
            input[i..input.len()].iter().map(|i| *i).sum::<i64>() % 10
        })
        .collect::<Vec<i64>>()
}
