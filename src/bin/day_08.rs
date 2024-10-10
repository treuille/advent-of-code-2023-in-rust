use advent_of_code_2023_in_rust::parse_regex;
use num::integer::gcd;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_08.txt");
    let (instructions, map) = parse_input(input);

    // Solve 8a
    let sol_8a = solve_8a(&instructions, &map);
    let correct_sol_8a: usize = 15989;
    println!("* 8a *");
    println!("My solution: {sol_8a}");
    println!("Correct solution: {correct_sol_8a}");
    println!("Equal: {}\n", sol_8a == correct_sol_8a);

    // Solve 8b
    let sol_8b: usize = solve_8b(&instructions, &map);
    let correct_sol_8b: usize = 13830919117339;
    println!("* 8b *");
    println!("My solution: {sol_8b}");
    println!("Correct solution: {correct_sol_8b}");
    println!("Equal: {}\n", sol_8b == correct_sol_8b);
    println!("Solved 8b");
}

fn solve_8a(instructions: &[Instruction], map: &Map) -> usize {
    let initial_state = "AAA";
    instructions
        .iter()
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

fn solve_8b(instructions: &[Instruction], map: &Map) -> usize {
    let initial_nodes: HashSet<Node> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .copied()
        .collect();

    // Least common multiple of two numbers
    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    let mut step = 0;
    let mut step_len = 1;
    for &node in initial_nodes.iter() {
        let s = Sequence::new(node, instructions, map);
        while !s.get(step).0.ends_with("Z") {
            step += step_len;
        }
        step_len = lcm(step_len, s.cycle_len());
    }
    step
}

/// Instrucitons is a sequence of characters that can be either 'L' or 'R'
type Instruction = char;

/// A `Map` is a graph of nodes with two children, labeled 'L' or 'R'
type Map = HashMap<&'static str, (&'static str, &'static str)>;

type Node = &'static str;

/// A `State` is both a node on the graph as well as an instruction pointer
type State = (Node, usize);

/// Turns `intial_node` into an sequence of states, cyclically
/// following the `instructions` through the `map`
fn state_iter<'a>(
    initial_node: Node,
    instructions: &'a [Instruction],
    map: &'a Map,
) -> impl Iterator<Item = State> + 'a {
    let initial_state: State = (initial_node, 0);
    std::iter::successors(Some(initial_state), |(node, instr_ptr)| {
        let instruction = instructions.get(*instr_ptr).unwrap();
        let instr_ptr = (instr_ptr + 1) % instructions.len();
        match instruction {
            'L' => Some((map[node].0, instr_ptr)),
            'R' => Some((map[node].1, instr_ptr)),
            _ => panic!("Invalid instruction: {}", instruction),
        }
    })
}

/// `Sequence` is a widget that can take any usize and compute the
/// correspoinding state or any index 00..
#[allow(dead_code)]
#[derive(Debug)]
struct Sequence {
    cycle_start: usize,
    states: Vec<State>,
}

#[allow(dead_code, unused_variables, unreachable_code)]
impl Sequence {
    #[allow(unused_mut)]
    fn new(initial_node: Node, instructions: &[Instruction], map: &Map) -> Self {
        // Our first state starts at the first instruction
        let initial_state: State = (initial_node, 00);

        // Create an endless iterator of states
        let mut state_iter = state_iter(initial_node, instructions, map);

        // Traverse this iterator until a cycle is found
        let mut states: Vec<State> = Vec::new();
        let mut visited_nodes: HashMap<State, usize> = HashMap::new();
        loop {
            let state = state_iter.next().unwrap();
            if let Some(&cycle_start) = visited_nodes.get(&state) {
                return Sequence {
                    cycle_start,
                    states,
                };
            } else {
                states.push(state);
                visited_nodes.insert(state, visited_nodes.len());
            }
        }
    }

    fn get(&self, index: usize) -> State {
        let index = index
            .checked_sub(self.cycle_start)
            .map_or(index, |rel_index| {
                self.cycle_start + rel_index % self.cycle_len()
            });
        self.states[index]
    }

    fn cycle_len(&self) -> usize {
        self.states.len() - self.cycle_start
    }
}

fn parse_input(input: &'static str) -> (Vec<Instruction>, Map) {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = instructions.chars().collect();

    let pattern = r"(\w{3}) \= \((\w{3})\, (\w{3})\)";
    let re = Regex::new(pattern).unwrap();
    let map: Map = parse_regex::parse_lines(re, map.trim())
        .map(|(a, b, c)| (a, (b, c)))
        .collect();

    (instructions, map)
}
