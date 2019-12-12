fn main() {
    let input = ".#..#\n.....\n#####\n....#\n...##"; //include_str!("input.txt");
    let mut asterorids = 1;
    let interpreted: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|symbol| {
                    if symbol == '#' {
                        asterorids += 1;
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();
    println!("{}", asterorids);
    part_one(&interpreted);
}

fn part_one(input: &Vec<Vec<bool>>) {
    let mut record = 0;
    let mut record_index = (0, 0);

    for test_x in 0..input.len() {
        for test_y in 0..input[test_x].len() {
            let (test_x, test_y) = (2, 2);
            //FOR EACH SQUARE
            let mut can_see = 0;
            if input[test_x][test_y] {
                println!("{}, {}", test_x, test_y);
                for cmp_x in 0..input.len() {
                    for cmp_y in 0..input[cmp_x].len() {
                        //FOR EACH COMPARISON
                        if input[cmp_x][cmp_y] && !(test_x == cmp_x && test_y == cmp_y) {
                            if trace(
                                input,
                                (test_x as f64, test_y as f64),
                                (cmp_x as f64, cmp_y as f64),
                            ) {
                                println!(
                                    "NO CRASH: {}, {} <-> {}, {}",
                                    test_x, test_y, cmp_x, cmp_y
                                );
                                let _ = trace(
                                    input,
                                    (test_x as f64, test_y as f64),
                                    (cmp_x as f64, cmp_y as f64),
                                );
                                can_see += 1;
                            } else {
                                println!(
                                    "   CRASH: {}, {} <-> {}, {}",
                                    test_x, test_y, cmp_x, cmp_y
                                );
                            }
                        } else {
                            println!("EMPTY SPACE        {}, {}", cmp_x, cmp_y);
                        }
                    }
                }
                if can_see > record {
                    record = can_see;
                    record_index = (test_x, test_y);
                }
            }
            break;
        }
        break;
    }
    println!("Best place: {:?} which can see {}", record_index, record);
}

fn trace(map: &Vec<Vec<bool>>, from: (f64, f64), to: (f64, f64)) -> bool {
    let (min_x, max_x) = {
        if from.0 > to.0 {
            (to.0, from.0)
        } else {
            (from.0, to.0)
        }
    };
    let (min_y, max_y) = {
        if from.1 > to.1 {
            (to.1, from.1)
        } else {
            (from.1, to.1)
        }
    };
    let line = Line::from_points(from, to);
    println!("{:?}", to);
    for x in ((min_x + 1.0) as usize)..(max_x as usize) {
        let y = line.y(x as f64);
        println!("x{}: y{}", x, y);
        if y % 1.0 == 0.0 && (y as usize) < map.len() {
            if map[x][y as usize] {
                return false;
            }
        }
    }
    for y in ((min_y + 1.0) as usize)..(max_y as usize) {
        let x = line.x(y as f64);
        println!("y{}: x{}", y, x);
        if x % 1.0 == 0.0 && (x as usize) < map.len() {
            if map[x as usize][y] {
                return false;
            }
        }
    }
    true
}

struct Line {
    k: f64,
    m: f64,
}

impl Line {
    fn y(&self, x: f64) -> f64 {
        x * self.k + self.m
    }

    fn x(&self, y: f64) -> f64 {
        (y - self.m) / self.k
    }
    
    fn new(k: f64, m: f64) -> Line {
        Line { k: k, m: m }
    }
    
    fn from_points(p1: (f64, f64), p2: (f64, f64)) -> Line {
        let k = (p1.1 - p2.1) / (p1.0 - p2.0);
        let tmp = Line::new(k, 0.0);
        let m = tmp.y(p1.0) - p1.1;
        Line::new(k, m)
    }
}
