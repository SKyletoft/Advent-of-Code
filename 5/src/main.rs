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
        let param_one = array[pos + 1];
        let param_two = array[pos + 2];
        let param_three = array[pos + 3];
        let mut mode = [false, false, false];
        match &opcode[2..3] {
            "0" => mode[0] = true,
            "1" => mode[0] = false,
            _ => panic!(),
        }
        match &opcode[1..2] {
            "0" => mode[1] = true,
            "1" => mode[1] = false,
            _ => panic!(),
        }
        match &opcode[0..1] {
            "0" => mode[2] = true,
            "1" => mode[2] = false,
            _ => panic!(),
        }
        let op = &opcode[3..5];
        let _x = 5;
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
                
            }
            "99" => break,
            _ => panic!(),
        }
    }
    output
}

fn load (mode_bool: bool, param: i32, array: &[i32]) -> i32 {
    if mode_bool {
        array[param as usize]
    } else {
        param
    }
}