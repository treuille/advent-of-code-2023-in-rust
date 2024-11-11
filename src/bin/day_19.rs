use advent_of_code_2023_in_rust::parse_regex::{parse_line, parse_lines};
use cached::proc_macro::cached;
use cached::UnboundCache;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

/// The descriptor of a workflow
type Workflow = &'static str;

/// The type of points along each of the axes of the 4D lattice
type Dim = u64;

/// We index the into 4 axes using `usize`
type Axis = usize;

/// A Part is a point in the 4D lattice [1,4001)^4
type Part = [Dim; 4];

/// An axis-aligned rectangle in the 4D lattice, defined by the minimum
/// (inclusive) and maximum (exclusive) positions along each axis.
type Rect = [(Dim, Dim); 4];

/// A rule is part of a decision tree that either accepts or rejects parts
enum Rule {
    Accept,
    Reject,
    Split {
        split_axis: Axis,
        split_pos: Dim,
        children: [Arc<Rule>; 2],
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

fn solve_part_a(rule: &Arc<Rule>, parts: &[Part]) -> u64 {
    parts
        .iter()
        .filter_map(|part| {
            if rule.accepts_part(part) {
                let part_score: u64 = part.iter().sum();
                Some(part_score)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_b(rule: &Arc<Rule>) -> Dim {
    let full_space: Rect = [(1, 4001); 4];
    rule.accepts_vol(&full_space)
}

impl Rule {
    /// Parse a string discripton of the puzzle
    fn from_puzzle_str(input: &'static str) -> Arc<Rule> {
        let workflow_regex = Regex::new(r"(\w+)\{(.+\,\w+)\}").unwrap();
        let raw_rules: HashMap<Workflow, &'static str> =
            parse_lines(workflow_regex, input).collect();
        construct_rules("in", &raw_rules)
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
                let part_pos = part[*split_axis];
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
                rect_left[*split_axis].1 = *split_pos;
                let vol_left = children[0].accepts_vol(&rect_left);

                let mut rect_right = *rect;
                rect_right[*split_axis].0 = *split_pos;
                let vol_right = children[1].accepts_vol(&rect_right);

                vol_left + vol_right
            }
        }
    }
}

/// Helper function for `from_puzzle_str` to recurvesly construct a tree of `Arc<Rule>`
#[cached(
    ty = "UnboundCache<String, Arc<Rule>>", // Specify the type of cache here
    create = "{ UnboundCache::new() }", // Initialize cache as
    convert = r#"{ workflow.to_owned() }"#,
)]
fn construct_rules(workflow: Workflow, raw_rules: &HashMap<Workflow, &'static str>) -> Arc<Rule> {
    if workflow == "A" {
        return Arc::new(Rule::Accept);
    } else if workflow == "R" {
        return Arc::new(Rule::Reject);
    }
    let rule_regex = Regex::new(r"([xmas])([<>])(\d+):(\w+)").unwrap();
    let mut rules: Vec<&str> = raw_rules.get(workflow).unwrap().split(",").collect();
    let fallback_workflow = rules.pop().unwrap();
    let rule = &construct_rules(fallback_workflow, raw_rules);
    let mut rule = Arc::clone(rule);
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
        let child_rule = construct_rules(next_workflow, raw_rules);
        rule = Arc::new(Rule::Split {
            split_axis: axis,
            split_pos: split,
            children: if reverse_children {
                [rule, child_rule]
            } else {
                [child_rule, rule]
            },
        });
    }
    rule
}

/// Converts a list of parts in to a Vec<Part>
fn parts_from_str(input: &'static str) -> Vec<Part> {
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    parse_lines(part_regex, input)
        .map(|(x_pos, m_pos, a_pos, s_pos)| [x_pos, m_pos, a_pos, s_pos])
        .collect()
}
