use std::collections::{/*HashMap, */ HashSet};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    let matches_per_card: Vec<usize> = input
        .lines()
        .map(|line| {
            let card = line.split_once(": ").unwrap().1;
            let (winning_nums, your_nums) = card.split_once(" | ").unwrap();
            let winning_nums: HashSet<&str> = winning_nums.split_whitespace().collect();
            let your_nums: HashSet<&str> = your_nums.split_whitespace().collect();
            winning_nums.intersection(&your_nums).count()
        })
        .collect();

    // Acccumulate partial answers to 4a and 4b in `points` and `copies`
    let mut points = 0;
    let mut copies: Vec<usize> = vec![1; matches_per_card.len()];
    for (i, matches) in matches_per_card.iter().enumerate() {
        for j in (i + 1)..(i + 1 + matches) {
            copies[j] += copies[i];
        }
        points += (1 << matches) >> 1;
    }

    // Solve 4a
    let sol_4a: usize = points;
    let correct_sol_4a: usize = 23028;
    println!("* 4a *");
    println!("My solution: {sol_4a}");
    println!("Correct solution: {correct_sol_4a}");
    println!("Equal: {}\n", sol_4a == correct_sol_4a);

    // Solve 4b
    let sol_4b: usize = copies.iter().sum();
    let correct_sol_4b: usize = 9236992;
    println!("* 4b *");
    println!("My solution: {sol_4b}");
    println!("Correct solution: {correct_sol_4b}");
    println!("Equal: {}\n", sol_4b == correct_sol_4b);
}
