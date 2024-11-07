#![allow(unused_imports)]

use advent_of_code_2023_in_rust::parse_regex::parse_line;
use regex::Regex;

fn main() {
    //let input = include_str!("../../puzzle_inputs/day_19.txt");
    let input = include_str!("../../puzzle_inputs/day_19_test.txt");

    println!("input len: {}", input.len());

    // Solve 19a
    let sol_19a: usize = solve_part_a();
    let correct_sol_19a: usize = 19114;
    println!("* 19a *");
    println!("My solution: {sol_19a}");
    println!("Correct solution: {correct_sol_19a}");
    println!("Equal: {}\n", sol_19a == correct_sol_19a);

    // Solve 19b
    let sol_19b: usize = solve_part_b();
    let correct_sol_19b: usize = 167409079868000;
    println!("* 19b *");
    println!("My solution: {sol_19b}");
    println!("Correct solution: {correct_sol_19b}");
    println!("Equal: {:?}\n", sol_19b.cmp(&correct_sol_19b));
}

fn solve_part_a() -> usize {
    todo!("implement solve_part_a");
}

fn solve_part_b() -> usize {
    todo!("implement solve_part_b");
}
