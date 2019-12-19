use std::collections::HashMap;

fn main() {
    let input = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL"; //include_str!("input.txt");
    let interpreted: Vec<Reaction> = input.lines().map(|line| Reaction::new(line)).collect();
    let result = part_one(&interpreted);
    println!("{}", result);
}

fn part_one(reactions: &[Reaction]) -> i32 {
    let mut inventory: HashMap<&str, i32> = HashMap::new();
    inventory.insert("FUEL", -1);
    inventory.insert("ORE", i32::max_value());
    let mut in_inventory = vec!["FUEL", "ORE"];
    while *inventory.values().min().unwrap_or(&0) < 0 {
        for i in 0..in_inventory.len() {
            let amount = inventory.get(in_inventory[i]).unwrap_or(&0);
            let material = in_inventory[i];
            if *amount < 0 {
                let index = reactions
                    .iter()
                    .position(|reaction| &reaction.result_name == material)
                    .unwrap();
                for requirement in reactions[index].requirements.iter() {
                    inventory.insert(
                        &requirement.1,
                        *inventory.get(requirement.1.as_str()).unwrap_or(&0) - requirement.0,
                    );
                    inventory.insert(
                        in_inventory[i],
                        *inventory.get(in_inventory[i]).unwrap_or(&0) + reactions[index].result_count,
                    );
                    if !in_inventory.contains(&requirement.1.as_str()) {
                        in_inventory.push(&requirement.1);
                    }
                }
                println!("{:#?}", inventory);
            }
        }
    }
    i32::max_value() - inventory.get("ORE").unwrap()
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
                (
                    split.next().unwrap().parse::<i32>().unwrap(),
                    String::from(split.next().unwrap()),
                )
            })
            .collect();
        let (count, name) = {
            let mut split = initial_split.next().unwrap().split(" ");
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                String::from(split.next().unwrap()),
            )
        };

        Reaction {
            result_name: name,
            result_count: count,
            requirements: requirements,
        }
    }
}
