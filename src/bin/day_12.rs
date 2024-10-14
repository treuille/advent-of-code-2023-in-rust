#[allow(unreachable_code, clippy::never_loop)]
fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12.txt");
    //let input = include_str!("../../puzzle_inputs/day_12_test_1.txt");
    ////println!("input len: {}", input.len());
    //println!("input:\n{}", input);
    //println!("parse_input(input): {:?}", parse_input(input));

    let puzzle = parse_input(input);

    // Solve 12a
    let sol_12a: usize = solve(&puzzle);
    let correct_sol_12a: usize = 7622;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    let puzzle = increase_puzzle_size(puzzle);

    //for (row, damaged_springs) in &puzzle {
    //    println!("row: {:?}", row);
    //    println!("damaged_springs: {:?}", damaged_springs);
    //    println!();
    //    break;
    //}
    //
    //unimplemented!("early stop here");

    // Solve 12b
    let sol_12b: usize = solve(&puzzle);
    let correct_sol_12b: usize = 525152;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

fn solve(puzzle: &[(Vec<char>, Vec<usize>)]) -> usize {
    let mut total_arrangements = 0;
    for (row, damaged_springs) in puzzle {
        println!("row: {:?}", row);
        println!("damaged_springs: {:?}", damaged_springs);
        let arrangements = count_arrangements(Some(row[0]), &row[1..], 0, damaged_springs);
        println!("arrangements: {}\n", arrangements);
        total_arrangements += arrangements;
    }
    println!("total_arrangements: {}", total_arrangements);
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

#[allow(unreachable_code)]
fn count_arrangements(
    first_spring: Option<char>,
    rest_of_springs: &[char],
    n_contiguous: usize,
    damaged_springs: &[usize],
) -> usize {
    let (next_spring, subsequent_springs): (Option<char>, &[char]) = rest_of_springs
        .split_first()
        .map_or((None, &[]), |(first_spring, rest_of_springs)| {
            (Some(*first_spring), rest_of_springs)
        });

    let matches_next_damaged_spring = Some(&n_contiguous) == damaged_springs.first();
    match (
        first_spring,
        n_contiguous,
        matches_next_damaged_spring,
        damaged_springs.len(),
    ) {
        (Some('.'), 0, _, _) => {
            count_arrangements(next_spring, subsequent_springs, 0, damaged_springs)
        }
        (Some('.'), _, true, _) => {
            count_arrangements(next_spring, subsequent_springs, 0, &damaged_springs[1..])
        }
        (Some('.'), _, _, _) => 0,
        (Some('#'), _, _, _) => count_arrangements(
            next_spring,
            subsequent_springs,
            n_contiguous + 1,
            damaged_springs,
        ),
        (Some('?'), _, true, _) => {
            count_arrangements(Some('.'), rest_of_springs, n_contiguous, damaged_springs)
        }
        (Some('?'), _, _, 0) => {
            count_arrangements(Some('.'), rest_of_springs, n_contiguous, damaged_springs)
        }
        (Some('?'), _, _, _) => {
            count_arrangements(Some('.'), rest_of_springs, n_contiguous, damaged_springs)
                + count_arrangements(Some('#'), rest_of_springs, n_contiguous, damaged_springs)
        }
        (None, 0, _, 0) => 1,
        (None, 0, _, _) => 0,
        (None, _, true, 1) => 1,
        (None, _, true, 0) => 0,
        (None, _, _, _) => 0,
        _ => panic!("Invalid spring character: {:?}", first_spring),
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
