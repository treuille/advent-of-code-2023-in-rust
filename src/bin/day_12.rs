#![allow(clippy::obfuscated_if_else)]
use cached::proc_macro::cached;
use std::iter;

/// A row of springs, eiher '.' (operational), '#' (damaged), or '?' (unknown)
type Row = String;

/// A list of contiguous damaged springs in a row
type DamagedSprings = Vec<usize>;

/// The baic strategy is to match each `row` with a list of `damaged_springs`
/// *recursively* by splitting the row into three parts. The split occurs
/// implicitly at a string of the form ".#####." where the "#"s match the
/// length of center-most element of `damaged_springs`. Additionally, the
/// `#[cached]` memoization turns the computation from 3s to <1s.
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

fn solve(puzzle: &[(Row, DamagedSprings)]) -> usize {
    puzzle
        .iter()
        .map(|(row, damaged_springs)| count_arrangements(row.clone(), damaged_springs.clone()))
        .sum()
}

fn increase_puzzle_size(puzzle: Vec<(Row, DamagedSprings)>) -> Vec<(Row, DamagedSprings)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let damaged_springs_len = damaged_springs.len();
            let row = iter::repeat(row.clone())
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let damaged_springs = damaged_springs
                .into_iter()
                .cycle()
                .take(damaged_springs_len * 5)
                .collect();
            (row, damaged_springs)
        })
        .collect()
}

#[cached]
fn count_arrangements(row: Row, damaged_springs: DamagedSprings) -> usize {
    if damaged_springs.is_empty() {
        match row.chars().all(|c| c != '#') {
            true => 1,
            false => 0,
        }
    } else if row.chars().filter(|&c| c != '.').count() < damaged_springs.iter().sum() {
        0
    } else {
        // Split the damaged_springs into three parts damaged_springs[..,split,..]
        let split = damaged_springs.len() / 2;
        let left_damaged_springs = &damaged_springs[..split];
        let right_damaged_springs = &damaged_springs[(split + 1)..];

        // The split occurs implicitly at a string of the form ".#####."
        // where the "#"s match the length of split_len
        // and each terminal "." depends on having a nonempty left or right split,
        // respectively. These "."s force separation of the damaged springs.
        let split_len = damaged_springs[split];
        let split_str_len = split_len
            + (!left_damaged_springs.is_empty() as usize)
            + (!right_damaged_springs.is_empty() as usize);

        // The pot_damaged vector is an acceleration structure
        // that prevents us from scanning the entire row of damaged springs
        let mut pot_damaged: Vec<usize> = Vec::with_capacity(row.len());
        pot_damaged.extend(iter::once('.').chain(row.chars()).scan(0, |acc, c| {
            *acc += (c == '.').then_some(0).unwrap_or(1);
            Some(*acc)
        }));
        let max_pot_damaged = pot_damaged.last().unwrap();
        let rds_sum: usize = right_damaged_springs.iter().sum();

        // Scan the row for possible splits
        let mut total_arrangements: usize = 0;
        for i in 0..=(row.len() - split_str_len) {
            // Ensure the right split has suffciently many potential damaged springs.
            // Since right_pot_damaged is decreasing, we can break early.
            let right_pot_damaged = max_pot_damaged - pot_damaged[i + split_str_len];
            if right_pot_damaged < rds_sum {
                break;
            }

            // Ensure the center split has suffciently many potential damaged springs
            if pot_damaged[i + split_str_len] - pot_damaged[i] < split_len {
                continue;
            }

            // Ensure that the center split is exctly valid
            let left_empty = left_damaged_springs.is_empty();
            let right_empty = right_damaged_springs.is_empty();
            if !matches_row(&row[i..], split_len, left_empty, right_empty) {
                continue;
            }

            // Count the number of arrangements on the left
            let left_row = &row[..i];
            let left_arrangements =
                count_arrangements(left_row.to_string(), left_damaged_springs.to_vec());

            // Early return if no arrangements are possible on the left
            if left_arrangements == 0 {
                continue;
            }

            // Count the number of arrangements on the right
            let right_row = &row[(i + split_str_len)..];
            let right_arrangements =
                count_arrangements(right_row.to_string(), right_damaged_springs.to_vec());

            // This is te key to the combinatoral speedup
            total_arrangements += left_arrangements * right_arrangements
        }
        return total_arrangements;
    }
}

// Implicitly matches the string ".#####." where the "#"s match the length of split_len
fn matches_row(row: &str, split_len: usize, left_empty: bool, right_empty: bool) -> bool {
    let mut row = row.chars();
    if !left_empty && row.next() == Some('#') {
        return false;
    }
    for _ in 0..split_len {
        match row.next() {
            Some('.') | None => return false,
            _ => (),
        }
    }
    if !right_empty && row.next() == Some('#') {
        return false;
    }
    true
}

fn parse_input(input: &'static str) -> Vec<(Row, DamagedSprings)> {
    input
        .lines()
        .map(|line| {
            let (row, damaged_springs) = line.split_once(" ").unwrap();
            (
                row.to_string(),
                damaged_springs.split(",").flat_map(str::parse).collect(),
            )
        })
        .collect()
}
