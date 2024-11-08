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
use std::fmt::write;
use std::ops::Deref;
use std::rc::Rc;

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
    rule: Rc<Rule>,
}

/// A rule is part of a decision tree that either accepts or rejects parts
enum Rule {
    Accept,
    Reject,
    Split {
        axis: Axis,
        split_idx: usize,
        children: [Rc<Rule>; 2],
    },
}

/// A Part is a point in the 4D lattice [1,4000]^4
#[derive(Debug)]
struct Part([Pos; 4]);

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
        children: [Rc<Volume>; 2],
    },
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    //let input = include_str!("../../puzzle_inputs/day_19_test.txt");
    let (puzzle, parts) = input.split_once("\n\n").unwrap();
    let puzzle = Puzzle::from_str(puzzle);
    let parts = Part::from_str(parts);

    //println!("parts: {:?}", parts);
    //for part in parts {
    //    println!("part: {:?}", part);
    //    println!("- accepts: {:?}", puzzle.accepts(&puzzle.rule, &part));
    //}
    //
    ////puzzle.print_rules(&puzzle.rule, 0);
    //panic!("just printing the rules for now");

    //// debug - begin
    //println!(
    //    "puzzle pos [{} {} {} {}]",
    //    puzzle.pos[0].len(),
    //    puzzle.pos[1].len(),
    //    puzzle.pos[2].len(),
    //    puzzle.pos[3].len()
    //);
    //println!(
    //    "puzzle inv_pos [{} {} {} {}]",
    //    puzzle.pos_inv[0].len(),
    //    puzzle.pos_inv[1].len(),
    //    puzzle.pos_inv[2].len(),
    //    puzzle.pos_inv[3].len()
    //);
    //// debug - end

    // Solve 19a
    let sol_19a: u64 = solve_part_a(&puzzle, &parts);
    let correct_sol_19a: u64 = 19114;
    //let correct_sol_19a: u64 = 532551;
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

fn solve_part_a(puzzle: &Puzzle, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter_map(|part| {
            //println!(
            //    "part: [x={:?}, m={:?}, a={:?}, s={:?}]",
            //    part[Axis::X as usize],
            //    part[Axis::M as usize],
            //    part[Axis::A as usize],
            //    part[Axis::S as usize]
            //);
            let accepts = puzzle.accepts(&puzzle.rule, part);
            //println!("accepts: {:?}\n", accepts);
            if accepts {
                Some(part.score())
            } else {
                None
            }
        })
        .sum()
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
    /// Parse a string discripton of the puzzle
    fn from_str(input: &'static str) -> Self {
        //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)\}").unwrap();
        //let rule_regex = Regex::new(r"(\w+)\{((\w\,)+)(\w+)").unwrap();
        println!("first line of rules: {}", input.lines().next().unwrap());
        let workflow_regex = Regex::new(r"(\w+)\{(.+)\,(\w+)\}").unwrap();
        let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        let raw_rules: HashMap<Workflow, (Vec<(Axis, bool, Pos, Workflow)>, Workflow)> =
            parse_lines(workflow_regex, input)
                .map(
                    |(workflow, rules, fallback_workflow): (Workflow, &'static str, Workflow)| {
                        //println!("workflow: {}", workflow);
                        //println!("rules: {}", rules);
                        //println!("fallback_workflow: {}", fallback_workflow);
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
                                //println!("axis: {:?}", axis);
                                //println!("reverse_children: {}", reverse_children);
                                //println!("split: {:?}", split);
                                //println!("next_workflow: {}", next_workflow);
                                (axis, reverse_children, split, next_workflow)
                            })
                            .collect();
                        //println!("rules: {:?}", rules);
                        (workflow, (rules, fallback_workflow))
                    },
                )
                .collect();

        // Pick up all of the indices for each axis
        let mut pos: [HashSet<Pos>; 4] = array::from_fn(|_| [Pos(1), Pos(4001)].into());
        for (raw_rules, _fallback_workflow) in raw_rules.values() {
            for (axis, _reverse_children, split, _next_workflow) in raw_rules {
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

        // debug - begin assert the that the inverses were computed correctly
        //println!("pos: {:?}", pos);
        //println!("pos_inv: {:?}", pos_inv);
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
        let rule = Puzzle::construct_rules("in", &pos_inv, &raw_rules, &mut rules);

        Puzzle { pos, pos_inv, rule }
    }

    fn accepts(&self, rule: &Rc<Rule>, part: &Part) -> bool {
        match &**rule {
            Rule::Accept => true,
            Rule::Reject => false,
            Rule::Split {
                axis,
                split_idx,
                children,
            } => {
                let part_pos = part[*axis as usize];
                let rule_pos = self.pos[*axis as usize][*split_idx];
                //println!(
                //    "testing axis:{:?} part:{:?} < rule:{:?}",
                //    axis, part_pos, rule_pos
                //);
                if part_pos < rule_pos {
                    self.accepts(&children[0], part)
                } else {
                    self.accepts(&children[1], part)
                }
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
        pos_inv: &[HashMap<Pos, usize>],
        raw_rules: &HashMap<Workflow, (Vec<(Axis, bool, Pos, Workflow)>, Workflow)>,
        rules: &mut HashMap<Workflow, Rc<Rule>>,
    ) -> Rc<Rule> {
        match rules.get(workflow) {
            Some(rule) => Rc::clone(rule),
            None => {
                let (workflow_raw_rules, fallback_workflow) = raw_rules.get(workflow).unwrap();
                let mut rule = Rc::clone(&Puzzle::construct_rules(
                    fallback_workflow,
                    pos_inv,
                    raw_rules,
                    rules,
                ));
                for (axis, reverse_children, split_pos, next_workflow) in
                    workflow_raw_rules.iter().rev()
                {
                    let child_rule =
                        Puzzle::construct_rules(next_workflow, pos_inv, raw_rules, rules);
                    rule = Rc::new(Rule::Split {
                        axis: *axis,
                        split_idx: pos_inv[*axis as usize][split_pos],
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

    // Print out a recuring description of the puzzle rules
    fn print_rules(&self, rule: &Rc<Rule>, depth: usize) {
        let indent = "  ".repeat(depth);
        match &**rule {
            Rule::Accept => println!("{}Accept", indent),
            Rule::Reject => println!("{}Reject", indent),
            Rule::Split {
                axis,
                split_idx,
                children,
            } => {
                println!(
                    "{}if {:?} < {:?} (idx = {}):",
                    indent, axis, self.pos[*axis as usize][*split_idx], split_idx
                );
                self.print_rules(&children[0], depth + 1);
                println!("{}else:", indent);
                self.print_rules(&children[1], depth + 1);
            }
        }
    }
}

impl Rule {}

impl Part {
    fn from_str(input: &'static str) -> Vec<Self> {
        let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
        parse_lines(part_regex, input)
            .map(|(x_pos, m_pos, a_pos, s_pos)| {
                Part([Pos(x_pos), Pos(m_pos), Pos(a_pos), Pos(s_pos)])
            })
            .collect()
    }

    fn score(&self) -> u64 {
        self.0.iter().map(|pos| **pos as u64).sum()
    }
}

impl Deref for Part {
    type Target = [Pos; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Volume {}

impl Contents {}
