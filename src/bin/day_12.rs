#![allow(clippy::obfuscated_if_else)]
use itertools::Itertools;
use std::iter;

/// A row of springs, eiher '.' (operational), '#' (damaged), or '?' (unknown)
type Row = String;

/// A list of contiguous damaged springs in a row
type DamagedSprings = Vec<usize>;

/// I discovered two solutions to this problem:
///
/// 1. A dynamic programming approach, where you scan from the left
///    matching damaged springs and branching on every '?'
///
/// 2. A divide-and-conquer approach, where you split the row into three
///    parts. The split occurs implicitly at a string of the form
///    ".#####." where the "#"s match the length of center-most element
///    of `damaged_springs`.
///
/// The first solution is too slow unless you memoize the results. (I put
/// that first solution in the `fast_dynamic_memoize` branch.) This file
/// contains the second solution. It turns out it's slower than the first,
/// but in some ways more interesting because it reflects the combinatorial
/// structure of the problem more deeply.
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
        .map(|(row, damaged_springs)| count_arrangements(row, damaged_springs.clone()))
        .sum()
}

fn count_arrangements(row: &str, damaged_springs: DamagedSprings) -> usize {
    if damaged_springs.is_empty() {
        return row.chars().all(|c| c != '#') as usize;
    } else if row.chars().filter(|&c| c != '.').count() < damaged_springs.iter().sum() {
        return 0;
    }

    // Split the damaged_springs into three parts damaged_springs[..,split,..]
    let split = damaged_springs.len() / 2;
    let left_damaged_springs = &damaged_springs[..split];
    let right_damaged_springs = &damaged_springs[(split + 1)..];

    // The split occurs implicitly at a string of the form ".#####."
    // where the "#"s match the length of split_len
    // and each terminal "." depends on having a nonempty left or right split,
    // respectively. These "."s force separation of the damaged springs.
    let split_spring = damaged_springs[split];
    let left_empty = left_damaged_springs.is_empty();
    let right_empty = right_damaged_springs.is_empty();
    let split_str_len = split_spring + (!left_empty as usize) + (!right_empty as usize);

    // The pot_damaged vector is an acceleration structure
    // that prevents us from scanning the entire row of damaged springs
    let pot_damaged: Vec<usize> = iter::once(0)
        .chain(row.chars().rev().scan(0, |acc, c| {
            *acc += (c == '.').then_some(0).unwrap_or(1);
            Some(*acc)
        }))
        .collect();
    let pot_damaged: Vec<usize> = pot_damaged.into_iter().rev().collect();

    // Scan the row for possible splits
    let mut total_arrangements: usize = 0;
    for i in 0..=(row.len() - split_str_len) {
        // Ensure the right split has sufficiently many potential damaged springs.
        // Since right_pot_damaged is decreasing, we can break early.
        if pot_damaged[i + split_str_len] < right_damaged_springs.iter().sum() {
            break;
        }

        // Ensure the center split has suffciently many potential damaged springs
        if pot_damaged[i] - pot_damaged[i + split_str_len] < split_spring {
            continue;
        }

        // Ensure that the center split is exctly valid
        let mut center_row = row.chars().skip(i);
        if !left_empty && center_row.next() == Some('#')
            || center_row.by_ref().take(split_spring).any(|c| c == '.')
            || !right_empty && center_row.next() == Some('#')
        {
            continue;
        }

        // Count the number of arrangements on the left
        let left_row = &row[..i];
        let left_arrangements = count_arrangements(left_row, left_damaged_springs.to_vec());

        // Early return if no arrangements are possible on the left
        if left_arrangements == 0 {
            continue;
        }

        // Count the number of arrangements on the right
        let right_row = &row[(i + split_str_len)..];
        let right_arrangements = count_arrangements(right_row, right_damaged_springs.to_vec());

        // This is te key to the combinatoral speedup
        total_arrangements += left_arrangements * right_arrangements
    }
    total_arrangements
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

fn increase_puzzle_size(puzzle: Vec<(Row, DamagedSprings)>) -> Vec<(Row, DamagedSprings)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let row = iter::repeat(row).take(5).collect_vec().join("?");
            let damaged_springs = iter::repeat(damaged_springs).take(5).flatten().collect();
            (row, damaged_springs)
        })
        .collect()
}
