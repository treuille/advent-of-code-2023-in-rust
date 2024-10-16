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
    let input = include_str!("../../puzzle_inputs/day_12.txt");
    //let input = include_str!("../../puzzle_inputs/day_12_test_1.txt");

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
    let correct_sol_12b: usize = 4964259839627;
    //let correct_sol_12b: usize = 525152;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

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

#[allow(clippy::iter_skip_zero)]
fn solve(puzzle: &[(Row, DamagedSprings)]) -> usize {
    let mut total_arrangements = 0;
    for (row, damaged_springs) in puzzle.iter().skip(0) {
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn rows_match(row_1: &str, row_2: &str) -> bool {
    //fn rows_match(
    //    row_1: impl IntoIterator<Item = char>,
    //    row_2: impl IntoIterator<Item = char>,
    //) -> bool {
    row_1
        .chars()
        .zip(row_2.chars())
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

// TODO: Add a short curcuit basd on the sum(damaged_springs)
#[allow(unused_variables, unreachable_code)]
fn count_arrangements(row: &str, damaged_springs: &[usize]) -> usize {
    //println!(
    //    "ENTRY\nrow: \"{}\"\ndamaged_springs: {:?}\n",
    //    row, damaged_springs
    //);

    if damaged_springs.is_empty() {
        if row.chars().all(|c| c != '#') {
            //println!("CASE A1 - return 1\n");
            1
        } else {
            //println!("CASE A2 - return 0\n");
            0
        }
    } else if row.chars().filter(|&c| c != '.').count() < damaged_springs.iter().sum() {
        //println!("CASE B - return 0\n");
        0
    } else {
        //println!(
        //    "CASE C\nrow: \"{}\"\ndamaged_springs: {:?}\n",
        //    row, damaged_springs
        //);
        let split = damaged_springs.len() / 2;
        assert!(split < damaged_springs.len());
        let left_damaged_springs = &damaged_springs[..split];
        let right_damaged_springs = &damaged_springs[(split + 1)..];

        //let split_spring = get_split_string(
        //    damaged_springs[split],
        //    left_damaged_springs.is_empty(),
        //    right_damaged_springs.is_empty(),
        //);

        //let mut split_spring = String::with_capacity(damaged_springs[split] + 2);
        let mut split_spring_len = damaged_springs[split];
        if !left_damaged_springs.is_empty() {
            //split_spring.push('.');
            split_spring_len += 1;
        }
        //split_spring.extend(std::iter::repeat('#').take(damaged_springs[split]));
        if !right_damaged_springs.is_empty() {
            //split_spring.push('.');
            split_spring_len += 1;
        }
        //let split_spring_len = split_spring.len();
        assert!(split_spring_len <= row.len());

        let mut pot_damaged: Vec<usize> = Vec::with_capacity(row.len());
        pot_damaged.extend(row.chars().scan(0, |acc, c| {
            if c != '.' {
                *acc += 1;
            }
            Some(*acc)
        }));
        let max_pot_damaged = pot_damaged.last().unwrap();
        let lds_sum: usize = left_damaged_springs.iter().sum();
        let rds_sum: usize = right_damaged_springs.iter().sum();
        (0..=row.len() - split_spring_len)
            //.filter(|&i| {
            //    ((i == 0) && (lds_sum == 0)) || ((i > 0) && (pot_damaged[i - 1] >= lds_sum))
            //})
            .filter(|&i| {
                (i == 0)
                    || ((i > 0)
                        && (pot_damaged[i + split_spring_len - 1] - pot_damaged[i - 1]
                            >= damaged_springs[split]))
            })
            .filter(|&i| (max_pot_damaged - pot_damaged[i + split_spring_len - 1]) >= rds_sum)
            .filter(|&i| {
                matches_row(
                    &row[i..],
                    damaged_springs[split],
                    left_damaged_springs.is_empty(),
                    right_damaged_springs.is_empty(),
                )
            })
            .map(|i| {
                let left_row = &row[..i];
                let left_arrangements = count_arrangements(left_row, left_damaged_springs);
                if left_arrangements == 0 {
                    return 0;
                }
                let right_row = &row[(i + split_spring_len)..];
                let right_arrangements = count_arrangements(right_row, right_damaged_springs);
                left_arrangements * right_arrangements
            })
            .sum()
    }
}

#[allow(dead_code, unreachable_code)]
fn matches_row(
    row: &str,
    damaged_spring: usize,
    left_springs_empty: bool,
    right_springs_empty: bool,
) -> bool {
    let mut row = row.chars();
    if !left_springs_empty && row.next() == Some('#') {
        //panic!("Case A\nrow: {:?}\ndamaged_spring: {}\nleft_springs_empty: {}\nright_springs_empty: {}\n", row, damaged_spring, left_springs_empty, right_springs_empty);
        return false;
    }
    for _ in 0..damaged_spring {
        match row.next() {
            Some('.') => {
                //panic!("Case B\nrow: {:?}\ndamaged_spring: {}\nleft_springs_empty: {}\nright_springs_empty: {}\n", row, damaged_spring, left_springs_empty, right_springs_empty);
                return false;
            }
            None => {
                //panic!("Case C\nrow: {:?}\ndamaged_spring: {}\nleft_springs_empty: {}\nright_springs_empty: {}\n", row, damaged_spring, left_springs_empty, right_springs_empty);
                return false;
            }
            _ => (),
        }
    }
    if !right_springs_empty && row.next() == Some('#') {
        //panic!("Case D\nrow: {:?}\ndamaged_spring: {}\nleft_springs_empty: {}\nright_springs_empty: {}\n", row, damaged_spring, left_springs_empty, right_springs_empty);
        return false;
    }
    true
    //if !left_springs_empty && row.chars().next() != Some('.') {
    //    return false;
    //}
    //row = &row[1..];
    //if row.chars().take(damaged_spring).any(|c| c == '.') {
    //    return false;
    //}
    //row = &row[damaged_spring..];
    //if !right_springs_empty && row.chars().next() != Some('.') {
    //    return false;
    //}
    //true
}

#[cached]
fn get_split_string(
    damaged_spring: usize,
    left_springs_empty: bool,
    right_springs_empty: bool,
) -> String {
    let mut split_spring = "#".repeat(damaged_spring);
    if !left_springs_empty {
        split_spring.insert(0, '.');
    }
    if !right_springs_empty {
        split_spring.push('.');
    }
    split_spring
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
