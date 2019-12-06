fn main() {
    let input = include_str!("input.txt");
    let array: Vec<usize> = input
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .collect();
    let goal = 19690720;
    for verb in 0..=99 {
        for noun in 0..=99 {
            let result = machine(noun, verb, &array);
            if goal == result {
                println!("{}, {}", noun, verb);
                break;
            }
        }
    }
}

fn machine(noun: usize, verb: usize, program: &[usize]) -> usize {
    let mut array = Vec::from(program);
    array[1] = noun;
    array[2] = verb;
    let mut pos = 0;
    loop {
        let res = match array[pos] {
            99 => break,
            1 => array[array[pos + 1]] + array[array[pos + 2]],
            2 => array[array[pos + 1]] * array[array[pos + 2]],
            _ => panic!(),
        };
        let pos_decoded = array[pos + 3];
        array[pos_decoded] = res;
        pos += 4;
    }
    array[0]
}
