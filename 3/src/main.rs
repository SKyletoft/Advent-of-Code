fn main() {
    let input = include_str!("input.txt");
    let wires: Vec<&str> = input.lines().collect();
    let wire_1 = interpret(wires[0]);
    let wire_2 = interpret(wires[1]);
    let mut crossings = Vec::new();
    for (index_1, coord_1) in wire_1.iter().enumerate() {
        for (index_2, coord_2) in wire_2.iter().enumerate() {
            if coord_1 == coord_2 {
                //crossings.push(manhattan(coord_1));
                crossings.push(index_1 + index_2 + 2);
            }
        }
    }
    let winner = crossings.iter().min().unwrap();
    println!("{}", winner);
}

fn manhattan(coord: &(i32, i32)) -> i32 {
    coord.0.abs() + coord.1.abs()
}

fn str_to_usize(string: &str) -> usize {
    string.parse::<usize>().unwrap()
}

fn interpret(wire_str: &str) -> Vec<(i32, i32)> {
    let mut iterator = wire_str.split(',');
    let mut wire_vec = Vec::new();
    let mut x = 0;
    let mut y = 0;
    loop {
        let this = iterator.next();
        if let Some(string) = this {
            match string.get(0..1) {
                Some("U") => {
                    for _ in 0..str_to_usize(&string[1..string.len()]) {
                        y -= 1;
                        wire_vec.push((x, y));
                    }
                }
                Some("D") => {
                    for _ in 0..str_to_usize(&string[1..string.len()]) {
                        y += 1;
                        wire_vec.push((x, y));
                    }
                }
                Some("L") => {
                    for _ in 0..str_to_usize(&string[1..string.len()]) {
                        x -= 1;
                        wire_vec.push((x, y));
                    }
                }
                Some("R") => {
                    for _ in 0..str_to_usize(&string[1..string.len()]) {
                        x += 1;
                        wire_vec.push((x, y));
                    }
                }
                _ => {}
            }
        } else {
            break;
        }
    }
    wire_vec
}
