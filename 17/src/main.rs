fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");
    let array: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<i64>().unwrap())
        .collect();
    let mut machine = Machine {
        memory: array,
        input: vec![],
        output: vec![],
        instruction_pointer: 0,
        relative_base: 0,
        end: false,
    };

    let result = part_one(&mut machine);
    println!("{}", result);

    Ok(())
}

fn part_one(machine: &mut Machine) -> i64 {
    let _ = machine.run_continuously();
    let out_raw = machine.output.clone();
    let mut out_string = String::with_capacity(out_raw.len());
    for number in out_raw.iter() {
        out_string.push((*number as u8) as char);
    }
    let mut intersections = Vec::new();
    let by_lines: Vec<Vec<char>> = out_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    for x in 1..by_lines.len() - 2 {
        for y in 1..by_lines[x].len() - 1 {
            if by_lines[x][y] == '#'
                && by_lines[x - 1][y] == '#'
                && by_lines[x + 1][y] == '#'
                && by_lines[x][y - 1] == '#'
                && by_lines[x][y + 1] == '#'
            {
                intersections.push((x, y));
            }
        }
    }
    println!("{}", out_string);
    intersections.iter().map(|(x, y)| x * y).sum::<usize>() as i64
}

#[derive(Debug)]
struct Machine {
    memory: Vec<i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    instruction_pointer: usize,
    relative_base: i64,
    end: bool,
}

