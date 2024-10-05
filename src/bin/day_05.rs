fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_05_test.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    let lines = input.lines();
    let seeds = lines.next().unwrap().split_whitespace();

    // Solve 5a
    let sol_5a: usize = 12;
    let correct_sol_5a: usize = 32;
    println!("* 5a *");
    println!("My solution: {sol_5a}");
    println!("Correct solution: {correct_sol_5a}");
    println!("Equal: {}\n", sol_5a == correct_sol_5a);

    // Solve 5b
    let sol_5b: usize = 56;
    let correct_sol_5b: usize = 79;
    println!("* 5b *");
    println!("My solution: {sol_5b}");
    println!("Correct solution: {correct_sol_5b}");
    println!("Equal: {}\n", sol_5b == correct_sol_5b);
}
