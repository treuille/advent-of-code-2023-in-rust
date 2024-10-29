fn main() {
    //// Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    // Solve 17a
    let sol_17a: usize = 12;
    let correct_sol_17a: usize = 32;
    println!("* 17a *");
    println!("My solution: {sol_17a}");
    println!("Correct solution: {correct_sol_17a}");
    println!("Equal: {}\n", sol_17a == correct_sol_17a);

    // Solve 17b
    let sol_17b: usize = 56;
    let correct_sol_17b: usize = 79;
    println!("* 17b *");
    println!("My solution: {sol_17b}");
    println!("Correct solution: {correct_sol_17b}");
    println!("Equal: {}\n", sol_17b == correct_sol_17b);
}
