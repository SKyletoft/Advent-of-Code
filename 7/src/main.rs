fn main() {
    /*
    let input = include_str!("input.txt");
    let array: Vec<i128> = input
        .split(',')
        .map(|number| number.parse::<i128>().unwrap())
        .collect();
    let mut record = 0;
    let mut record_order = Vec::new();
    let numbers: Vec<i128> = (0..5).collect();
    let orders = all_orders(&numbers);
    for order in orders.iter() {
        let mut second = 0;
        for amp in order.iter() {
            second = machine(vec![*amp, second], &array);
        }
        if second > record {
            record = second;
            record_order = order.clone();
        }
    }
    println!("{}, {:?}", record, record_order);
    */
    let array = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let order = [9, 8, 7, 6, 5];
    let mut A = array.clone();
    let mut B = array.clone();
    let mut C = array.clone();
    let mut D = array.clone();
    let mut E = array.clone();
    
    let mut io_A = vec![9, 0];
    let mut io_B = vec![8];
    let mut io_C = vec![7];
    let mut io_D = vec![6];
    let mut io_E = vec![5];

    let mut pos = [0,0,0,0,0];

    loop {
        let mut done = true;
        if io_A.len() > 0 {
            done = false;
            machine(&mut io_A, &mut A, &mut io_B, &mut pos[0]);
        }
        if io_B.len() > 0 {
            done = false;
            machine(&mut io_B, &mut B, &mut io_C, &mut pos[1]);
        }
        if io_C.len() > 0 {
            done = false;
            machine(&mut io_C, &mut C, &mut io_D, &mut pos[2]);
        }
        if io_D.len() > 0 {
            done = false;
            machine(&mut io_D, &mut D, &mut io_E, &mut pos[3]);
        }
        if io_E.len() > 0 {
            done = false;
            println!("{}", io_E[0]);
            machine(&mut io_E, &mut E, &mut io_A, &mut pos[4]);
        }
        println!("{}, {}, {}, {}, {}", io_A.len(), io_B.len(), io_C.len(), io_D.len(), io_E.len());
        if done {
            break;
        }
    }
}

fn all_orders(values: &[i128]) -> Vec<Vec<i128>> {
    if values.len() == 2 {
        return vec![vec![values[0], values[1]], vec![values[1], values[0]]];
    }
    let mut returns = Vec::new();
    for value in 0..values.len() {
        let mut clone = Vec::from(values);
        clone.remove(value);
        let mut pos = all_orders(&clone);
        for possibility in pos.iter_mut() {
            possibility.push(values[value]);
            returns.push(possibility.clone());
        }
    }
    returns
}

fn machine(input: &mut Vec<i128>, array: &mut [i128], output: &mut Vec<i128>, pos: &mut usize) {
    //let mut array = Vec::from(program);
    let mut input_index = 0;
    loop {
        let opcode = format!("{:05}", array[*pos]);
        let param_one = *array.get(*pos + 1).unwrap_or(&0);
        let param_two = *array.get(*pos + 2).unwrap_or(&0);
        let param_three = *array.get(*pos + 3).unwrap_or(&0);
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
                *pos += 4;
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
                *pos += 4;
            }
            "03" => {
                array[param_one as usize] = *input.get(0).unwrap_or(&0);
                if input.len() > 0 {
                    input.remove(0);
                }
                *pos += 2;
            }
            "04" => {
                output.push(array[param_one as usize]);
                *pos += 2;
                return;
            }
            "05" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                if first != 0 {
                    *pos = second as usize;
                } else {
                    *pos += 3;
                }
            }
            "06" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                if first == 0 {
                    *pos = second as usize;
                } else {
                    *pos += 3;
                }
            }
            "07" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let position = array[*pos + 3] as usize;
                if first < second {
                    array[position] = 1;
                } else {
                    array[position] = 0;
                }
                *pos += 4;
            }
            "08" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let position = array[*pos + 3] as usize;
                if first == second {
                    array[position] = 1;
                } else {
                    array[position] = 0;
                }
                *pos += 4;
            }
            "99" => break,
            _ => panic!(),
        }
    }
    //output
}

fn load(mode_bool: bool, param: i128, array: &[i128]) -> i128 {
    if mode_bool {
        array[param as usize]
    } else {
        param
    }
}
