use advent_of_code_2023_in_rust::parse_regex::{parse_line, parse_lines};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

// Cleanup plan:
// [ ] Simplify `Part`: type Part = [Dim; 4];
// [ ] Remove and add back in the derive blocks
// [ ] Remove comments

/// The descriptor of a workflow
type Workflow = &'static str;

/// The type of points along each of the axes of the 4D lattice
type Dim = u64;

/// We index the into 4 axes using `usize`
type Axis = usize;

/// A Part is a point in the 4D lattice [1,4001)^4
#[derive(Debug)]
struct Part([Dim; 4]);

type Rect = [(Dim, Dim); 4];

/// A rule is part of a decision tree that either accepts or rejects parts
enum Rule {
    Accept,
    Reject,
    Split {
        split_axis: Axis,
        split_pos: Dim,
        children: [Rc<Rule>; 2],
    },
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    let (puzzle, parts) = input.split_once("\n\n").unwrap();
    let rule = Rule::from_puzzle_str(puzzle);
    let parts = parts_from_str(parts);

    // Solve 19a
    let sol_19a: Dim = solve_part_a(&rule, &parts);
    let correct_sol_19a: Dim = 532551;
    println!("* 19a *");
    println!("My solution: {sol_19a}");
    println!("Correct solution: {correct_sol_19a}");
    println!("Equal: {:?}\n", sol_19a.cmp(&correct_sol_19a));

    // Solve 19b
    let sol_19b: Dim = solve_part_b(&rule);
    let correct_sol_19b: Dim = 134343280273968;
    println!("* 19b *");
    println!("My solution: {sol_19b}");
    println!("Correct solution: {correct_sol_19b}");
    println!("Equal: {:?}\n", sol_19b.cmp(&correct_sol_19b));
}

fn solve_part_a(rule: &Rc<Rule>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter_map(|part| {
            if rule.accepts_part(part) {
                let part_score: u64 = part.0.iter().sum();
                Some(part_score)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_b(rule: &Rc<Rule>) -> Dim {
    let full_space: Rect = [(1, 4001); 4];
    rule.accepts_vol(&full_space)
}

impl Rule {
    /// Parse a string discripton of the puzzle
    fn from_puzzle_str(input: &'static str) -> Rc<Rule> {
        let workflow_regex = Regex::new(r"(\w+)\{(.+\,\w+)\}").unwrap();
        let raw_rules: HashMap<Workflow, &'static str> =
            parse_lines(workflow_regex, input).collect();
        let mut rules: HashMap<Workflow, Rc<Rule>> =
            [("A", Rc::new(Rule::Accept)), ("R", Rc::new(Rule::Reject))].into();
        Rule::construct_rules("in", &raw_rules, &mut rules)
    }

    /// Helper function for `from_puzzle_str` to recurvesly construct a tree of `Rc<Rule>`
    fn construct_rules(
        workflow: Workflow,
        raw_rules: &HashMap<Workflow, &'static str>,
        rc_rules: &mut HashMap<Workflow, Rc<Rule>>,
    ) -> Rc<Rule> {
        let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
        match rc_rules.get(workflow) {
            Some(rule) => Rc::clone(rule),
            None => {
                let mut rules: Vec<&str> = raw_rules.get(workflow).unwrap().split(",").collect();
                let rule = &Rule::construct_rules(rules.pop().unwrap(), raw_rules, rc_rules);
                let mut rule = Rc::clone(rule);
                while let Some(rule_str) = rules.pop() {
                    let (axis, order, split, next_workflow) = parse_line(&rule_regex, rule_str);
                    let axis = match axis {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!("Invalid axis: {}", axis),
                    };
                    let (reverse_children, split) = match order {
                        '<' => (false, split),
                        '>' => (true, split + 1),
                        _ => panic!("Invalid order: {}", order),
                    };
                    let child_rule = Rule::construct_rules(next_workflow, raw_rules, rc_rules);
                    rule = Rc::new(Rule::Split {
                        split_axis: axis,
                        split_pos: split,
                        children: if reverse_children {
                            [rule, child_rule]
                        } else {
                            [child_rule, rule]
                        },
                    });
                }
                rc_rules.insert(workflow, Rc::clone(&rule));
                rule
            }
        }
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
                let part_pos = part.0[*split_axis];
                if part_pos < *split_pos {
                    children[0].accepts_part(part)
                } else {
                    children[1].accepts_part(part)
                }
            }
        }
    }

    /// Returns the volume of the subspace of the bounds that this rule accepts
    fn accepts_vol(&self, rect: &Rect) -> Dim {
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
                rect_left[*split_axis].1 = *split_pos;
                rect_right[*split_axis].0 = *split_pos;

                let vol_left = children[0].accepts_vol(&rect_left);
                let vol_right = children[1].accepts_vol(&rect_right);
                vol_left + vol_right
            }
        }
    }
}

/// Converts a list of parts in to a Vec<Part>
fn parts_from_str(input: &'static str) -> Vec<Part> {
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    parse_lines(part_regex, input)
        .map(|(x_pos, m_pos, a_pos, s_pos)| Part([x_pos, m_pos, a_pos, s_pos]))
        .collect()
}
