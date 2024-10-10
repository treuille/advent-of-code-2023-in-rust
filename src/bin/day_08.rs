use advent_of_code_2023_in_rust::parse_regex;
use regex::Regex;

#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[allow(unreachable_code, unused_variables, dead_code)]
fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_08.txt");
    //let input = include_str!("../../puzzle_inputs/day_08_test.txt");
    //let input = include_str!("../../puzzle_inputs/day_08_test_2.txt");
    //let input = include_str!("../../puzzle_inputs/day_08_test_3.txt");
    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let (instructions, network) = input.split_once("\n\n").unwrap();

    //dbg!(instructions);

    let pattern = r"(\w{3}) \= \((\w{3})\, (\w{3})\)";
    let re = Regex::new(pattern).unwrap();
    let map: Map = parse_regex::parse_lines(re, network.trim())
        .map(|(a, b, c)| (a, (b, c)))
        .collect();

    //let soln_8a = solve_8a(instructions, &map);
    //let correct_soln_8a: usize = 15989;
    //println!("soln_8a: {}", soln_8a);
    //println!("correct solution: {}", correct_soln_8a);

    //unimplemented!("TODO: Implement parsing of instructions");
    //

    solve_8b(instructions, &map);

    println!("Solved 8b");
}

#[allow(unused_variables, unreachable_code, dead_code)]
fn solve_8a(instructions: Instructions, map: &Map) -> usize {
    let initial_state = "AAA";
    instructions
        .chars()
        .cycle()
        .scan(initial_state, |node, instruction| {
            if *node == "ZZZ" {
                return None;
            }
            let next_node = match (instruction, map[node]) {
                ('L', (s, _)) => s,
                ('R', (_, s)) => s,
                _ => panic!("Invalid instruction: {}", instruction),
            };
            *node = next_node;
            Some(node.to_string())
        })
        .count()
}

#[allow(unused_variables, unreachable_code, dead_code)]
fn solve_8b(instructions: Instructions, map: &Map) {
    //dbg!(instructions);
    //dbg!(map);

    let initial_nodes: HashSet<Node> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|&node| node)
        .collect();

    dbg!(&initial_nodes);

    // pick one for now
    let initial_node = initial_nodes.iter().next().unwrap();

    let s = Sequence::new("AAA", instructions, map);

    println!("s: {:?}", s);
    //dbg!(s);

    //impl Sequence {
    //    fn new(starting_node: Node, instructions: Instructions, map: &Map) -> Self {

    unimplemented!("TODO: Implement solve_8b(..)")

    //let is_prime = assert!(
    //    (2..instructions.len()).all(|i| instructions.len() % i != 0),
    //    "instructions.len() is not prime"
    //);
    //
    //// Solve b - correct answer: ???
    //println!("About to solve the second part of the puzzle");
    //println!("initial_state: {:?}", initial_state);
    //
    //// Find the cycle length for each node
    //#[allow(clippy::never_loop)]
    //for node in initial_state.iter() {
    //    println!("node: {}", node);
    //
    //    }
    //}
}

/// Instrucitons is a sequence of characters that can be either 'L' or 'R'
type Instructions = &'static str;

/// A `Map` is a graph of nodes with two children, labeled 'L' or 'R'
type Map = HashMap<&'static str, (&'static str, &'static str)>;

type Node = &'static str;

/// A `State` is both a node on the graph as well as an instruction pointer
#[allow(dead_code)]
type State = (Node, usize);

/// `Sequence` is a widget that can take any usize and compute the
/// correspoinding state or any index 0..
#[allow(dead_code)]
#[derive(Debug)]
struct Sequence {}

#[allow(dead_code, unused_variables, unreachable_code)]
impl Sequence {
    fn new(initial_node: Node, instructions: Instructions, map: &Map) -> Self {
        // Our first state starts at the first instruction
        let initial_state: State = (*node, 0);
        dbg!(initial_state);

        // Create an endless iterator of states
        let mut state_iter = std::iter::successors(Some(initial_state), |(node, instr_ptr)| {
            let instruction = instructions.get(*instr_ptr).unwrap();
            let instr_ptr = (instr_ptr + 1) % instructions.len();
            match instruction {
                'L' => Some((network[node].0, instr_ptr)),
                'R' => Some((network[node].1, instr_ptr)),
                _ => panic!("Invalid instruction: {}", instruction),
            }
        });

        // Traverse this iterator until a cycle is found
        let mut visited_nodes: HashMap<State, usize> = HashMap::new();
        println!("state: {:?}", state);
        panic!("Stop here");
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
        unimplemented!("TODO: Implement Sequence::new()")
    }

    fn get(&self, index: usize) -> State {
        unimplemented!("TODO: Implement Sequence::get()")
    }
}
