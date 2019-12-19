//use std::collections::HashMap;

fn main() {
    let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";//include_str!("input.txt");
    let interpreted: Vec<Reaction> = input.lines().map(|line| Reaction::new(line)).collect();
    
    let result = part_one(&interpreted);
    println!("{}", result);
}

fn part_one (reactions: &[Reaction]) -> i32 {
    let mut to_find = vec![(1, "FUEL")];
    loop {
        let index = reactions.iter().position(|reaction| reaction.result_name == to_find[0].1).unwrap();
        for item in reactions[index].requirements.iter() {
            to_find.push((item.0 * to_find[0].0, &item.1, ));
        }
        to_find.remove(0);
        if to_find[0].1 == "ORE" {
            break;
        }
        if to_find.len() == 0 {
            panic!();
        }
    }
    to_find[0].0
}

struct Reaction {
    result_name: String,
    result_count: i32,
    requirements: Vec<(i32, String)>,
}

impl Reaction {
    fn new(line: &str) -> Reaction {
        let mut initial_split = line.split(" => ");
        let requirements: Vec<(i32, String)> = initial_split
            .next()
            .unwrap()
            .split(", ")
            .map(|req| {
                let mut split = req.split(" ");
                println!("REQ: [{}]", req);
                let first = split.next().unwrap();
                let second = split.next().unwrap();
                println!("{}-\n{}-", first, second);
                (
                    first.parse::<i32>().unwrap(),
                    String::from(second),
                )
            })
            .collect();
        let (name, count) = {
            let mut split = initial_split.next().unwrap().split(" ");
            (
                String::from(split.next().unwrap()),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        };

        Reaction {
            result_name: name,
            result_count: count,
            requirements: requirements,
        }
    }
}
