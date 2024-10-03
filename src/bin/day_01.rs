//use std::{collections::HashMap, fmt::write};

//use std::fmt::write;

fn main() {
    // This is the input string
    let input = include_str!("../../puzzle_inputs/day_01.txt");

    // Solve puzzle 1
    let mut digits = vec![
        (String::from("0"), 0),
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
    ];
    let sol_1a = solve(input.lines(), &digits);
    let correct_sol_1a: usize = 54159;
    println!("* 1A *");
    println!("My solution: {sol_1a}");
    println!("Correct solution: {correct_sol_1a}");
    println!("Equal: {}\n", sol_1a == correct_sol_1a);

    // Puzzle 2 is the same as puzzle 1, but with spelled out digits
    digits.extend(vec![
        (String::from("zero"), 0),
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ]);
    let sol_1b = solve(input.lines(), &digits);
    let correct_sol_1b: usize = 53866;
    println!("* 1B *");
    println!("My solution: {sol_1b}");
    println!("Correct solution: {correct_sol_1b}");
    println!("Equal: {}\n", sol_1b == correct_sol_1b);
}

/// Returns the first digit (as defined in the `digits` map) found in the line
fn find_first_digit(line: &str, digits: &[(String, usize)]) -> usize {
    digits
        .iter()
        .min_by_key(|(digit_str, _)| line.find(digit_str).unwrap_or(usize::MAX))
        .unwrap()
        .1
}

/// Solves the puzzle relative to a particuar digit map
fn solve<'a>(lines: impl Iterator<Item = &'a str>, digits: &[(String, usize)]) -> usize {
    // Reverse a string
    let rev = |s: &str| s.chars().rev().collect::<String>();

    // Create a reverse digit map
    let rev_digits: Vec<(String, usize)> = digits
        .iter()
        .map(|(digit_str, digit)| (rev(digit_str), *digit))
        .collect();

    // Iterate through the lines, find the first and last digit, and sum them
    lines
        .map(|line| {
            let first_digit = find_first_digit(line, digits);
            let last_digit = find_first_digit(&rev(line), &rev_digits);
            first_digit * 10 + last_digit
        })
        .sum()
}
