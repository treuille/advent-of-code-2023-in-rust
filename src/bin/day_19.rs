#![allow(
    unused_imports,
    dead_code,
    clippy::type_complexity,
    unused_variables,
    unreachable_code,
    clippy::four_forward_slashes,
    clippy::let_and_return,
    missing_docs,
    unused_doc_comments,
    clippy::only_used_in_recursion
)]

use advent_of_code_2023_in_rust::parse_regex::{parse_line, parse_lines};
use itertools::{izip, Itertools};
use num::CheckedSub;
use regex::Regex;
use std::array;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Formatter, Result};
use std::ops::Deref;
use std::rc::Rc;

// Cleanup plan:
// [ ] All other obvious removals
// [ ] Remove printlns
// [ ] Remove asserts
// [ ] Remove all the allow blocks
// [ ] Remove and add back in the derive blocks
// [ ] Remove comments

/// The descriptor of a workflow
type Workflow = &'static str;

/// We are in a 4-dimensional space with axes labels as follows:
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Axis {
    X = 0,
    M,
    A,
    S,
}

/// A rule is part of a decision tree that either accepts or rejects parts
enum Rule {
    Accept,
    Reject,
    Split {
        split_axis: Axis,
        split_pos: u64,
        children: [Rc<Rule>; 2],
    },
}

/// A Part is a point in the 4D lattice [1,4001)^4
#[derive(Debug)]
struct Part([u64; 4]);

type Rect = [(u64, u64); 4];

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    //let input = include_str!("../../puzzle_inputs/day_19_test.txt");
    let (puzzle, parts) = input.split_once("\n\n").unwrap();
    let rule = Rule::from_puzzle_str(puzzle);
    let parts = Part::from_str(parts);

    // Solve 19a
    let sol_19a: u64 = solve_part_a(&rule, &parts);
    let correct_sol_19a: u64 = 532551;
    println!("* 19a *");
    println!("My solution: {sol_19a}");
    println!("Correct solution: {correct_sol_19a}");
    println!("Equal: {:?}\n", sol_19a.cmp(&correct_sol_19a));

    // Solve 19b
    let sol_19b: u64 = solve_part_b(&rule);
    let correct_sol_19b: u64 = 134343280273968;
    println!("* 19b *");
    println!("My solution: {sol_19b}");
    println!("Correct solution: {correct_sol_19b}");
    println!("Equal: {:?}\n", sol_19b.cmp(&correct_sol_19b));
}

