fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_10_test.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    // Solve 10a
    let sol_10a: usize = 12;
    let correct_sol_10a: usize = 32;
    println!("* 10a *");
    println!("My solution: {sol_10a}");
    println!("Correct solution: {correct_sol_10a}");
    println!("Equal: {}\n", sol_10a == correct_sol_10a);

    // Solve 10b
    let sol_10b: usize = 56;
    let correct_sol_10b: usize = 79;
    println!("* 10b *");
    println!("My solution: {sol_10b}");
    println!("Correct solution: {correct_sol_10b}");
    println!("Equal: {}\n", sol_10b == correct_sol_10b);
}
