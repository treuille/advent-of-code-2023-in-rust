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
use std::collections::{HashMap, HashSet};

/// The descriptor of a workflow
type Workflow = &'static str;

/// The types of a dimension in the puzzle space
type Dim = u16;

/// Stores the full tree of rules
struct Puzzle {}

/// A rule is part of a decision tree that either accepts or rejects parts
enum Rule {
    Accept,
    Reject,
    Split {
        axis: Axis,
        split: Dim,
        children: [Box<Rule>; 2],
    },
}

// Represents a volumne of space in the part lattice [1,4000]^4
struct Volume {
    min_max: [(Dim, Dim); 4],
    contents: Contents,
}

// A Volume is either empty, full, or split along an axis
enum Contents {
    Empty,
    Full,
    Split {
        axis: Axis,
        split: Dim,
        children: [Box<Volume>; 2],
    },
}

/// We are in a 4-dimensional space with axes labels as follows:
#[derive(Debug, Copy, Clone)]
enum Axis {
    X = 0,
    M,
    A,
    S,
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    //let input = include_str!("../../puzzle_inputs/day_19_test.txt");

    println!("input len: {}", input.len());
    figure_out_indices(input);

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

fn figure_out_indices(input: &'static str) {
    let (rules, parts) = input.split_once("\n\n").unwrap();
    //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)\}").unwrap();
    //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)").unwrap();
    println!("first line of rules: {}", rules.lines().next().unwrap());
    let workflow_regex = Regex::new(r"(\w+)\{(.+)\,(\w+)\}").unwrap();
    let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
    let rules: HashMap<Workflow, (Vec<(Axis, bool, Dim, Workflow)>, Workflow)> =
        parse_lines(workflow_regex, rules)
            .map(
                |(workflow, rules, fallback_workflow): (Workflow, &'static str, Workflow)| {
                    println!("workflow: {}", workflow);
                    println!("rules: {}", rules);
                    println!("fallback_workflow: {}", fallback_workflow);
                    let rules = rules
                        .split(",")
                        .map(|rule| {
                            let (axis, order, split, next_workflow): (char, char, Dim, Workflow) =
                                parse_line(&rule_regex, rule);
                            let axis = Axis::from_char(axis);
                            let (reverse_children, split) = match order {
                                '<' => (false, split),
                                '>' => (true, split + 1),
                                _ => panic!("Invalid order: {}", order),
                            };
                            println!("axis: {:?}", axis);
                            println!("reverse_children: {}", reverse_children);
                            println!("split: {}", split);
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
    let mut indices: [HashSet<Dim>; 4] = Default::default();
    for (rules, _fallback_workflow) in rules.values() {
        for (axis, _reverse_children, split, _next_workflow) in rules {
            indices[*axis as usize].insert(*split);
        }
    }
    for axis in Axis::iter() {
        println!("axis: {:?}", axis);
        println!(
            "indices: {:?}",
            indices[axis as usize].iter().sorted().collect_vec()
        );
    }
}

impl Puzzle {}

impl Rule {}

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