fn solve_part_a(rule: &Rc<Rule>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter_map(|part| {
            let accepts = rule.accepts_part(part);
            if accepts {
                Some(part.score())
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_b(rule: &Rc<Rule>) -> u64 {
    //println!("pos: {:?}", puzzle.pos[Axis::S as usize]);
    //
    //let [s2_left, s2_right] = Volume::halfspaces(&vol_full.bounds, Axis::S, 1);
    ////println!("s2_left:\n{:?}\n", s2_left);
    //println!("s2_left idx_volume: {}", s2_left.idx_volume());
    ////println!("s2_right:\n{:?}\n", s2_right);
    //println!("s2_right idx_volume: {}", s2_right.idx_volume());
    //
    //let [m2_left, m2_right] = Volume::halfspaces(&vol_full.bounds, Axis::M, 2);
    //println!("m2_left:\n{:?}\n", m2_left);
    //println!("m2_right:\n{:?}\n", m2_right);

    let rect: Rect = [(1, 4001); 4];
    //println!("rect: {:?}", rect);
    //println!("vol: {}", rect_vol(&rect));
    let vol_soln_2 = rule.accepts_vol(&rect);

    //let rule = Rc::new(Rule::Split {
    //    split_axis: Axis::X,
    //    split_idx: 1,
    //    children: [Rc::new(Rule::Accept), Rc::new(Rule::Accepnt)],
    //});
    //println!("pos: {:?}", puzzle.pos[Axis::X as usize][1]);
    //println!("vol: {}", puzzle.accepts_vol(&rect, &rule));
    //todo!("stop here");

    //let vol_soln = puzzle.rule.intersect(&vol_full, 0);

    //println!("vol_soln");
    //println!("- bounds: {:?}", vol_soln.bounds);
    //println!("- idx_volume: {:?}", vol_soln.idx_volume());
    //println!("- measure: {:?}", puzzle.measure(&vol_soln));
    //println!();
    //
    //let answer = puzzle.measure(&vol_soln);

    println!("solve_part_b - Answer: {}", vol_soln_2);
    vol_soln_2
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

impl Rule {
    /// Parse a string discripton of the puzzle
    fn from_puzzle_str(input: &'static str) -> Rc<Rule> {
        //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)\}").unwrap();
        //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)").unwrap();
        println!("first line of rules: {}", input.lines().next().unwrap());
        let workflow_regex = Regex::new(r"(\w+)\{(.+)\,(\w+)\}").unwrap();
        let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        let raw_rules: HashMap<Workflow, (Vec<(Axis, bool, u64, Workflow)>, Workflow)> =
            parse_lines(workflow_regex, input)
                .map(
                    |(workflow, rules, fallback_workflow): (Workflow, &'static str, Workflow)| {
                        let rules = rules
                            .split(",")
                            .map(|rule| {
                                let (axis, order, split, next_workflow): (
                                    char,
                                    char,
                                    u64,
                                    Workflow,
                                ) = parse_line(&rule_regex, rule);
                                let axis = Axis::from_char(axis);
                                let (reverse_children, split) = match order {
                                    '<' => (false, split),
                                    '>' => (true, split + 1),
                                    _ => panic!("Invalid order: {}", order),
                                };
                                (axis, reverse_children, split, next_workflow)
                            })
                            .collect();
                        //println!("rules: {:?}", rules);
                        (workflow, (rules, fallback_workflow))
                    },
                )
                .collect();

        // Pick up all of the indices for each axis
        let mut pos: [HashSet<u64>; 4] = array::from_fn(|_| [1, 4001].into());
        for (raw_rules, _fallback_workflow) in raw_rules.values() {
            for (axis, _reverse_children, split, _next_workflow) in raw_rules {
                pos[*axis as usize].insert(*split);
            }
        }
        let pos: [Vec<u64>; 4] = pos.map(|set| set.into_iter().sorted().collect());
        let pos_inv: [HashMap<u64, usize>; 4] = pos
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

        // debug - begin assert the that the inverses were computed correctly
        for axis in Axis::iter() {
            assert_eq!(pos[axis as usize].len(), pos_inv[axis as usize].len());
            for (index, &p) in pos[axis as usize].iter().enumerate() {
                assert_eq!(pos_inv[axis as usize][&p], index);
            }
        }
        // debug - end

        // Actualy contruct the puzzle data structure
        let mut rules: HashMap<Workflow, Rc<Rule>> =
            [("A", Rc::new(Rule::Accept)), ("R", Rc::new(Rule::Reject))].into();
        let rule = Rule::construct_rules("in", &raw_rules, &mut rules);

        rule
    }

    /// Returns true if the rule accepts the part
    fn accepts_part(&self, part: &Part) -> bool {
        match self {
            Rule::Accept => true,
            Rule::Reject => false,
            Rule::Split {
                split_axis,
                split_pos,
                children,
            } => {
                let part_pos = part.0[*split_axis as usize];
                let rule_pos = *split_pos;
                if part_pos < rule_pos {
                    children[0].accepts_part(part)
                } else {
                    children[1].accepts_part(part)
                }
            }
        }
    }

    /// Returns the volume of the subspace of the bounds that this rule accepts
    fn accepts_vol(&self, rect: &Rect) -> u64 {
        match self {
            Rule::Accept => rect
                .iter()
                .map(|&(min_pos, max_pos)| max_pos - min_pos)
                .product(),
            Rule::Reject => 0,
            Rule::Split {
                split_axis,
                split_pos,
                children,
            } => {
                let mut rect_left = *rect;
                let mut rect_right = *rect;
                rect_left[*split_axis as usize].1 = *split_pos;
                rect_right[*split_axis as usize].0 = *split_pos;

                let vol_left = children[0].accepts_vol(&rect_left);
                let vol_right = children[1].accepts_vol(&rect_right);
                vol_left + vol_right
            }
        }
    }

    /// Helper function for from_str to contruct puzzle.rules from a set of HashMaps
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow for which we will construct the rule
    /// * `pos_inv` - The inverse mapping of indices to their position along each axis
    /// * `raw_rules` - A raw (non-recursive) description of each rules
    /// * `rules` - The full set of rules that have been constructed so far
    ///
    /// # Returns
    ///
    /// The rule for the given workflow
    fn construct_rules(
        workflow: Workflow,
        raw_rules: &HashMap<Workflow, (Vec<(Axis, bool, u64, Workflow)>, Workflow)>,
        rules: &mut HashMap<Workflow, Rc<Rule>>,
    ) -> Rc<Rule> {
        match rules.get(workflow) {
            Some(rule) => Rc::clone(rule),
            None => {
                let (workflow_raw_rules, fallback_workflow) = raw_rules.get(workflow).unwrap();
                let mut rule =
                    Rc::clone(&Rule::construct_rules(fallback_workflow, raw_rules, rules));
                for (axis, reverse_children, split_pos, next_workflow) in
                    workflow_raw_rules.iter().rev()
                {
                    let child_rule = Rule::construct_rules(next_workflow, raw_rules, rules);
                    rule = Rc::new(Rule::Split {
                        split_axis: *axis,
                        split_pos: *split_pos,
                        children: if *reverse_children {
                            [rule, child_rule]
                        } else {
                            [child_rule, rule]
                        },
                    });
                }
                rules.insert(workflow, Rc::clone(&rule));
                rule
            }
        }
    }
}

impl Part {
    fn from_str(input: &'static str) -> Vec<Self> {
        let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        parse_lines(part_regex, input)
            .map(|(x_pos, m_pos, a_pos, s_pos)| Part([x_pos, m_pos, a_pos, s_pos]))
            .collect()
    }

    fn score(&self) -> u64 {
        self.0.iter().sum()
    }
}
