#![allow(
    unused_imports,
    dead_code,
    clippy::type_complexity,
    unused_variables,
    unreachable_code
)]

use advent_of_code_2023_in_rust::parse_regex::{parse_line, parse_lines};
use itertools::Itertools;
use regex::Regex;
use std::array;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

/// The descriptor of a workflow
type Workflow = &'static str;

/// We are in a 4-dimensional space with axes labels as follows:
#[derive(Debug, Copy, Clone)]
enum Axis {
    X = 0,
    M,
    A,
    S,
}

/// A position along a dimension
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(u16);

/// Stores the full tree of rules
struct Puzzle {
    /// Ordered list of indices along each axis
    pos: [Vec<Pos>; 4],

    /// Inverse mapping of indices to their position in the ordered list
    pos_inv: [HashMap<Pos, usize>; 4],

    /// The root rule of the puzzle
    rule: Rule,
}

/// A Part is a point in the 4D lattice [1,4000]^4
struct Part([Pos; 4]);

/// A rule is part of a decision tree that either accepts or rejects parts
enum Rule {
    Accept,
    Reject,
    Split {
        axis: Axis,
        split: usize,
        children: [Box<Rule>; 2],
    },
}

// Represents a volumne of space in the part lattice [1,4000]^4
struct Volume {
    min_max: [(usize, usize); 4],
    contents: Contents,
}

// A Volume is either empty, full, or split along an axis
enum Contents {
    Empty,
    Full,
    Split {
        axis: Axis,
        split: usize,
        children: [Box<Volume>; 2],
    },
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    //let input = include_str!("../../puzzle_inputs/day_19_test.txt");
    let (puzzle, parts) = input.split_once("\n\n").unwrap();
    let puzzle = Puzzle::from_str(puzzle);

    // debug - begin
    println!(
        "puzzle pos [{} {} {} {}]",
        puzzle.pos[0].len(),
        puzzle.pos[1].len(),
        puzzle.pos[2].len(),
        puzzle.pos[3].len()
    );
    println!(
        "puzzle inv_pos [{} {} {} {}]",
        puzzle.pos_inv[0].len(),
        puzzle.pos_inv[1].len(),
        puzzle.pos_inv[2].len(),
        puzzle.pos_inv[3].len()
    );
    // debug - end

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

/// There are four axes in this puzzle, charmingly named 'x', 'm', 'a', and 's'.
impl Axis {
    /// Returns an iterator over the axis variants
    fn iter() -> impl Iterator<Item = Axis> {
        [Axis::X, Axis::M, Axis::A, Axis::S].into_iter()
    }

    /// Parses an axis from one of the characters: 'x', 'm', 'a', 's'
    fn from_char(c: char) -> Self {
        match c {
            'x' => Axis::X,
            'm' => Axis::M,
            'a' => Axis::A,
            's' => Axis::S,
            _ => panic!("Invalid axis: {}", c),
        }
    }
}

impl Pos {}

impl Deref for Pos {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Puzzle {
    fn from_str(input: &'static str) -> Self {
        //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)\}").unwrap();
        //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)").unwrap();
        println!("first line of rules: {}", input.lines().next().unwrap());
        let workflow_regex = Regex::new(r"(\w+)\{(.+)\,(\w+)\}").unwrap();
        let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        let rules: HashMap<Workflow, (Vec<(Axis, bool, Pos, Workflow)>, Workflow)> =
            parse_lines(workflow_regex, input)
                .map(
                    |(workflow, rules, fallback_workflow): (Workflow, &'static str, Workflow)| {
                        println!("workflow: {}", workflow);
                        println!("rules: {}", rules);
                        println!("fallback_workflow: {}", fallback_workflow);
                        let rules = rules
                            .split(",")
                            .map(|rule| {
                                let (axis, order, split, next_workflow): (
                                    char,
                                    char,
                                    u16,
                                    Workflow,
                                ) = parse_line(&rule_regex, rule);
                                let axis = Axis::from_char(axis);
                                let (reverse_children, split) = match order {
                                    '<' => (false, Pos(split)),
                                    '>' => (true, Pos(split + 1)),
                                    _ => panic!("Invalid order: {}", order),
                                };
                                println!("axis: {:?}", axis);
                                println!("reverse_children: {}", reverse_children);
                                println!("split: {:?}", split);
                                println!("next_workflow: {}", next_workflow);
                                (axis, reverse_children, split, next_workflow)
                            })
                            .collect();
                        println!("rules: {:?}", rules);
                        (workflow, (rules, fallback_workflow))
                    },
                )
                .collect();

        // Pick up all of the indices for each axis
        let mut pos: [HashSet<Pos>; 4] = array::from_fn(|_| [Pos(1), Pos(4001)].into());
        for (rules, _fallback_workflow) in rules.values() {
            for (axis, _reverse_children, split, _next_workflow) in rules {
                pos[*axis as usize].insert(*split);
            }
        }
        let pos: [Vec<Pos>; 4] = pos.map(|set| set.into_iter().sorted().collect());
        let pos_inv: [HashMap<Pos, usize>; 4] = pos
            .iter()
            .map(|vec| {
                vec.iter()
                    .enumerate()
                    .map(|(index, &p)| (p, index))
                    .collect()
            })
            .collect_vec()
            .try_into()
            .unwrap();

        println!("pos: {:?}", pos);
        println!("pos_inv: {:?}", pos_inv);

        // debug - begin assert the that the inverses were computed correctly
        for axis in Axis::iter() {
            assert_eq!(pos[axis as usize].len(), pos_inv[axis as usize].len());
            for (index, &p) in pos[axis as usize].iter().enumerate() {
                assert_eq!(pos_inv[axis as usize][&p], index);
            }
        }
        // debug - end
        todo!("Need to construct the puzzle");
    }
}

impl Part {}

impl Deref for Part {
    type Target = [Pos; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rule {}

impl Volume {}

impl Contents {}
