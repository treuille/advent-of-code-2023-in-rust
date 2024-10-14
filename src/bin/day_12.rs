fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12_test_1.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);
    println!("parse_input(input): {:?}", parse_input(input));

    // Solve 12a
    let sol_12a: usize = 12;
    let correct_sol_12a: usize = 32;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    // Solve 12b
    let sol_12b: usize = 56;
    let correct_sol_12b: usize = 79;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

fn parse_input(input: &'static str) -> Vec<(&str, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (row, damaged_springs) = line.split_once(" ").unwrap();
            let damages_springs = damaged_springs.split(",").flat_map(str::parse).collect();
            (row, damages_springs)
        })
        .collect()
}
