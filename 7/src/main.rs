fn main() {
    let input = include_str!("input.txt");
    let array: Vec<i32> = input
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    part_one(&array);
}

fn part_two(array: &Vec<i32>, io: &[i32]) -> i32 {
    let mut mem_a = array.clone();
    let mut mem_b = array.clone();
    let mut mem_c = array.clone();
    let mut mem_d = array.clone();
    let mut mem_e = array.clone();
    let mut io_a = vec![io[0], 0];
    let mut io_b = vec![io[1]];
    let mut io_c = vec![io[2]];
    let mut io_d = vec![io[3]];
    let mut io_e = vec![io[4]];

    let mut pos = [0, 0, 0, 0, 0];

    loop {
        //println!("\nA");
        if mem_a[pos[0]] == 99 {
            pos[0] = 0;
            panic!();
        }
        machine(&mut io_a, &mut mem_a, &mut io_b, &mut pos[0]);
        //println!("\nB");
        if mem_b[pos[1]] == 99 {
            pos[1] = 0;
            panic!();
        }
        machine(&mut io_b, &mut mem_b, &mut io_c, &mut pos[1]);
        //println!("\nC");
        if mem_c[pos[2]] == 99 {
            pos[2] = 0;
            panic!();
        }
        machine(&mut io_c, &mut mem_c, &mut io_d, &mut pos[2]);
        //println!("\nD");
        if mem_d[pos[3]] == 99 {
            pos[3] = 0;
            panic!();
        }
        machine(&mut io_d, &mut mem_d, &mut io_e, &mut pos[3]);
        //println!("\nE");
        if mem_e[pos[4]] == 99 {
            pos[4] = 0;
            panic!();
        }
        machine(&mut io_e, &mut mem_e, &mut io_a, &mut pos[4]);
        if mem_e[pos[4]] == 99 {
            //println!("E ended {:?}", io_e);
            break;
        }
    }
    //println!("\n\nFinal IO state:\n{:?} {:?} {:?} {:?} {:?}", io_a, io_b, io_c, io_d, io_e);
    *io_a.get(0).unwrap_or(&0)
}

fn part_one(array: &Vec<i32>) {
    let mut record = 0;
    let mut record_order = Vec::with_capacity(0);
    let numbers: Vec<i32> = (5..10).collect();
    let orders = all_orders(&numbers);
    for order in orders.iter() {
        let second = part_two(array, &order);
        if second > record {
            record = second;
            record_order = order.clone();
        }
    }
    println!("{}, {:?}", record, record_order);
}

fn all_orders(values: &[i32]) -> Vec<Vec<i32>> {
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

fn machine(input: &mut Vec<i32>, array: &mut [i32], output: &mut Vec<i32>, pos: &mut usize) {
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
                /*println!(
                    "add {:04} {:04} ({:04})\t\t-> {}{:04}",
                    first,
                    second,
                    sum,
                    {
                        if mode[2] {
                            ""
                        } else {
                            "*"
                        }
                    },
                    param_three
                );*/
                continue;
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
                /*println!(
                    "mul {:04} {:04} ({:04})\t\t-> {}{:04}",
                    first,
                    second,
                    sum,
                    {
                        if mode[2] {
                            ""
                        } else {
                            "*"
                        }
                    },
                    param_three
                );*/
                continue;
            }
            "03" => {
                if input.len() > 0 {
                    array[param_one as usize] = input[0];
                    input.remove(0);
                    *pos += 2;
                    //println!("read {:04}\t\t\t-> {:04}", array[param_one as usize], param_one);
                    continue;
                } else {
                    //println!("\tfailed to read, no inputs buffered");
                    return;
                }
            }
            "04" => {
                output.push(array[param_one as usize]);
                *pos += 2;
                //println!("out {:04}", array[param_one as usize]);
                continue;
            }
            "05" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                if first != 0 {
                    *pos = second as usize;
                } else {
                    *pos += 3;
                }
                //println!("is0 {:04}\t\t\t-> {:04}", first, second);
                continue;
            }
            "06" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                if first == 0 {
                    *pos = second as usize;
                } else {
                    *pos += 3;
                }
                //println!("not0 {:04}\t\t-> {:04}", first, second);
                continue;
            }
            "07" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let position;
                if mode[2] {
                    position = array[*pos + 3] as usize;
                } else {
                    let position_tmp = array[*pos + 3] as usize;
                    position = array[position_tmp] as usize;
                }
                if first < second {
                    array[position] = 1;
                } else {
                    array[position] = 0;
                }
                *pos += 4;
                //println!("less {:04} {:04}\t\t\t-> {:04}", first, second, position);
                continue;
            }
            "08" => {
                let first = load(mode[0], param_one, &array);
                let second = load(mode[1], param_two, &array);
                let position;
                if mode[2] {
                    position = array[*pos + 3] as usize;
                } else {
                    let position_tmp = array[*pos + 3] as usize;
                    position = array[position_tmp] as usize;
                }
                if first == second {
                    array[position] = 1;
                } else {
                    array[position] = 0;
                }
                *pos += 4;
                //println!("eq {:04} {:04}\t\t\t-> {:04}", first, second, position);
                continue;
            }
            "99" => {
                //println!("END!");
                return;
            }
            _ => panic!(),
        }
    }
    //output
}

fn load(mode_bool: bool, param: i32, array: &[i32]) -> i32 {
    if mode_bool {
        array[param as usize]
    } else {
        param
    }
}
