use cached::proc_macro::cached;
use std::iter;

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

fn solve(puzzle: &[(Vec<char>, Vec<usize>)]) -> usize {
    puzzle
        .iter()
        .map(|(row, damaged_springs)| count_arrangements(row.clone(), damaged_springs.clone()))
        .sum::<usize>()
}

fn increase_puzzle_size(puzzle: Vec<(Vec<char>, Vec<usize>)>) -> Vec<(Vec<char>, Vec<usize>)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let row_len = row.len();
            let damaged_spring_len = damaged_springs.len();
            let row = row
                .into_iter()
                .chain(['?'])
                .cycle()
                .take(row_len * 5 + 4)
                .collect();
            let damaged_springs = damaged_springs
                .into_iter()
                .cycle()
                .take(damaged_spring_len * 5)
                .collect();
            (row, damaged_springs)
        })
        .collect()
}

#[cached]
fn count_arrangements(row: Vec<char>, damaged_springs: Vec<usize>) -> usize {
    // This three-line acceleration decreases runtime by 30%, even with memoization
    if row.iter().filter(|&&c| c != '.').count() < damaged_springs.iter().sum() {
        return 0;
    }
    match (row.first(), damaged_springs.first()) {
        (Some(&'.'), _) => count_arrangements(row[1..].to_vec(), damaged_springs.to_vec()),
        (Some(&'#'), Some(&first_damaged_spring)) if row.len() < first_damaged_spring => 0,
        (Some(&'#'), Some(&first_damaged_spring))
            if row[0..first_damaged_spring].iter().all(|&c| c != '.') =>
        {
            let row = &row[first_damaged_spring..];
            let damaged_springs = &damaged_springs[1..];
            match row.first() {
                Some(&'#') => 0,
                Some(_) => count_arrangements(row[1..].to_vec(), damaged_springs.to_vec()),
                None => count_arrangements(row.to_vec(), damaged_springs.to_vec()),
            }
        }
        (Some(&'#'), Some(_)) => 0,
        (Some(&'#'), _) if row.iter().all(|&c| c != '#') => 1,
        (Some(&'#'), _) => 0,
        (Some(&'?'), _) => {
            count_arrangements(
                iter::once('.').chain(row[1..].iter().copied()).collect(),
                damaged_springs.to_vec(),
            ) + count_arrangements(
                iter::once('#').chain(row[1..].iter().copied()).collect(),
                damaged_springs.to_vec(),
            )
        }
        (None, Some(_)) => 0,
        (None, _) => 1,
        (Some(&c), _) => unimplemented!("unexpected character: {}", c),
    }
}

fn parse_input(input: &'static str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (row, damaged_springs) = line.split_once(" ").unwrap();
            let damaged_springs = damaged_springs.split(",").flat_map(str::parse).collect();
            (row.chars().collect(), damaged_springs)
        })
        .collect()
}
