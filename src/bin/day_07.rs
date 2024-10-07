fn main() {
    //// Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_07_test.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);
    //
    //let lines = input.lines();
    //let seeds = lines.next().unwrap().split_whitespace();

    // Solve 7a
    let sol_7a: usize = 12;
    let correct_sol_7a: usize = 32;
    println!("* 7a *");
    println!("My solution: {sol_7a}");
    println!("Correct solution: {correct_sol_7a}");
    println!("Equal: {}\n", sol_7a == correct_sol_7a);

    // Solve 7b
    let sol_7b: usize = 56;
    let correct_sol_7b: usize = 79;
    println!("* 7b *");
    println!("My solution: {sol_7b}");
    println!("Correct solution: {correct_sol_7b}");
    println!("Equal: {}\n", sol_7b == correct_sol_7b);
}
