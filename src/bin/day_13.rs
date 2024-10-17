fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    // Solve 13a
    let sol_13a: usize = 12;
    let correct_sol_13a: usize = 32;
    println!("* 13a *");
    println!("My solution: {sol_13a}");
    println!("Correct solution: {correct_sol_13a}");
    println!("Equal: {}\n", sol_13a == correct_sol_13a);

    // Solve 13b
    let sol_13b: usize = 56;
    let correct_sol_13b: usize = 79;
    println!("* 13b *");
    println!("My solution: {sol_13b}");
    println!("Correct solution: {correct_sol_13b}");
    println!("Equal: {}\n", sol_13b == correct_sol_13b);
}
