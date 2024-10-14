fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12.txt");
    //let input = include_str!("../../puzzle_inputs/day_12_test_1.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);
    println!("parse_input(input): {:?}", parse_input(input));

    let mut total_arrangements = 0;
    for (row, damaged_springs) in parse_input(input) {
        println!("row: {:?}", row);
        println!("damaged_springs: {:?}", damaged_springs);
        let arrangements = count_arrangements(Some(row[0]), &row[1..], 0, &damaged_springs);
        println!("arrangements: {}\n", arrangements);
        total_arrangements += arrangements;
    }
    println!("total_arrangements: {}", total_arrangements);
    //// Solve 12a
    //let sol_12a: usize = 12;
    //let correct_sol_12a: usize = 32;
    //println!("* 12a *");
    //println!("My solution: {sol_12a}");
    //println!("Correct solution: {correct_sol_12a}");
    //println!("Equal: {}\n", sol_12a == correct_sol_12a);
    //
    //// Solve 12b
    //let sol_12b: usize = 56;
    //let correct_sol_12b: usize = 79;
    //println!("* 12b *");
    //println!("My solution: {sol_12b}");
    //println!("Correct solution: {correct_sol_12b}");
    //println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

#[allow(unreachable_code)]
fn count_arrangements(
    first_spring: Option<char>,
    rest_of_springs: &[char],
    n_contiguous: usize,
    damaged_springs: &[usize],
) -> usize {
    //println!("first_spring: {:?}", first_spring);
    //println!("rest_of_springs: {:?}", rest_of_springs);
    //println!("n_contiguous: {}", n_contiguous);
    //println!("damaged_springs: {:?}", damaged_springs);

    let (next_spring, subsequent_springs): (Option<char>, &[char]) = rest_of_springs
        .split_first()
        .map_or((None, &[]), |(first_spring, rest_of_springs)| {
            (Some(*first_spring), rest_of_springs)
        });

    //println!("next_spring: {:?}", next_spring);
    //println!("subsequent_springs: {:?}", subsequent_springs);

    match first_spring {
        Some('.') => {
            if n_contiguous == 0 {
                count_arrangements(next_spring, subsequent_springs, 0, damaged_springs)
            } else if Some(&n_contiguous) == damaged_springs.first() {
                count_arrangements(next_spring, subsequent_springs, 0, &damaged_springs[1..])
            } else {
                0
            }
        }
        Some('#') => count_arrangements(
            next_spring,
            subsequent_springs,
            n_contiguous + 1,
            damaged_springs,
        ),
        Some('?') => {
            count_arrangements(Some('.'), rest_of_springs, n_contiguous, damaged_springs)
                + count_arrangements(Some('#'), rest_of_springs, n_contiguous, damaged_springs)
        }
        None => {
            if n_contiguous == 0 {
                if damaged_springs.is_empty() {
                    1
                } else {
                    0
                }
            } else if Some(&n_contiguous) == damaged_springs.first() {
                if damaged_springs.len() == 1 {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        }
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
