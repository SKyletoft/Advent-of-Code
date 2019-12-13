pub mod mergesort;
use mergesort::*;

fn main() {
    let input = include_str!("input.txt");
    let interpreted: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|symbol| symbol == '#').collect())
        .collect();
    let twohundredth = part_two(&mut interpreted.clone(), part_one(&interpreted), 200);
    println!("{:?}", twohundredth);
}

fn part_one(input: &Vec<Vec<bool>>) -> (usize, usize) {
    let mut record = 0;
    let mut record_index = (0, 0);
    for test_x in 0..input.len() {
        for test_y in 0..input[test_x].len() {
            let mut can_see = 0;
            if input[test_x][test_y] {
                for cmp_x in 0..input.len() {
                    for cmp_y in 0..input[cmp_x].len() {
                        if input[cmp_x][cmp_y]
                            && !(test_x == cmp_x && test_y == cmp_y)
                            && trace(
                                input,
                                (test_x as f64, test_y as f64),
                                (cmp_x as f64, cmp_y as f64),
                            )
                        {
                            can_see += 1;
                        }
                    }
                }
                if can_see > record {
                    record = can_see;
                    record_index = (test_x, test_y);
                }
            }
        }
    }
    println!("Best place: {:?} which can see {}", record_index, record);
    record_index
}

fn part_two(map: &mut Vec<Vec<bool>>, base: (usize, usize), find_index: usize) -> (usize, usize) {
    let base_as_f64 = (base.0 as f64, base.1 as f64);
    let mut vapourised = 0;
    loop {
        let mut angles_linear = Vec::with_capacity(map.len() * map[0].len());
        for (row_index, row) in map.iter().enumerate() {
            for (column_index, _) in row.iter().enumerate() {
                if map[row_index][column_index]
                    && !(row_index == base.0 && column_index == base.1)
                    && trace(map, base_as_f64, (row_index as f64, column_index as f64))
                {
                    angles_linear.push((
                        (2.0 * std::f64::consts::PI)
                            - (column_index as f64 - base.1 as f64)
                                .atan2(row_index as f64 - base.0 as f64),
                        (row_index, column_index),
                    ));
                }
            }
        }
        let sorted = mergesort(&angles_linear);
        for asteroid in sorted.iter() {
            vapourised += 1;
            map[asteroid.1 .0][asteroid.1 .1] = false;
            if vapourised == find_index {
                return (asteroid.1 .1, asteroid.1 .0);
            }
        }
        return (0, 0);
    }
}

fn trace(map: &Vec<Vec<bool>>, from: (f64, f64), to: (f64, f64)) -> bool {
    let (x_step, y_step) = simplify_fraction((to.0 - from.0) as i32, (to.1 - from.1) as i32);
    let mut coords = (from.0 as i32, from.1 as i32);
    loop {
        coords.0 += x_step;
        coords.1 += y_step;
        if coords.0 < 0
            || coords.0 as usize > map.len()
            || coords.1 < 0
            || coords.1 as usize > map[0].len()
            || (coords.0 == to.0 as i32 && coords.1 == to.1 as i32)
        {
            break;
        }
        if map[coords.0 as usize][coords.1 as usize] {
            return false;
        }
    }
    true
}

fn simplify_fraction(dividend: i32, divisor: i32) -> (i32, i32) {
    if dividend == 0 && divisor == 0 {
        (0, 0)
    } else if divisor == 0 {
        (dividend / dividend.abs(), 0)
    } else if dividend == 0 {
        (0, divisor / divisor.abs())
    } else {
        let min = {
            if dividend > divisor {
                divisor
            } else {
                dividend
            }
        };
        let mut ret = (dividend, divisor);
        for i in 2..=min.abs() {
            let first = dividend % i;
            let second = divisor % i;
            if first == 0 && second == 0 {
                ret = simplify_fraction(dividend / i, divisor / i);
                break;
            }
        }
        ret
    }
}
