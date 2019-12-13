pub fn mergesort(input: &[(f64, (usize, usize))]) -> Vec<(f64, (usize, usize))> {
    if input.len() == 1 {
        return Vec::from(input);
    }
    if input.len() == 2 {
        if input[0].0 > input[1].0 {
            return vec![input[1], input[0]];
        } else {
            return Vec::from(input);
        }
    }
    let (first, second) = input.split_at(input.len() / 2);
    let first_vec = mergesort(first);
    let second_vec = mergesort(second);
    let mut output = Vec::with_capacity(input.len());
    let mut left = 0;
    let mut right = 0;
    for i in 0..input.len() {
        let x = {
            if left < first_vec.len() {
                first_vec[left].0
            } else {
                i32::max_value() as f64
            }
        };
        let y = {
            if right < second_vec.len() {
                second_vec[right].0
            } else {
                i32::max_value() as f64
            }
        };
        if x < y {
            output.insert(i, first_vec[left]);
            left += 1;
        } else {
            output.insert(i, second_vec[right]);
            right += 1;
        }
    }
    output
}