impl Machine {
    fn load_argument(&self, arg: usize) -> Result<Address, ()> {
        let arg_adr = Address::Value(arg);
        let mem = self.get_memory(arg_adr)?;
        let opcode = format!("{:05}", self.get_memory(Address::Value(0))?);
        match &opcode[(3 - arg)..(4 - arg)] {
            "2" => Ok(Address::Relative(mem)),
            "1" => Ok(arg_adr),
            "0" => Ok(Address::Position(mem as usize)),
            _ => panic!(), //Err(()),
        }
    }
    fn get_memory(&self, position: Address) -> Result<i64, ()> {
        match position {
            Address::Position(adr) => Ok(*self.memory.get(adr).unwrap_or(&0)),
            Address::Relative(offset) => Ok(*self
                .memory
                .get((self.relative_base + offset) as usize)
                .unwrap_or(&0)),
            Address::Value(arg_number) => Ok(*self
                .memory
                .get((self.instruction_pointer + arg_number) as usize)
                .unwrap_or(&0)),
        }
    }
    fn write_memory(&mut self, position: Address, value: i64) {
        let pos = match position {
            Address::Position(adr) => adr,
            Address::Relative(offset) => (self.relative_base + offset) as usize,
            Address::Value(arg_number) => self.instruction_pointer + arg_number,
        };
        if self.memory.capacity() <= pos {
            self.memory.reserve(pos + 1);
        }
        if self.memory.len() <= pos {
            for _ in self.memory.len()..=pos {
                self.memory.push(0);
            }
        }
        self.memory[pos] = value;
    }
    fn cycle(&mut self) -> Result<(), ()> {
        if self.end {
            return Ok(());
        }
        let opcode = format!("{:05}", self.get_memory(Address::Value(0))?);
        //println!("{:?}", self);
        //print!("\n{:03}: ", self.instruction_pointer);
        match &opcode[3..5] {
            "01" => {
                let arg1 = self.load_argument(1)?;
                let arg2 = self.load_argument(2)?;
                let arg3 = self.load_argument(3)?;
                let first = self.get_memory(arg1)?;
                let second = self.get_memory(arg2)?;
                let sum = first + second;
                self.write_memory(arg3, sum);
                self.instruction_pointer += 4;
                /*println!(
                    "add {}:{} {}:{} {}:{}",
                    first, arg1, second, arg2, sum, arg3
                );*/
            }
            "02" => {
                let arg1 = self.load_argument(1)?;
                let arg2 = self.load_argument(2)?;
                let arg3 = self.load_argument(3)?;
                let first = self.get_memory(arg1)?;
                let second = self.get_memory(arg2)?;
                let sum = first * second;
                self.write_memory(arg3, sum);
                self.instruction_pointer += 4;
                /*println!(
                    "mul {}:{} {}:{} {}:{}",
                    first, arg1, second, arg2, sum, arg3
                );*/
            }
            "03" => {
                let first = self.load_argument(1)?;
                if self.input.len() > 0 {
                    self.write_memory(first, self.input[0]);
                    //println!("in  {}:{}", self.input[0], first);
                    self.input.remove(0);
                    self.instruction_pointer += 2;
                } else {
                    return Err(());
                }
            }
            "04" => {
                let arg1 = self.load_argument(1)?;
                let first = self.get_memory(arg1)?;
                self.output.push(first);
                //println!("out {}:{}", first, arg1);
                self.instruction_pointer += 2;
            }
            "05" => {
                let arg1 = self.load_argument(1)?;
                let arg2 = self.load_argument(2)?;
                let first = self.get_memory(arg1)?;
                let second = self.get_memory(arg2)?;
                //println!("jmp_true {}:{} {}:{}", first, arg1, second, arg2);
                if first != 0 {
                    self.instruction_pointer = second as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            "06" => {
                let arg1 = self.load_argument(1)?;
                let arg2 = self.load_argument(2)?;
                let first = self.get_memory(arg1)?;
                let second = self.get_memory(arg2)?;
                //println!("jmp_false {}:{} {}:{}", first, arg1, second, arg2);
                if first == 0 {
                    self.instruction_pointer = second as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            "07" => {
                let arg1 = self.load_argument(1)?;
                let arg2 = self.load_argument(2)?;
                let arg3 = self.load_argument(3)?;
                let first = self.get_memory(arg1)?;
                let second = self.get_memory(arg2)?;
                if first < second {
                    self.write_memory(arg3, 1);
                //println!("less {}:{} {}:{} 1:{}", first, arg1, second, arg2, arg3);
                } else {
                    self.write_memory(arg3, 0);
                    //println!("less {}:{} {}:{} 0:{}", first, arg1, second, arg2, arg3);
                }
                self.instruction_pointer += 4;
            }
            "08" => {
                let arg1 = self.load_argument(1)?;
                let arg2 = self.load_argument(2)?;
                let arg3 = self.load_argument(3)?;
                let first = self.get_memory(arg1)?;
                let second = self.get_memory(arg2)?;
                if first == second {
                    self.write_memory(arg3, 1);
                //println!("eq {}:{} {}:{} 1:{}", first, arg1, second, arg2, arg3);
                } else {
                    self.write_memory(arg3, 0);
                    //println!("eq {}:{} {}:{} 0:{}", first, arg1, second, arg2, arg3);
                }
                self.instruction_pointer += 4;
            }
            "09" => {
                let arg1 = self.load_argument(1)?;
                let first = self.get_memory(arg1)?;
                self.relative_base += first;
                self.instruction_pointer += 2;
                //println!("rel {}:{}", first, arg1);
            }
            "99" => {
                //println!("END");
                self.end = true;
            }
            _ => panic!(), //return Err(()),
        }
        Ok(())
    }
    fn run_continuously(&mut self) -> Result<i64, ()> {
        while !self.end {
            self.cycle()?;
        }
        Ok(self.output[self.output.len() - 1])
    }
    fn run_till_out(&mut self) -> Result<i64, ()> {
        while self.output.len() == 0 && !self.end {
            self.cycle()?;
        }
        Ok(self.output.pop().unwrap_or(0))
    }
}

#[derive(Clone, Copy)]
enum Address {
    Position(usize),
    Relative(i64),
    Value(usize),
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Address::Position(adr) => write!(f, "[{}]", adr),
            Address::Relative(adr) => write!(f, "{{{}}}", adr),
            Address::Value(adr) => write!(f, "{}", adr),
        }
    }
}
