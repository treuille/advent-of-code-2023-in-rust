use std::array;

fn main() {
    //// Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_15.txt").trim();

    // Solve 15a
    let sol_15a: usize = solve_15a(input);
    let correct_sol_15a: usize = 515495;
    println!("* 15a *");
    println!("My solution: {sol_15a}");
    println!("Correct solution: {correct_sol_15a}");
    println!("Equal: {}\n", sol_15a == correct_sol_15a);

    // Solve 15b
    let sol_15b: usize = solve_15b(input);
    let correct_sol_15b: usize = 229349;
    println!("* 15b *");
    println!("My solution: {sol_15b}");
    println!("Correct solution: {correct_sol_15b}");
    println!("Equal: {}\n", sol_15b == correct_sol_15b);
}

fn solve_15a(input: &str) -> usize {
    input.split(",").map(hash).sum()
}

fn solve_15b(input: &str) -> usize {
    let mut boxes: [Vec<(&str, usize)>; 256] = array::from_fn(|_| Vec::new());
    for operation in input.split(",") {
        if let Some(eq_idx) = operation.find('=') {
            let label = &operation[..eq_idx];
            let focal_length: usize = operation[eq_idx + 1..].parse().unwrap();
            let box_idx = hash(label);
            if let Some(box_lens) = boxes[box_idx]
                .iter_mut()
                .find(|(box_label, _)| *box_label == label)
            {
                box_lens.1 = focal_length;
            } else {
                boxes[box_idx].push((label, focal_length));
            }
        } else {
            assert!(operation.ends_with('-'));
            let label = &operation[..operation.len() - 1];
            let box_idx = hash(label);
            boxes[box_idx].retain(|(box_label, _)| *box_label != label);
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_idx, a_box)| {
            a_box
                .iter()
                .enumerate()
                .map(move |(slot_idx, &(_, focal_length))| {
                    (box_idx + 1) * (slot_idx + 1) * focal_length
                })
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.chars().fold(0usize, |current_value, c| {
        (current_value + (c as usize)) * 17 % 256
    })
}
