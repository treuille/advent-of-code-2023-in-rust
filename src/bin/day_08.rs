use advent_of_code_2023_in_rust::parse_regex;
use num::integer::gcd;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    // Parse the input into a set of `instructions` and a `map` of nodes
    let input = include_str!("../../puzzle_inputs/day_08.txt");
    let puzzle = Puzzle::new(input);

    // Solve 8a
    let sol_8a = solve_8a(&puzzle);
    let correct_sol_8a: usize = 15989;
    println!("* 8a *");
    println!("My solution: {sol_8a}");
    println!("Correct solution: {correct_sol_8a}");
    println!("Equal: {}\n", sol_8a == correct_sol_8a);

    // Solve 8b
    let sol_8b: usize = solve_8b(&puzzle);
    let correct_sol_8b: usize = 13830919117339;
    println!("* 8b *");
    println!("My solution: {sol_8b}");
    println!("Correct solution: {correct_sol_8b}");
    println!("Equal: {}\n", sol_8b == correct_sol_8b);
}

fn solve_8a(puzzle: &Puzzle) -> usize {
    let initial_nodes = ["AAA"].into_iter().collect();
    let terminal_nodes = ["ZZZ"].into_iter().collect();
    puzzle.solve(&initial_nodes, &terminal_nodes)
}

fn solve_8b(puzzle: &Puzzle) -> usize {
    let initial_nodes: HashSet<&'static str> = puzzle
        .map
        .keys()
        .filter(|node| node.ends_with("A"))
        .copied()
        .collect();
    let terminal_nodes: HashSet<&'static str> = puzzle
        .map
        .keys()
        .filter(|node| node.ends_with("Z"))
        .copied()
        .collect();
    puzzle.solve(&initial_nodes, &terminal_nodes)
}

struct Puzzle {
    instructions: Vec<char>,
    map: HashMap<&'static str, (&'static str, &'static str)>,
}

impl Puzzle {
    fn new(input: &'static str) -> Self {
        let (instructions, map) = input.split_once("\n\n").unwrap();
        let instructions: Vec<char> = instructions.chars().collect();

        let pattern = r"(\w{3}) \= \((\w{3})\, (\w{3})\)";
        let re = Regex::new(pattern).unwrap();
        let map = parse_regex::parse_lines(re, map.trim())
            .map(|(a, b, c)| (a, (b, c)))
            .collect();

        Self { instructions, map }
    }

    /// Solve the puzzle
    fn solve(
        &self,
        initial_nodes: &HashSet<&'static str>,
        terminal_nodes: &HashSet<&'static str>,
    ) -> usize {
        // Least common multiple of two numbers
        let lcm = |a, b| (a * b) / gcd(a, b);

        let mut step = 0;
        let mut step_len = 1;
        for &node in initial_nodes.iter() {
            let path = Path::starting_from(node, self);
            while !terminal_nodes.contains(path.at_step(step)) {
                step += step_len;
            }
            step_len = lcm(step_len, path.cycle_len());
        }
        step
    }
}

/// A `Path` is a sequence of nodes each of which can be computed by index in costant time
/// using path.at_step(index).
struct Path {
    /// A path is a sequence of nodes, which cycle after `cycle_start` steps.
    nodes: Vec<&'static str>,

    /// Index of the first node in the cycle
    cycle_start: usize,
}

impl Path {
    /// Construct a path from an `initial_node` by following the instructions
    /// found in `puzzle`.
    fn starting_from(initial_node: &'static str, puzzle: &Puzzle) -> Path {
        // Traverse this iterator until a cycle is found
        let mut node = initial_node;
        let mut instr_ptr = 0;
        let mut nodes = Vec::new();
        let mut visited_states = HashMap::new();
        loop {
            let state = (node, instr_ptr);
            if let Some(&cycle_start) = visited_states.get(&state) {
                return Path { cycle_start, nodes };
            }
            nodes.push(node);
            visited_states.insert(state, visited_states.len());
            node = match puzzle.instructions.get(instr_ptr).unwrap() {
                'L' => puzzle.map[node].0,
                'R' => puzzle.map[node].1,
                instruction => panic!("Invalid instruction: {}", instruction),
            };
            instr_ptr = (instr_ptr + 1) % puzzle.instructions.len();
        }
    }

    fn at_step(&self, index: usize) -> &'static str {
        let index = index
            .checked_sub(self.cycle_start)
            .map_or(index, |rel_index| {
                self.cycle_start + rel_index % self.cycle_len()
            });
        self.nodes[index]
    }

    fn cycle_len(&self) -> usize {
        self.nodes.len() - self.cycle_start
    }
}
