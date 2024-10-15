#[allow(unused_imports)]
use std::path::PrefixComponent;

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

fn solve(puzzle: &mut [(Vec<char>, Vec<usize>)]) -> usize {
    let mut total_arrangements = 0;
    for (row, damaged_springs) in puzzle {
        println!("row: {:?}", row);
        println!("damaged_springs: {:?}\n", damaged_springs);
        let arrangements = count_arrangements(row, damaged_springs);
        println!("arrangements: {}\n", arrangements);
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

#[allow(unreachable_code, unused_variables, clippy::never_loop)]
fn count_arrangements(mut row: &mut [char], mut damaged_springs: &[usize]) -> usize {
    loop {
        match row.first() {
            Some(&'.') => {
                // Skip any operational springs
                //println!("row: {:?}", row);
                //println!("damaged_springs: {:?}", damaged_springs);
                //println!("Test case A1\n");
                row = &mut row[1..];
            }
            Some(&'#') => {
                // Skip any contiguous damaged springs
                if let Some(&first_damaged_spring) = damaged_springs.first() {
                    if row.len() < first_damaged_spring {
                        //println!("row: {:?}", row);
                        //println!("damaged_springs: {:?}", damaged_springs);
                        //println!("Test case A2 - NO SOLUTION\n");
                        return 0;
                    }
                    if row[0..first_damaged_spring].iter().all(|&c| c != '.') {
                        //println!("row: {:?}", row);
                        //println!("damaged_springs: {:?}", damaged_springs);
                        //println!("Test case A3\n");
                        row = &mut row[first_damaged_spring..];
                        damaged_springs = &damaged_springs[1..];
                        match row.first() {
                            Some(&'#') => return 0,
                            Some(_) => row = &mut row[1..],
                            None => (),
                        }
                    } else {
                        //println!("row: {:?}", row);
                        //println!("damaged_springs: {:?}", damaged_springs);
                        //println!("Test case B - NO SOLUTION\n");
                        return 0;
                    }
                } else if row.iter().all(|&c| c != '#') {
                    // The rest of the row is consistant with no more damaged springs
                    //println!("row: {:?}", row);
                    //println!("damaged_springs: {:?}", damaged_springs);
                    //panic!("Test case C - FOUND A SOLUTION\n");
                    return 1;
                } else {
                    // The rest of the row is *inconsistant* with no more damaged springs
                    //println!("row: {:?}", row);
                    //println!("damaged_springs: {:?}", damaged_springs);
                    //println!("Test case D - NO SOLUTION\n");
                    return 0;
                }
            }
            Some(&'?') => {
                //println!("row: {:?}", row);
                //println!("damaged_springs: {:?}", damaged_springs);
                //println!("Test case G\n");
                row[0] = '.';
                let case_a = count_arrangements(row, damaged_springs);
                row[0] = '#';
                let case_b = count_arrangements(row, damaged_springs);
                row[0] = '?';
                return case_a + case_b;
            }
            None => {
                #[allow(clippy::redundant_pattern_matching)]
                if let Some(_) = damaged_springs.first() {
                    // Cannot satisfy
                    //println!("row: {:?}", row);
                    //println!("damaged_springs: {:?}", damaged_springs);
                    //println!("Test case E - NO SOLUTION\n");
                    return 0;
                } else {
                    //println!("row: {:?}", row);
                    //println!("damaged_springs: {:?}", damaged_springs);
                    //println!("Test case F - FOUND A SOLUTION\n");
                    return 1;
                }
                // no springs left
                unimplemented!("no springs left");
            }
            Some(&c) => unimplemented!("unexpected character: {}", c),
        }
    }
    //if let Some(&spring) = row.first() {
    unimplemented!("count_arrangements");
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
