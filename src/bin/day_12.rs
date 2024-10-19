use cached::proc_macro::cached;
use std::iter;

/// Initially, I had rejectd this dynamic programming solution because
/// I didn't think of using memoization until I saw other solutions using it.
/// Therefore, my `main` branch solution solves this in <4s *without* using a
/// divide-and-conquer approach. However, I must admit that this solution is way
/// faster, more elegant, and requires less code. I'm keeping the old solution though
/// bcause I'm proud of it, and because it reflects an interesteing structure
/// to the problem.
fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12.txt");

    // Solve 12a
    let puzzle = parse_input(input);
    let sol_12a: usize = solve(&puzzle);
    let correct_sol_12a: usize = 7622;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    // Solve 12b
    let puzzle = increase_puzzle_size(puzzle);
    let sol_12b: usize = solve(&puzzle);
    let correct_sol_12b: usize = 4964259839627;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

fn solve(puzzle: &[(String, Vec<usize>)]) -> usize {
    puzzle
        .iter()
        .map(|(row, damaged_springs)| count_arrangements(row.clone(), damaged_springs.clone()))
        .sum::<usize>()
}

fn increase_puzzle_size(puzzle: Vec<(String, Vec<usize>)>) -> Vec<(String, Vec<usize>)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let row = iter::repeat(row).take(5).collect::<Vec<_>>().join("?");
            let damaged_springs = iter::repeat(damaged_springs).take(5).flatten().collect();
            (row, damaged_springs)
        })
        .collect()
}

#[cached]
fn count_arrangements(row: String, damaged_springs: Vec<usize>) -> usize {
    // This three-line acceleration decreases runtime by 30%
    if row.chars().filter(|&c| c != '.').count() < damaged_springs.iter().sum() {
        return 0;
    }
    match (row.chars().next(), damaged_springs.first()) {
        (Some('.'), _) => count_arrangements(row[1..].to_string(), damaged_springs.to_vec()),
        (Some('#'), Some(&first_damaged_spring)) if row.len() < first_damaged_spring => 0,
        (Some('#'), Some(&first_damaged_spring))
            if row.chars().take(first_damaged_spring).all(|c| c != '.') =>
        {
            let row = &row[first_damaged_spring..];
            let damaged_springs = &damaged_springs[1..];
            match row.chars().next() {
                Some('#') => 0,
                Some(_) => count_arrangements(row[1..].to_string(), damaged_springs.to_vec()),
                None => count_arrangements(row.to_string(), damaged_springs.to_vec()),
            }
        }
        (Some('#'), Some(_)) => 0,
        (Some('#'), _) if row.chars().all(|c| c != '#') => 1,
        (Some('#'), _) => 0,
        (Some('?'), _) => {
            count_arrangements(
                iter::once('.').chain(row.chars().skip(1)).collect(),
                damaged_springs.to_vec(),
            ) + count_arrangements(
                iter::once('#').chain(row.chars().skip(1)).collect(),
                damaged_springs.to_vec(),
            )
        }
        (None, Some(_)) => 0,
        (None, _) => 1,
        (Some(c), _) => unimplemented!("unexpected character: {}", c),
    }
}

fn parse_input(input: &'static str) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (row, damaged_springs) = line.split_once(" ").unwrap();
            let damaged_springs = damaged_springs.split(",").flat_map(str::parse).collect();
            (row.to_string(), damaged_springs)
        })
        .collect()
}
