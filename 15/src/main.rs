use std::collections::HashMap;

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

    part_two(&mut machine);
    Ok(())
}

fn part_one(machine: &mut Machine) -> i64 {
    let mut map = HashMap::new();
    //map.insert((0, 0), true);
    let mut exit = (0, 0);
    let result = explore(0, 0, &mut machine.clone(), &mut map, &mut exit, 1, &mut None);
    println!("{} {:?}", result, exit);
    draw_map(&map, exit);
    0
}

fn part_two(machine: &mut Machine) {
    let mut map = HashMap::new();
    //map.insert((0, 0), true);
    let mut depths = Vec::new();
    let mut machine_at_end_pos = Some(Machine {
        memory: vec![],
        input: vec![],
        output: vec![],
        instruction_pointer: 0,
        relative_base: 0,
        end: false,
    });
    explore(0, 0, &mut machine.clone(), &mut map, &mut (0, 0), 1, &mut machine_at_end_pos);
    map = HashMap::new();
    let res = fill_oxygen(-16, -12, &mut machine_at_end_pos.unwrap(), &mut map, 0, &mut depths);
    let max_depth = depths.iter().max().unwrap();
    println!("{}, {}", max_depth, res);
}

fn draw_map(shell: &HashMap<(i64, i64), bool>, end: (i64, i64)) {
    let mut canvas: Vec<Vec<i64>> = Vec::new();
    let min_x = shell.keys().map(|i| i.0).min().unwrap_or(0);
    let min_y = shell.keys().map(|i| i.1).min().unwrap_or(0);
    for (indices, colour) in shell.iter() {
        let (x, y) = ((indices.0 - min_x) as usize, (indices.1 - min_y) as usize);
        while canvas.len() <= x {
            canvas.push(Vec::new());
        }
        while canvas[x].len() <= y {
            canvas[x].push(0);
        }
        canvas[x][y] = if *colour {1} else {0};
    }
    for i in 0..canvas.len() {
        for j in 0..canvas[i].len() {
            if i as i64 + min_x == end.0 && j as i64 + min_y == end.1 {
                print!("E");
            } else if i as i64 + min_x == 0 && j as i64 + min_y == 0 {
                print!("S");
            } else {
                if canvas[i][j] == 1 {
                print!("O");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

fn explore(
    x: i64,
    y: i64,
    machine: &mut Machine,
    map: &mut HashMap<(i64, i64), bool>,
    exit: &mut (i64, i64),
    depth: i64,
    out_machine: &mut Option<Machine>
) -> i64 {
    //OUTPUT KEY
    //  0: continue as normal
    //  1: exit, found exit
    //  2: exit, dead end
    const DIRECTIONS: [(i64, (i64, i64)); 4] =
        [(1, (0, -1)), (2, (0, 1)), (3, (-1, 0)), (4, (1, 0))]; //up, down, left, right
    if let Some(_) = map.get(&(x, y)) {
        return 2;
    }
    map.insert((x, y), true);
    for direction in DIRECTIONS.iter() {
        let mut machine_clone = machine.clone();
        machine_clone.input.push(direction.0);
        let out = machine_clone.run_till_out().unwrap_or(99);
        let new_pos = (x + direction.1 .0, y + direction.1 .1);
        match out {
            0 => {
                map.insert(new_pos, false);
            }
            2 => {
                map.insert(new_pos, true);
                *exit = new_pos;
                *out_machine = Some(machine_clone);
                println!("depth: {}", depth);
                return 1;
            }
            1 => {
                let result = explore(
                    x + direction.1 .0,
                    y + direction.1 .1,
                    &mut machine_clone,
                    map,
                    exit,
                    depth + 1,
                    out_machine
                );
                match result {
                    1 => return 1,
                    2 => continue,
                    _ => panic!(),
                };
            }
            _ => panic!(),
        };
    }
    2
}

fn fill_oxygen(
    x: i64,
    y: i64,
    machine: &mut Machine,
    map: &mut HashMap<(i64, i64), bool>,
    depth: i64,
    depths: &mut Vec<i64>
) -> i64 {
    const DIRECTIONS: [(i64, (i64, i64)); 4] =
        [(1, (0, -1)), (2, (0, 1)), (3, (-1, 0)), (4, (1, 0))]; //up, down, left, right
    if let Some(_) = map.get(&(x, y)) {
        return 2;
    }
    map.insert((x, y), true);
    for direction in DIRECTIONS.iter() {
        let mut machine_clone = machine.clone();
        machine_clone.input.push(direction.0);
        let out = machine_clone.run_till_out().unwrap_or(99);
        let new_pos = (x + direction.1 .0, y + direction.1 .1);
        match out {
            0 => {
                depths.push(depth);
                map.insert(new_pos, false);
            }
            1 => {
                let result = fill_oxygen(
                    x + direction.1 .0,
                    y + direction.1 .1,
                    &mut machine_clone,
                    map,
                    depth + 1,
                    depths
                );
                match result {
                    1 => return 1,
                    2 => continue,
                    _ => panic!(),
                };
            }
            2 => {}
            _ => panic!(),
        };
    }
    2
}

#[derive(Debug, Clone)]
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
                self.write_memory(first, self.input[0]);
                //println!("in  {}:{}", self.input[0], first);
                self.input.remove(0);
                self.instruction_pointer += 2;
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
