use cached::proc_macro::cached;
use itertools::Itertools;
use std::iter;

//#[allow(unused_imports)]
//use std::path::PrefixComponent;

#[allow(unreachable_code, clippy::never_loop)]
fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12.txt");
    //let input = include_str!("../../puzzle_inputs/day_12_test_1.txt");

    // Solve 12a
    let mut puzzle = parse_input(input);
    let sol_12a: usize = solve(&mut puzzle);
    //let correct_sol_12a: usize = 7622;
    let correct_sol_12a: usize = 21;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    // Solve 12b
    let mut puzzle = increase_puzzle_size(puzzle);
    let sol_12b: usize = solve(&mut puzzle);
    let correct_sol_12b: usize = 525152;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

fn solve(puzzle: &[(Vec<char>, Vec<usize>)]) -> usize {
    let mut total_arrangements = 0;
    for (row, damaged_springs) in puzzle {
        //println!("row: {:?}", row);
        //println!("damaged_springs: {:?}\n", damaged_springs);
        let arrangements = count_arrangements(row.clone(), damaged_springs.clone());
        //println!("arrangements: {}\n", arrangements);
        total_arrangements += arrangements;
        //panic!("Early exit");
    }
    //println!("total_arrangements: {}", total_arrangements);
    total_arrangements
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

//#[allow(unreachable_code, unused_variables, clippy::never_loop)]
#[cached]
fn count_arrangements(mut row: Vec<char>, mut damaged_springs: Vec<usize>) -> usize {
    if row.iter().filter(|&&c| c != '.').count() < damaged_springs.iter().sum() {
        return 0;
    }
    loop {
        match row.first() {
            Some(&'.') => {
                // Remove the first element from row
                row = row.into_iter().skip(1).collect();
            }
            Some(&'#') => {
                // Skip any contiguous damaged springs
                if let Some(&first_damaged_spring) = damaged_springs.first() {
                    if row.len() < first_damaged_spring {
                        return 0;
                    }
                    if row[0..first_damaged_spring].iter().all(|&c| c != '.') {
                        row = row.into_iter().skip(first_damaged_spring).collect();
                        damaged_springs = damaged_springs.into_iter().skip(1).collect();
                        match row.first() {
                            Some(&'#') => return 0,
                            Some(_) => row = row.into_iter().skip(1).collect(),
                            None => (),
                        }
                    } else {
                        return 0;
                    }
                } else if row.iter().all(|&c| c != '#') {
                    return 1;
                } else {
                    return 0;
                }
            }
            Some(&'?') => {
                let case_a_row = iter::once('.')
                    .chain(row.iter().skip(1).copied())
                    .collect_vec();
                let case_a = count_arrangements(case_a_row, damaged_springs.clone());
                let case_b_row = iter::once('#')
                    .chain(row.iter().skip(1).copied())
                    .collect_vec();
                let case_b = count_arrangements(case_b_row, damaged_springs.clone());
                return case_a + case_b;
            }
            None =>
            {
                #[allow(clippy::redundant_pattern_matching)]
                if let Some(_) = damaged_springs.first() {
                    return 0;
                } else {
                    return 1;
                }
            }
            Some(&c) => unimplemented!("unexpected character: {}", c),
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
