//#[allow(unused_imports)]
use cached::proc_macro::cached;
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

    // Solve 12b
    let puzzle = increase_puzzle_size(puzzle);
    let sol_12b: usize = solve(&puzzle);
    let correct_sol_12b: usize = 525152;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

#[cached]
fn damaged_spring_regex(damaged_spring: usize) -> Regex {
    let regex = format!(r"^[.?]*[#?]{{{damaged_spring}}}[.?]*$");
    println!("regex: {}", regex);
    return Regex::new(&regex).unwrap();
}

#[allow(dead_code)]
fn test_damaged_spring_regex() {
    let _regex = damaged_spring_regex(1);
    let _regex = damaged_spring_regex(1);
    let _regex = damaged_spring_regex(1);
    let _regex = damaged_spring_regex(2);
    let _regex = damaged_spring_regex(2);
    let regex = damaged_spring_regex(3);
    let test_patterns = [".#.", ".#.?", "?#.", "?#?", "?#.?", "?.?#.?", "?.##.?", "#"];
    for pattern in test_patterns {
        let is_match = regex.is_match(pattern);
        println!("pattern: {}, is_match: {}", pattern, is_match);
    }
}

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

#[allow(unused_variables)]
fn count_arrangements(row: &str, damaged_springs: &[usize]) -> usize {
    unimplemented!("count_arrangements")
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
