fn main() {
    let input = include_str!("input.txt");
    let width = 25;
    let height = 6;
    let layers = {
        let layer_size = height * width;
        let layer_count = input.len() / layer_size;
        let mut layers = Vec::new();
        for i in 0..layer_count {
            layers.push(&input[(i * layer_size)..((i + 1) * layer_size)]);
        }
        layers
    };
    //part_one(&layers);
    part_two(&layers, (width, height));
}

fn part_two(layers: &Vec<&str>, size: (usize, usize)) {
    let layer_size = size.0 * size.1;
    let mut out = String::with_capacity(layer_size);
    for index in 0..layer_size {
        out.insert(index, {
            let mut to_append = 'A';
            for layer in layers.iter() {
                if layer.get(index..(index + 1)).unwrap() == "2" {
                    continue;
                }
                to_append = layer.chars().nth(index).unwrap();
                break;
            }
            to_append
        });
    }
    println!("{}\n", out);
    for i in 0..size.1 {
        println!("{}", (&out[(i * size.0)..((i + 1) * size.0)]));
    }
    println!("\n\n");
    out = out.chars().map(|digit| if digit == '1' {'O'} else {' '}).collect();
    for i in 0..size.1 {
        println!("{}", (&out[(i * size.0)..((i + 1) * size.0)]));
    }
}

fn part_one(layers: &Vec<&str>) {
    let mut lowest = (0, usize::max_value(), 0, 0);
    for (index, layer) in layers.iter().enumerate() {
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        for digit in layer.chars() {
            if digit == '0' {
                zeroes += 1;
            } else if digit == '1' {
                ones += 1;
            } else if digit == '2' {
                twos += 1;
            }
        }
        if zeroes < lowest.1 {
            lowest = (index, zeroes, ones, twos);
        }
    }
    let res = lowest.2 * lowest.3;
    println!("{}", res);
}
