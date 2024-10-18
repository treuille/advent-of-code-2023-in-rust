use cached::proc_macro::cached;
use itertools::Itertools;
use std::iter;

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12.txt");

    // Solve 12a
    let puzzle = parse_input(input);
    let sol_12a: usize = solve(&puzzle);
    let correct_sol_12a: usize = 7622;
    //let correct_sol_12a: usize = 21;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    // Solve 12b
    let puzzle = increase_puzzle_size(puzzle);
    let sol_12b: usize = solve(&puzzle);
    //let correct_sol_12b: usize = 525152;
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
    if row.iter().filter(|&&c| c != '.').count() < damaged_springs.iter().sum() {
        return 0;
    }
    let mut row: &[char] = &row;
    let mut damaged_springs: &[usize] = &damaged_springs;
    loop {
        match (row.first(), damaged_springs.first()) {
            (Some(&'.'), _) => {
                // Remove the first element from row
                row = &row[1..];
                return count_arrangements(row.to_vec(), damaged_springs.to_vec());
            }
            (Some(&'#'), Some(&first_damaged_spring)) => {
                // Skip any contiguous damaged springs
                if row.len() < first_damaged_spring {
                    return 0;
                }
                if row[0..first_damaged_spring].iter().all(|&c| c != '.') {
                    row = &row[first_damaged_spring..];
                    damaged_springs = &damaged_springs[1..];
                    match row.first() {
                        Some(&'#') => return 0,
                        Some(_) => row = &row[1..],
                        None => (),
                    }
                } else {
                    return 0;
                }
            }
            (Some(&'#'), _) => {
                if row.iter().all(|&c| c != '#') {
                    return 1;
                } else {
                    return 0;
                }
            }
            (Some(&'?'), _) => {
                let case_a_row = iter::once('.')
                    .chain(row.iter().skip(1).copied())
                    .collect_vec();
                let case_a = count_arrangements(case_a_row, damaged_springs.to_vec());
                let case_b_row = iter::once('#')
                    .chain(row.iter().skip(1).copied())
                    .collect_vec();
                let case_b = count_arrangements(case_b_row, damaged_springs.to_vec());
                return case_a + case_b;
            }
            (None, Some(_)) => return 0,
            (None, _) => return 1,
            (Some(&c), _) => unimplemented!("unexpected character: {}", c),
        }
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
