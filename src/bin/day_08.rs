use advent_of_code_2023_in_rust::parse_regex;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_08.txt");
    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");
    //let input = include_str!("../../puzzle_inputs/day_08_test_2.txt");
    //let input = include_str!("../../puzzle_inputs/day_08_test_3.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let (instructions, network) = input.split_once("\n\n").unwrap();
    let instructions: Vec<char> = instructions.trim().chars().collect();

    //println!("network:\n\"{:?}\"", network);

    let pattern = r"(\w{3}) \= \((\w{3})\, (\w{3})\)";
    println!("pattern: \"{}\"", pattern);
    let re = Regex::new(pattern).unwrap();
    let network: HashMap<&str, (&str, &str)> = parse_regex::parse_lines(re, network.trim())
        .map(|(a, b, c)| (a, (b, c)))
        .collect();

    // debug - begin
    println!("instructions: {:?}", instructions);
    println!("network:\n\"{:?}\"", network);
    // debug - end

    //// Solve a - correct answer: 15989
    //println!("** Start Solve A **");
    ////let initial_state = "AAA";
    //let answer = instructions
    //    .iter()
    //    .cycle()
    //    .scan(initial_state, |node, instruction| {
    //        if *node == "ZZZ" {
    //            return None;
    //        }
    //        let next_node = match (instruction, network[node]) {
    //            ('L', (s, _)) => s,
    //            ('R', (_, s)) => s,
    //            _ => panic!("Invalid instruction: {}", instruction),
    //        };
    //        *node = next_node;
    //        Some(node.to_string())
    //    })
    //    //.take(10)
    //    //.map(|node| println!("{}", node))
    //    .count();
    //println!("answer: {}", answer);
    //println!("** End Solve A **");

    // Solve b - correct answer: ???
    println!("About to solve the second part of the puzzle");
    let initial_state: HashSet<&str> = network
        .keys()
        .copied()
        .filter(|node| node.ends_with("A"))
        .collect();
    println!("initial_state: {:?}", initial_state);
    let answer = instructions
        .iter()
        .cycle()
        .scan(initial_state, |nodes, instruction| {
            if nodes.iter().all(|node| node.ends_with("Z")) {
                return None;
            }
            //if *node == "ZZZ" {
            //    return None;
            //}
            *nodes = nodes
                .iter()
                .map(|node| match (instruction, network[node]) {
                    ('L', (s, _)) => s,
                    ('R', (_, s)) => s,
                    _ => panic!("Invalid instruction: {}", instruction),
                })
                .collect();
            //let next_node = match (instruction, network[node]) {
            //    ('L', (s, _)) => s,
            //    ('R', (_, s)) => s,
            //    _ => panic!("Invalid instruction: {}", instruction),
            //};
            //*node = next_node;
            Some(nodes.clone())
        })
        .enumerate()
        .map(|(i, node)| {
            if i % 1000000 == 0 {
                println!("iter {i}: {:?}", node)
            }
        })
        .count();
    println!("answer: {}", answer);
    //println!("Finished running through the instructions");
}
