#[allow(unused_imports)]
use cached::proc_macro::cached;

#[allow(unused_imports)]
use regex::Regex;

/// A row of springs, eiher '.' (operational), '#' (damaged), or '?' (unknown)
type Row = String;

/// A list of contiguous damaged springs in a row
type DamagedSprings = Vec<usize>;

#[allow(unreachable_code)]
fn main() {
    //test_damaged_spring_regex();
    //unimplemented!("test_damaged_spring_regex");

    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_12.txt");
    let input = include_str!("../../puzzle_inputs/day_12_test_1.txt");

    // Solve 12a
    let puzzle = parse_input(input);
    let sol_12a: usize = solve(&puzzle);
    //let correct_sol_12a: usize = 7622;
    let correct_sol_12a: usize = 21;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    //// Solve 12b
    //let puzzle = increase_puzzle_size(puzzle);
    //let sol_12b: usize = solve(&puzzle);
    //let correct_sol_12b: usize = 525152;
    //println!("* 12b *");
    //println!("My solution: {sol_12b}");
    //println!("Correct solution: {correct_sol_12b}");
    //println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

//#[cached]
//fn damaged_spring_regex(damaged_spring: usize) -> Regex {
//    let regex = format!(r"^[.?]*[#?]{{{damaged_spring}}}[.?]*$");
//    println!("regex: {}", regex);
//    return Regex::new(&regex).unwrap();
//}
//
//#[allow(dead_code)]
//fn test_damaged_spring_regex() {
//    let _regex = damaged_spring_regex(1);
//    let _regex = damaged_spring_regex(1);
//    let _regex = damaged_spring_regex(1);
//    let _regex = damaged_spring_regex(2);
//    let _regex = damaged_spring_regex(2);
//    let regex = damaged_spring_regex(3);
//    let test_patterns = [".#.", ".#.?", "?#.", "?#?", "?#.?", "?.?#.?", "?.##.?", "#"];
//    for pattern in test_patterns {
//        let is_match = regex.is_match(pattern);
//        println!("pattern: {}, is_match: {}", pattern, is_match);
//    }
//}

fn solve(puzzle: &[(Row, DamagedSprings)]) -> usize {
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

fn increase_puzzle_size(puzzle: Vec<(Row, DamagedSprings)>) -> Vec<(Row, DamagedSprings)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let damaged_springs_len = damaged_springs.len();
            let row = [
                row.clone(),
                row.clone(),
                row.clone(),
                row.clone(),
                row.clone(),
            ]
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

fn rows_match(
    row_1: impl IntoIterator<Item = char>,
    row_2: impl IntoIterator<Item = char>,
) -> bool {
    row_1
        .into_iter()
        .zip(row_2)
        .all(|(a, b)| a == b || a == '?' || b == '?')
    //let s1 = row_1.into_iter().collect::<String>();
    //let s2 = row_2.into_iter().collect::<String>();
    //let s1_clone = s1.clone();
    //let s2_clone = s2.clone();
    //let row_1 = s1_clone.chars();
    //let row_2 = s2_clone.chars();
    //let answer = row_1
    //    .into_iter()
    //    .zip(row_2)
    //    .all(|(a, b)| a == b || a == '?' || b == '?');
    //println!("row_1: {}, row_2: {}, answer: {}", s1, s2, answer);
    //answer
}

#[allow(unused_variables, unreachable_code)]
fn count_arrangements(mut row: &str, damaged_springs: &[usize]) -> usize {
    // TODO: Add a short curcuit basd on the sum(damaged_springs)
    while !row.is_empty() && &row[row.len() - 1..] == "." {
        //println!("shortening: {:?} -> {:?}", row, &row[..(row.len() - 1)]);
        row = &row[..(row.len() - 1)];
    }
    match damaged_springs.len() {
        0 => unreachable!("Must have at least one damaged spring"),
        1 => {
            let damaged_spring = damaged_springs[0];
            if row.len() < damaged_spring {
                0
            } else {
                let beginning = ['.'].into_iter().cycle().take(row.len() - damaged_spring);
                let end = ['#'].into_iter().cycle().take(damaged_spring);
                match rows_match(beginning.chain(end), row.chars()) {
                    false => 0,
                    true => 1,
                }
                //(0..(row.len() - damaged_spring + 1))
                //    .filter(|&start| {
                //        let beginning = ['.'].into_iter().cycle().take(start);
                //        let middle = ['#'].into_iter().cycle().take(damaged_spring);
                //        let end = ['.']
                //            .into_iter()
                //            .cycle()
                //            .take(row.len() - start - damaged_spring);
                //        rows_match(beginning.chain(middle).chain(end), row.chars())
                //    })
                //    .count()
            }
        }
        n => {
            let split = n / 2;
            let lhs_damaged_springs = &damaged_springs[..split];
            let rhs_damaged_springs = &damaged_springs[split..];
            row.chars()
                .enumerate()
                .filter(|&(i, c)| c != '#')
                .map(|(i, _)| {
                    let lhs_row = &row[..i];
                    let lhs_arrangements = count_arrangements(lhs_row, lhs_damaged_springs);
                    if lhs_arrangements == 0 {
                        return 0;
                    }
                    let rhs_row = &row[(i + 1)..];
                    let rhs_arrangements = count_arrangements(rhs_row, rhs_damaged_springs);
                    let answer = lhs_arrangements * rhs_arrangements;
                    println!(
                        "{:?} + {:?} = {:?} | {:?} + {:?} = {:?} -> {}",
                        lhs_row,
                        lhs_damaged_springs,
                        lhs_arrangements,
                        rhs_row,
                        rhs_damaged_springs,
                        rhs_arrangements,
                        answer,
                    );
                    answer
                })
                .sum()
        }
    }
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
