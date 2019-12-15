fn main() {
    let initial = vec![
        Planet::new(8, 0, 8),
        Planet::new(0, -5, -10),
        Planet::new(16, 10, -5),
        Planet::new(19, -10, -7),
    ];
    part_one(&initial);
    part_two(&initial);
}

fn part_two(initial: &[Planet]) {
    let mut cycles = vec![
        find_cycle(initial, 0),
        find_cycle(initial, 1),
        find_cycle(initial, 2)
    ];
    
    let limit = {
        let mut clone = cycles.clone();
        clone.sort();
        (clone[0] as f64).sqrt() as usize
    };

    for i in 2..limit {
        let sum: usize = cycles.iter().map(|val| val % i).sum();
        if sum == 0 {
            cycles = cycles.iter().map(|val| val / i).collect();
        }
    }

    let cycle: usize = cycles.iter().product();
    println!("{:?} ({})", cycles, cycle);
}

fn find_cycle(initial: &[Planet], axis: usize) -> usize {
    let mut system = Vec::from(initial);
    let mut loops = 0;
    loop {
        loops += 1;
        for i in 0..system.len() {
            for j in (i + 1)..system.len() {
                gravity_on_pair_single_axis(&mut system, i, j, axis);
            }
        }
        for body in system.iter_mut() {
            body.apply_vel();
        }
        if system == initial {
            break;
        }
    }
    println!("{}: {}", axis, loops);
    loops
}

fn part_one(initial: &[Planet]) {
    let mut system = Vec::from(initial);
    for _ in 0..1000 {
        for i in 0..system.len() {
            for j in (i + 1)..system.len() {
                gravity_on_pair(&mut system, i, j);
            }
        }
        for body in system.iter_mut() {
            body.apply_vel();
        }
    }
    let sum: i32 = system.iter().map(|body| body.kin_en()).sum();
    println!("{}", sum);
}

fn gravity_on_pair_single_axis(array: &mut [Planet], first: usize, second: usize, axis: usize) {
    if array[first].pos[axis] > array[second].pos[axis] {
        array[first].vel[axis] -= 1;
        array[second].vel[axis] += 1;
    } else if array[first].pos[axis] < array[second].pos[axis] {
        array[first].vel[axis] += 1;
        array[second].vel[axis] -= 1;
    }
}

fn gravity_on_pair(array: &mut [Planet], first: usize, second: usize) {
    for i in 0..3 {
        gravity_on_pair_single_axis(array, first, second, i);
    }
}

#[derive(Clone, PartialEq, PartialOrd, std::cmp::Eq, Hash)]
struct Planet {
    vel: [i32; 3],
    pos: [i32; 3],
}

impl Planet {
    fn new(x: i32, y: i32, z: i32) -> Planet {
        Planet {
            vel: [0, 0, 0],
            pos: [x, y, z],
        }
    }

    fn apply_vel(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }

    fn kin_en(&self) -> i32 {
        (self.pos[0].abs() + self.pos[1].abs() + self.pos[2].abs())
            * (self.vel[0].abs() + self.vel[1].abs() + self.vel[2].abs())
    }
}

impl std::fmt::Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.pos[0], self.pos[1], self.pos[2], self.vel[0], self.vel[1], self.vel[2]
        )
    }
}
