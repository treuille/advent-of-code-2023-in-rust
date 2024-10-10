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
    let re = Regex::new(pattern).unwrap();
    let network: HashMap<&str, (&str, &str)> = parse_regex::parse_lines(re, network.trim())
        .map(|(a, b, c)| (a, (b, c)))
        .collect();

    //// debug - begin
    //println!("instructions: {:?}", instructions);
    //println!("network:\n\"{:?}\"", network);
    //// debug - end

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

    solve_8b(&instructions, &network);
}

#[allow(unused_variables, unreachable_code)]
fn solve_8b(instructions: &[char], network: &HashMap<&str, (&str, &str)>) {
    // Test that instructions.len() is prime
    let is_prime = assert!(
        (2..instructions.len()).all(|i| instructions.len() % i != 0),
        "instructions.len() is not prime"
    );

    // Solve b - correct answer: ???
    println!("About to solve the second part of the puzzle");
    let initial_state: HashSet<&str> = network
        .keys()
        .copied()
        .filter(|node| node.ends_with("A"))
        .collect();
    println!("initial_state: {:?}", initial_state);

    // Find the cycle length for each node
    #[allow(clippy::never_loop)]
    for node in initial_state.iter() {
        println!("node: {}", node);

        // Create an endless iterator of states
        let initial_state = (*node, 0);
        println!("initial_state: {:?}", initial_state);
        let mut state_iter = std::iter::successors(Some(initial_state), |(node, instr_idx)| {
            let instruction = instructions.get(*instr_idx).unwrap();
            let instr_idx = (instr_idx + 1) % instructions.len();
            match instruction {
                'L' => Some((network[node].0, instr_idx)),
                'R' => Some((network[node].1, instr_idx)),
                _ => panic!("Invalid instruction: {}", instruction),
            }
        });
        //let mut state_iter = state_iter.skip(7);

        // Traverse this iterator until a cycle is found
        let mut visited_nodes: HashMap<(&str, usize), usize> = HashMap::new();
        //println!("state: {:?}", state);
        //panic!("Stop here");
        loop {
            let state = state_iter.next().unwrap();
            let maybe_path_len = visited_nodes.get(&state);
            if let Some(path_len) = maybe_path_len {
                println!(
                    "cycle length: {} after steps {}",
                    visited_nodes.len() - path_len,
                    path_len
                );
                println!(
                    "nodes ending in Z: {:?}",
                    visited_nodes
                        .iter()
                        .filter(|((node, _), _)| node.ends_with("Z"))
                        .collect::<Vec<_>>()
                );
                println!(
                    "min value: {} and max value: {}",
                    visited_nodes.values().min().unwrap(),
                    visited_nodes.values().max().unwrap(),
                );
                break;
            } else {
                visited_nodes.insert(state, visited_nodes.len());
            }
        }
    }
}
