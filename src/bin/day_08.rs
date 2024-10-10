use advent_of_code_2023_in_rust::parse_regex;
use num::integer::gcd;
use regex::Regex;

#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::iter::Cycle;

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
    let instructions: Vec<Instruction> = instructions.chars().collect();

    //dbg!(instructions);

    let pattern = r"(\w{3}) \= \((\w{3})\, (\w{3})\)";
    let re = Regex::new(pattern).unwrap();
    let map: Map = parse_regex::parse_lines(re, network.trim())
        .map(|(a, b, c)| (a, (b, c)))
        .collect();

    let soln_8a = solve_8a(&instructions, &map);
    let correct_soln_8a: usize = 15989;
    println!("soln_8a: {}", soln_8a);
    println!("correct solution: {}", correct_soln_8a);

    //unimplemented!("TODO: Implement parsing of instructions");
    //

    solve_8b(&instructions, &map);

    println!("Solved 8b");
}

#[allow(unused_variables, unreachable_code, dead_code)]
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

#[allow(unused_variables, unreachable_code, dead_code)]
fn solve_8b(instructions: &[Instruction], map: &Map) {
    //dbg!(instructions);
    //dbg!(map);

    let initial_nodes: HashSet<Node> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|&node| node)
        .collect();

    dbg!(&initial_nodes);

    //// pick one for now
    //let initial_node = initial_nodes.iter().next().unwrap();
    //
    //let initial_node = "AAA";
    //let s = Sequence::new(initial_node, instructions, map);
    //
    //println!("** s **");
    //println!("cycle_start: {}", s.cycle_start);
    //println!("num states: {}", s.states.len());
    //println!("first state: {:?}", s.states.first().unwrap());
    //println!("cycle start state: {:?}", s.states[s.cycle_start]);
    //println!("last state: {:?}", s.states.last().unwrap());
    //println!();
    //
    //let state_at = |index: usize| -> State {
    //    state_iter(initial_node, instructions, map)
    //        .nth(index)
    //        .unwrap()
    //};
    //
    //println!("iter first state: {:?}", state_at(0));
    //println!("iter cycle_start state: {:?}", state_at(s.cycle_start));
    //println!("iter last state: {:?}", state_at(s.states.len() - 1));
    //println!("iter subsequent state: {:?}", state_at(s.states.len()));
    //println!("iter 1000000 state: {:?}", state_at(1000000));
    //println!("iter 2000000 state: {:?}", state_at(2000000));
    //println!();
    //
    //println!("get first state: {:?}", s.get(0));
    //println!("get cycle_start state: {:?}", s.get(s.cycle_start));
    //println!("get last state: {:?}", s.get(s.states.len() - 1));
    //println!("get subsequent state: {:?}", s.get(s.states.len()));
    //println!("get 1000000 state: {:?}", s.get(1000000));
    //println!("get 2000000 state: {:?}", s.get(2000000));

    /// Least common multiple of two numbers
    fn lcm(a: usize, b: usize) -> usize {
        (a * b) / gcd(a, b)
    }

    let mut step = 0;
    let mut step_len = 1;
    for &node in initial_nodes.iter() {
        println!("probing node: {}", node);
        let s = Sequence::new(node, instructions, map);
        while !s.get(step).0.ends_with("Z") {
            step += step_len;
        }
        println!("found {} at step {}", s.get(step).0, step);
        step_len = lcm(step_len, s.cycle_len());
    }
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
type Instruction = char;

/// A `Map` is a graph of nodes with two children, labeled 'L' or 'R'
type Map = HashMap<&'static str, (&'static str, &'static str)>;

type Node = &'static str;

/// A `State` is both a node on the graph as well as an instruction pointer
#[allow(dead_code)]
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
        dbg!(initial_state);

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
