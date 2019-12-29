use std::collections::HashMap;

fn main() {
    let input = 
                "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
                //"9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL";
                //include_str!("input.txt");
    let interpreted: Vec<Reaction> = input.lines().map(|line| Reaction::new(line)).collect();
    let result2 = part_one(&interpreted);
    println!("DONE 1");
    let result1 = part_one_rewrite(&interpreted);
    println!("DONE 2\n{} {}", result1, result2);
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
        false,
    )];
    'outer: loop {
        let fuel_got = *inventory.get("FUEL").unwrap_or(&0);
        if fuel_got > 0 {
            break;
        }
        'inner: for i in 0..reaction_priority_list.len() {
            let (amount, reaction, ref mut dealt_with) = reaction_priority_list[i];
            let mut needs = Vec::new();
            let mut ok = true;
            for requirement in reaction.requirements.iter() {
                if *inventory.get(requirement.1.as_str()).unwrap_or(&0) < requirement.0 * amount
                    && requirement.1 != "ORE"
                {
                    if !*dealt_with {
                        needs.push((
                            amount * requirement.0,
                            &reactions[reactions
                                .iter()
                                .position(|reaction| reaction.result_name == requirement.1)
                                .unwrap()],
                            false,
                        ));
                    } else {
                        ok = false;
                    }
                }
            }
            *dealt_with = true;
            if needs.len() == 0 && ok {
                let mut made = 0;
                while made < amount {
                    for requirement in reaction.requirements.iter() {
                        let str_name = requirement.1.as_str();
                        let reference_attempt = inventory.get_mut(str_name);
                        let reference = match reference_attempt {
                            Some(valid_reference) => valid_reference,
                            None => {
                                inventory.insert(str_name, 0);
                                inventory.get_mut(str_name).unwrap()
                            }
                        };
                        *reference -= requirement.0;
                    }
                    let str_name = reaction.result_name.as_str();
                    let reference_attempt = inventory.get_mut(str_name);
                    let reference = match reference_attempt {
                        Some(valid_reference) => valid_reference,
                        None => {
                            inventory.insert(str_name, 0);
                            inventory.get_mut(str_name).unwrap()
                        }
                    };
                    *reference += reaction.result_count;
                    made += reaction.result_count;
                }
                reaction_priority_list.remove(i);
                continue 'outer;
            } else {
                for need in needs.iter() {
                    if let Some(index) = reaction_priority_list
                                            .iter_mut()
                                            .position(|reaction| reaction.1 as *const Reaction == need.1 as *const Reaction)
                    {
                        reaction_priority_list[index].0 += need.0;
                    } else {
                        reaction_priority_list.push(*need);
                    }
                }
            }
        }
    }
    let ore_used = (*inventory.get("ORE").unwrap_or(&0)).abs();
    ore_used
}

#[derive(Debug)]
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
