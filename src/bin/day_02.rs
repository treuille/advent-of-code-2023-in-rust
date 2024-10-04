//use std::{collections::HashMap, fmt::write};

//use std::fmt::write;

fn main() {
    // This is the input string
    let input = include_str!("../../puzzle_inputs/day_02.txt");

    println!("input length: {}", input.len());
    println!("input:\n{}", input);

    // Dummy solution to 2a
    let sol_2a = 123;
    let correct_sol_2a: usize = 456;
    println!("* 2A *");
    println!("My solution: {sol_2a}");
    println!("Correct solution: {correct_sol_2a}");
    println!("Equal: {}\n", sol_2a == correct_sol_2a);

    // Dummy solution to 2a
    let sol_2b = 78;
    let correct_sol_2b: usize = 92;
    println!("* 2B *");
    println!("My solution: {sol_2b}");
    println!("Correct solution: {correct_sol_2b}");
    println!("Equal: {}\n", sol_2b == correct_sol_2b);
}
