fn main() {
    //// Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    // Solve 15a
    let sol_15a: usize = 12;
    let correct_sol_15a: usize = 32;
    println!("* 15a *");
    println!("My solution: {sol_15a}");
    println!("Correct solution: {correct_sol_15a}");
    println!("Equal: {}\n", sol_15a == correct_sol_15a);

    // Solve 15b
    let sol_15b: usize = 56;
    let correct_sol_15b: usize = 79;
    println!("* 15b *");
    println!("My solution: {sol_15b}");
    println!("Correct solution: {correct_sol_15b}");
    println!("Equal: {}\n", sol_15b == correct_sol_15b);
}
