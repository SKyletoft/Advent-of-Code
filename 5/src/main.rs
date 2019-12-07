fn main() {
    let input = include_str!("input.txt");
    let array: Vec<i32> = input
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();
    let output = machine(1, &array);
    println!("{}", output);
}

fn machine(input: i32, program: &[i32]) -> i32 {
    let mut array = Vec::from(program);
    let mut pos = 0;
    let mut output = 0;
    loop {
        let opcode = format!("{:05}", array[pos]);
        let param_one;
        let param_two;
        match &opcode[2..3] {
            "0" => param_one = array[pos + 1],
            "1" => param_one = array[array[(pos + 1)] as usize],
            _ => panic!(),
        }
        match &opcode[1..2] {
            "0" => param_two = array[pos + 2],
            "1" => param_two = array[array[(pos + 2)] as usize],
            _ => panic!(),
        }

        let op = &opcode[3..5];
        let x = 5;
        match op {
            "01" => {
                let sum = param_one + param_two;
                match &opcode[0..1] {
                    "0" => array[pos + 3] = sum,
                    "1" => {
                        let first = array[(pos + 3)] as usize;
                        array[first] = sum;
                    },
                    _ => panic!(),
                }
                pos += 4;
            },
            "02" => {
                let product = param_one * param_two;
                match &opcode[0..1] {
                    "0" => array[pos + 3] = product,
                    "1" => {
                        let first = array[(pos + 3)] as usize;
                        array[first] = product;
                    },
                    _ => panic!(),
                }
                pos += 4;
            },
            "03" => {
                let adr;
                match &opcode[0..1] {
                    "0" => adr = array[pos + 3],
                    "1" => adr = array[array[(pos + 3)] as usize],
                    _ => panic!(),
                }
                array[adr as usize] = input;
                pos += 2
            },
            "04" => {
                let adr;
                match &opcode[0..1] {
                    "0" => adr = array[pos + 3],
                    "1" => adr = array[array[(pos + 3)] as usize],
                    _ => panic!(),
                }
                output = array[adr as usize];
                pos += 2
            },
            "99" => break,
            _ => panic!()
        }
    }
    output
}
