fn main() {
    let input = include_str!("input.txt");
    let array: Vec<i32> = input
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    let output = machine(5, &array);
    println!("{}", output);
}

fn machine(input: i32, program: &[i32]) -> i32 {
    let mut array = Vec::from(program);
    let mut pos = 0;
    let mut output = 0;
    loop {
        let opcode = format!("{:05}", array[pos]);
        let param_one = *array.get(pos + 1).unwrap_or(&0);
        let param_two = *array.get(pos + 2).unwrap_or(&0);
        let param_three = *array.get(pos + 3).unwrap_or(&0);
        let mode = [
            {
                match &opcode[2..3] {
                    "0" => true,
                    "1" => false,
                    _ => panic!(),
                }
            },
            {
                match &opcode[1..2] {
                    "0" => true,
                    "1" => false,
                    _ => panic!(),
                }
            },
            {
                match &opcode[0..1] {
                    "0" => true,
                    "1" => false,
                    _ => panic!(),
                }
            },
        ];
        let op = &opcode[3..5];
        match op {
            "01" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let sum = first + second;
                if mode[2] {
                    array[param_three as usize] = sum;
                } else {
                    let position = array[param_three as usize] as usize;
                    array[position] = sum;
                }
                pos += 4;
            }
            "02" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let sum = first * second;
                if mode[2] {
                    array[param_three as usize] = sum;
                } else {
                    let position = array[param_three as usize] as usize;
                    array[position] = sum;
                }
                pos += 4;
            }
            "03" => {
                array[param_one as usize] = input;
                pos += 2
            }
            "04" => {
                output = array[param_one as usize];
                pos += 2
            }
            "05" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                if first != 0 {
                    pos = second as usize;
                } else {
                    pos += 3;
                }
            }
            "06" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                if first == 0 {
                    pos = second as usize;
                } else {
                    pos += 3;
                }
            }
            "07" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let position = array[pos + 3] as usize;
                if first < second {
                    array[position] = 1;
                } else {
                    array[position] = 0;
                }
                pos += 4;
            }
            "08" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let position = array[pos + 3] as usize;
                if first == second {
                    array[position] = 1;
                } else {
                    array[position] = 0;
                }
                pos += 4;
            }
            "99" => break,
            _ => panic!(),
        }
    }
    output
}

fn load(mode_bool: bool, param: i32, array: &[i32]) -> i32 {
    if mode_bool {
        array[param as usize]
    } else {
        param
    }
}
