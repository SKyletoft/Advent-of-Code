use std::collections::HashMap;

fn main() {
    for _ in 0..10000 {
        let input = include_str!("input7.txt");
        let array: Vec<i64> = input
            .split(',')
            .map(|number| number.parse::<i64>().unwrap())
            .collect();
        let mut program = HashMap::new();
        for (index, data) in array.iter().enumerate() {
            program.insert(index, *data);
        }
        part_one(&program);
    }
}

fn part_one(program: &HashMap<usize, i64>) {
    let mut out = Vec::new();
    machine(&mut vec![1], &mut program.clone(), &mut out, &mut 0, &mut 0);
    //println!("{:?}", out);
}

fn machine(
    input: &mut Vec<i64>,
    program: &mut HashMap<usize, i64>,
    output: &mut Vec<i64>,
    pos: &mut usize,
    rel_base: &mut usize,
) {
    loop {
        let opcode = format!("{:05}", program.get(pos).unwrap_or(&0));
        let param_one = *program.get(&(*pos + 1)).unwrap_or(&0);
        let param_two = *program.get(&(*pos + 2)).unwrap_or(&0);
        let param_three = *program.get(&(*pos + 3)).unwrap_or(&0);
        let mode = [
            { *(&opcode[2..3].parse::<i64>().unwrap_or(0)) },
            { *(&opcode[1..2].parse::<i64>().unwrap_or(0)) },
            { *(&opcode[0..1].parse::<i64>().unwrap_or(0)) },
        ];
        let op = &opcode[3..5];
        
        let mut array = Vec::new();
        for (index, value) in program.iter() {
            while array.len() <= *index {
                array.push(0);
            }
            array[*index] = *value;
        }

        /*println!("mem: {:?}, in: {:?}, out: {:?}, instruction_pointer: {}, relative_base: {}",
            array, input, output, *pos, *rel_base
        );*/

        //print!("\n{:03}: ", *pos);

        match op {
            "01" => {
                let first = load(mode[0], param_one, program, rel_base);
                let second = load(mode[1], param_two, program, rel_base);
                let sum = first + second;
                position_write(mode[2], program, param_three, sum, *rel_base);
                *pos += 4;
                //println!("add {}, {} => {}", first, second, param_three);
                continue;
            }
            "02" => {
                let first = load(mode[0], param_one, program, rel_base);
                let second = load(mode[1], param_two, program, rel_base);
                let sum = first * second;
                position_write(mode[2], program, param_three, sum, *rel_base);
                *pos += 4;
                //println!("mul {}, {} => {}", first, second, param_three);
                continue;
            }
            "03" => {
                if input.len() > 0 {
                    position_write(mode[0], program, 0, input[0], *rel_base);
                    //println!("in {}", input[0]);
                    input.remove(0);
                    *pos += 2;
                    continue;
                } else {
                    panic!();
                }
            }
            "04" => {
                output.push(load(mode[0], param_one, program, rel_base));
                //println!("\t\t\tOUT: {}",output[output.len() - 1]);
                *pos += 2;
                continue;
            }
            "05" => {
                let first = load(mode[0], param_one, program, rel_base);
                let second = load(mode[1], param_two, program, rel_base);
                if first != 0 {
                    //println!("jmp if true {} {} (JUMPED)", first, second);
                    *pos = second as usize;
                } else {
                    //println!("jmp if true {} {} (DIDN'T JUMP)", first, second);
                    *pos += 3;
                }
                continue;
            }
            "06" => {
                let first = load(mode[0], param_one, program, rel_base);
                let second = load(mode[1], param_two, program, rel_base);
                if first == 0 {
                    //println!("jmp if false {} {} (JUMPED)", first, second);
                    *pos = second as usize;
                } else {
                    //println!("jmp if false {} {} (DIDN'T JUMP)", first, second);
                    *pos += 3;
                }
                continue;
            }
            "07" => {
                let first = load(mode[0], param_one, program, rel_base);
                let second = load(mode[1], param_two, program, rel_base);
                let position = position_load(mode[2], program, *pos, *rel_base);
                //println!("cmp {} {} ({}) => {}", first, second, first < second, position);
                if first < second {
                    program.insert(position, 1);
                } else {
                    program.insert(position, 0);
                }
                *pos += 4;
                continue;
            }
            "08" => {
                let first = load(mode[0], param_one, program, rel_base);
                let second = load(mode[1], param_two, program, rel_base);
                let position = position_load(mode[2], program, *pos, *rel_base);
                //println!("eq {} {} ({}) => {}", first, second, first == second, position);
                if first == second {
                    program.insert(position, 1);
                } else {
                    program.insert(position, 0);
                }
                *pos += 4;
                continue;
            }
            "09" => {
                let first = load(mode[0], param_one, program, rel_base);
                //print!("rel {} {}", rel_base, first);
                *rel_base = (*rel_base as i64 + first) as usize;
                //println!(" ({})", rel_base);
                *pos += 2;
                continue;
            }
            "99" => {
                //println!("END");
                return;
            }
            _ => panic!("invalid instruction"),
        }
    }
    //output
}

fn load(mode_bool: i64, param: i64, program: &HashMap<usize, i64>, rel_base: &usize) -> i64 {
    match mode_bool {
        2 => *program
            .get(&((*rel_base as i64 + param) as usize))
            .unwrap_or(&0),
        1 => param,
        0 => *program.get(&(param as usize)).unwrap_or(&0),
        _ => panic!(),
    }
}

fn position_load(
    mode_int: i64,
    program: &HashMap<usize, i64>,
    pos: usize,
    rel_base: usize,
) -> usize {
    match mode_int {
        2 => (rel_base as i64 + *program.get(&(pos + 3)).unwrap_or(&0)) as usize,
        1 => {
            let position = *program.get(&(pos + 3)).unwrap_or(&0) as usize;
            *program.get(&position).unwrap_or(&0) as usize
        }
        0 => *program.get(&(pos + 3)).unwrap_or(&0) as usize,
        _ => panic!(),
    }
}

fn position_write(
    mode_int: i64,
    program: &mut HashMap<usize, i64>,
    param: i64,
    to_write: i64,
    rel_base: usize,
) {
    let position = {
        match mode_int {
            2 => (rel_base as i64 + param) as usize,
            1 => *program.get(&(param as usize)).unwrap_or(&0) as usize,
            0 => param as usize,
            _ => panic!("non-valid adress type"),
        }
    };
    program.insert(position, to_write);
}
