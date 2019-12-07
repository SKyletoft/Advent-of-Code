fn main() {
    let input_string = include_str!("input.txt");
    part_two(&input_string);
}

fn part_two (input_string: &str) {
    let mut to_me = vec!["YOU".to_string()];
    let mut to_father_christmas = vec!["SAN".to_string()];
    from_planet_to_star(input_string, "YOU", &mut to_me);
    from_planet_to_star(input_string, "SAN", &mut to_father_christmas);
    for (my_index, my_orbit) in to_me.iter().enumerate() {
        for (his_index, his_orbit) in to_father_christmas.iter().enumerate() {
            if his_orbit == my_orbit {
                println!("{}", (my_index + his_index) - 2);
                return;
            }
        }
    }
}

fn from_planet_to_star (input_string: &str, planet: &str, orbits: &mut Vec<String>) {
    for orbit in input_string.lines().filter(|line| &line[4..7] == planet) {
        orbits.push(orbit[0..3].to_string());
        from_planet_to_star(input_string, &orbit[0..3], orbits);
        break;
    };
}

fn part_one (input_string: &str) {
    let mut map = std::collections::HashMap::new();
    for line in input_string.lines() {
        map.insert(line[4..7].to_string(), 0);
    }
    map.insert("COM".to_string(), 0);
    let mut start = vec!["COM".to_string()];
    let mut depths = vec![0];
    while start.len() > 0 {
        let search_for = start[0].clone();
        let depth = depths[0];
        let iterator = input_string
            .lines()
            .filter(|line| &line[0..3] == search_for);
        let mut loops = 0;
        for name in iterator {
            loops += 1;
            let ptr = map.get_mut(&name[0..3].to_string());
            match ptr {
                Some(number) => {
                    *number = depth;
                    start.push(name[4..7].to_string());
                    depths.push(depth + 1);
                }
                None => panic!(),
            }
        }
        if loops == 0 {
            *map.get_mut(&search_for).unwrap() = depth;
        }
        start.remove(0);
        depths.remove(0);
    }

    let sum: i32 = map.iter().map(|entry| entry.1).sum();
    println!("\n{}", sum);
}