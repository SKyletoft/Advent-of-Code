use std::collections::HashMap;

fn main() {
    let input = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"; //"9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL"; //include_str!("input.txt");
    let interpreted: Vec<Reaction> = input.lines().map(|line| Reaction::new(line)).collect();
    let result = part_one_rewrite(&interpreted);
    println!("{}", result);
}

fn part_one(reactions: &[Reaction]) -> i64 {
    let mut inventory: HashMap<&str, i64> = HashMap::new();
    inventory.insert("FUEL", -1);
    inventory.insert("ORE", i64::max_value());
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
                    if !in_inventory.contains(&requirement.1.as_str()) {
                        in_inventory.push(&requirement.1);
                    }
                }
                inventory.insert(
                    in_inventory[i],
                    *inventory.get(in_inventory[i]).unwrap_or(&0) + reactions[index].result_count,
                );
            }
        }
    }
    i64::max_value() - inventory.get("ORE").unwrap()
}

fn _part_two(reactions: &[Reaction]) -> i64 {
    let per_fuel = part_one(reactions);
    let fuel_produced_directly = 1_000_000_000_000 / per_fuel;
    let ore_used = per_fuel * fuel_produced_directly;
    let _ore_remaining = 1_000_000_000_000 - ore_used;

    //HOW TO HANDLE REST PRODUCT!?

    0
}

fn part_one_rewrite(reactions: &[Reaction]) -> i64 {
    let mut inventory: HashMap<&str, i64> = HashMap::new();
    let mut reaction_priority_list = vec![(
        1,
        &reactions[reactions
            .iter()
            .position(|reaction| reaction.result_name == "FUEL")
            .unwrap()],
    )];
    'outer: loop {
        'inner: for i in 0..reaction_priority_list.len() {
            let (amount, reaction) = reaction_priority_list[i];
            let mut needs: Vec<(i64, &Reaction)> = Vec::new();
            for requirement in reaction.requirements.iter() {
                if *inventory.get(requirement.1.as_str()).unwrap_or(&0) < requirement.0 * amount
                    || requirement.1 != "ORE"
                {
                    needs.push((
                        amount,
                        &reactions[reactions
                            .iter()
                            .position(|reaction| reaction.result_name == requirement.1)
                            .unwrap()],
                    ));
                }
            }
            if needs.len() == 0 {
                //DO REACTION
            } else {
                for need in needs.iter() {
                    reaction_priority_list.push(*need);
                }
            }
        }
    }

    let cmp = part_one(reactions);
    //println!("{} {}", cmp, ore_used);
    0
}

struct Reaction {
    result_name: String,
    result_count: i64,
    requirements: Vec<(i64, String)>,
}

impl Reaction {
    fn new(line: &str) -> Reaction {
        let mut initial_split = line.split(" => ");
        let requirements: Vec<(i64, String)> = initial_split
            .next()
            .unwrap()
            .split(", ")
            .map(|req| {
                let mut split = req.split(" ");
                (
                    split.next().unwrap().parse::<i64>().unwrap(),
                    String::from(split.next().unwrap()),
                )
            })
            .collect();
        let (count, name) = {
            let mut split = initial_split.next().unwrap().split(" ");
            (
                split.next().unwrap().parse::<i64>().unwrap(),
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
