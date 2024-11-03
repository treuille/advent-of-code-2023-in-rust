#![allow(dead_code)]

use advent_of_code_2023_in_rust::parse_regex::parse_line;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

/// The four characters that can be in a part
const VAL_CHARS: [char; 4] = ['a', 's', 'm', 'x'];

type WorkflowName = &'static str;

#[derive(Debug, Copy, Clone)]
enum NextWorkflow {
    Accept,
    Reject,
    Workflow(WorkflowName),
}

impl NextWorkflow {
    fn from_str(s: &'static str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(s),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    instructions: Vec<(char, Ordering, usize, NextWorkflow)>,
    fallback: NextWorkflow,
}

impl Workflow {
    fn run(&self, part: &Part) -> NextWorkflow {
        for (val_char, test, workflow_val, next_workflow) in &self.instructions {
            let part_val = part.get(val_char).unwrap();
            if part_val.cmp(workflow_val) == *test {
                return *next_workflow;
            }
        }
        self.fallback
    }
}

type Part = HashMap<char, usize>;

/// A PartSet is is a set of non-overlapping PartRanges. It accepts the union of
/// those ranges.
#[derive(Debug, Clone)]
struct PartSet(Vec<PartRange>);

impl PartSet {
    /// Constructor
    fn new(part_ranges: Vec<PartRange>) -> Self {
        let part_set = Self(part_ranges);
        assert!(part_set.non_overlapping(), "Range overlap: {:?}", part_set);
        assert!(part_set.all_nonempty(), "Empty ranges: {:?}", part_set);
        part_set
    }

    /// The PartSet that accepts all values
    fn all() -> Self {
        Self::new(vec![PartRange::all()])
    }

    /// The PartSet that accepts no values
    fn none() -> Self {
        Self::new(vec![])
    }

    /// Union this PartSet with another PartSet
    fn union(&self, other: &Self) -> Self {
        let PartSet(self_ranges) = self;
        let PartSet(other_ranges) = other;
        let mut self_ranges = self_ranges.clone();
        let mut other_ranges = other_ranges.clone();
        let mut union_ranges: Vec<PartRange> = Vec::new();
        //let mut other_ranges = self_ranges.clone();
        while let Some(self_range) = self_ranges.pop() {
            for other_range in other_ranges {
                let (intersection, self_remainder, other_remainder) =
                    self_range.intersect_part_range(&other_range);
                if !intersection.empty() {
                    union_ranges.push(intersection);
                }
                if !self_remainder.empty() {
                    self_ranges.push(self_remainder);
                }
                if !other_remainder.empty() {
                    other_ranges.push(other_remainder);
                }
            }
        }
        //for self_range in self_ranges {
        //    for other_range in other_ranges {
        //        let (intersection, self_remainder, other_remainder) =
        //            self_range.intersect_part_range(other_range);
        //        todo!()
        //    }
        //}
        todo!()
    }

    /// Intersect this PartSet with a instruction
    /// Returns (intersection, remainder)
    /// where intersection is the subset of the PartSet that accepts the instruction
    /// and remainder is the subset of the PartSet that does not accept the instruction
    fn intersect_instruction(
        &self,
        val_char: char,
        test: Ordering,
        val_num: usize,
    ) -> (Self, Self) {
        let PartSet(part_ranges) = self;
        let mut intersection_ranges: Vec<PartRange> = Vec::new();
        let mut remainder_ranges: Vec<PartRange> = Vec::new();
        for part_range in part_ranges {
            let (intersection, remainder) =
                part_range.intersect_instruction(val_char, test, val_num);
            if !intersection.empty() {
                intersection_ranges.push(intersection);
            }
            if !remainder.empty() {
                remainder_ranges.push(remainder);
            }
        }
        (Self::new(intersection_ranges), Self::new(remainder_ranges))
    }

    /// Finds the intersection of this PartSet with a workflow
    fn intersect_workflow(
        &self,
        workflow_name: WorkflowName,
        workflows: &HashMap<WorkflowName, Workflow>,
    ) -> PartSet {
        let workflow = workflows.get(workflow_name).unwrap();
        let mut part_set: PartSet = self.clone();
        let mut workflow_intersection = PartSet::all();
        for &(val_char, test, val_num, next_workflow) in &workflow.instructions {
            if part_set.empty() {
                break;
            }
            let (intersection, remainder) = part_set.intersect_instruction(val_char, test, val_num);
            if !intersection.empty() {
                workflow_intersection = workflow_intersection.union(&match next_workflow {
                    NextWorkflow::Accept => intersection,
                    NextWorkflow::Reject => PartSet::none(),
                    NextWorkflow::Workflow(next_workflow) => {
                        intersection.intersect_workflow(next_workflow, workflows)
                    }
                });
            }
            part_set = remainder;
        }
        workflow_intersection = workflow_intersection.union(&match workflow.fallback {
            NextWorkflow::Accept => part_set,
            NextWorkflow::Reject => PartSet::none(),
            NextWorkflow::Workflow(next_workflow) => {
                part_set.intersect_workflow(next_workflow, workflows)
            }
        });
        workflow_intersection
    }

    /// Returns true if the PartSet is empty
    fn empty(&self) -> bool {
        assert!(self.non_overlapping(), "Range overlap: {:?}", self);
        assert!(self.all_nonempty(), "Empty ranges: {:?}", self);
        let PartSet(part_ranges) = self;
        part_ranges.is_empty()
    }

    /// Returns true if all PartRanges are non-empty
    fn all_nonempty(&self) -> bool {
        let PartSet(part_ranges) = self;
        part_ranges.iter().all(|part_range| !part_range.empty())
    }

    /// Returns true if all PartRanges are mutaually non-overlapping
    fn non_overlapping(&self) -> bool {
        let PartSet(part_ranges) = self;
        for (i, self_part_range) in part_ranges.iter().enumerate() {
            for other_part_range in part_ranges.iter().skip(i + 1) {
                let (intersection, _self_remainder, _other_remainder) =
                    self_part_range.intersect_part_range(other_part_range);
                if !intersection.empty() {
                    return false;
                }
            }
        }
        true
    }
}

/// A PartRange is a range of values
#[derive(Debug, Clone)]
struct PartRange(HashMap<char, (usize, usize)>);

impl PartRange {
    /// The PartRange that accepts all values
    fn all() -> Self {
        Self(
            VAL_CHARS
                .iter()
                .map(|&val_char| (val_char, (1, 4001)))
                .collect(),
        )
    }

    /// The PartRange that accepts no values
    fn none() -> Self {
        Self(
            VAL_CHARS
                .iter()
                .map(|&val_char| (val_char, (0, 0)))
                .collect(),
        )
    }

    /// Intersects this range with an instruction.
    /// Returns the tuple (intersection, remainder) where
    fn intersect_instruction(
        &self,
        val_char: char,
        test: Ordering,
        val_num: usize,
    ) -> (Self, Self) {
        let PartRange(ranges) = self;
        let (min, max) = ranges.get(&val_char).unwrap();

        // Calculate the the intersection
        let (intersection_min, intersection_max) = match test {
            Ordering::Less => (*min, val_num),
            Ordering::Greater => (val_num, *max),
            _ => unreachable!("Invalid test: {:?}", test),
        };
        let mut intersection_ranges = ranges.clone();
        intersection_ranges.insert(val_char, (intersection_min, intersection_max));

        // Now calculate the remainder
        let (remainder_min, remainder_max) = match test {
            Ordering::Less => (val_num, *max),
            Ordering::Greater => (*min, val_num),
            _ => unreachable!("Invalid test: {:?}", test),
        };
        let mut remainder_ranges = ranges.clone();
        remainder_ranges.insert(val_char, (remainder_min, remainder_max));

        (Self(intersection_ranges), Self(remainder_ranges))
    }

    /// Intersects this PartRange with another.
    /// Returns the tuple (intersection, self_remainder, other_remainder) where
    ///
    /// 1. `intersection` is the subset of both self and other.
    /// 2. `self_remainder` is a subset of `self`.
    /// 3. `other_remainder` is a subset of `other`.
    /// 4. All three PartRanges are non-overlapping.
    /// 5. The union of the PartRanges equals the union of `self` and `other`.
    fn intersect_part_range(&self, other: &Self) -> (Self, PartSet, PartSet) {
        let PartRange(self_ranges) = self;
        let PartRange(other_ranges) = other;
        let intersection_ranges: HashMap<char, (usize, usize)> = HashMap::new();
        let self_remainder_ranges: HashMap<char, Vec<(usize, usize)>> = VAL_CHARS
            .iter()
            .map(|&val_car| (char, Vec::new()))
            .collect();
        let other_remainder_ranges: HashMap<char, Vec<(usize, usize)>> = VAL_CHARS
            .iter()
            .map(|&val_car| (char, Vec::new()))
            .collect();
        for val_char in VAL_CHARS.iter() {
            let &(self_min, self_max) = self_ranges.get(val_char).unwrap();
            let &(other_min, other_max) = other_ranges.get(val_char).unwrap();
            let mut indices = vec![self_min, self_max, other_min, other_max];
            indices.sort();
            let subset = |min_1, max_1, min_2, max_2| min_1 >= min_2 && max_1 <= max_2;
            for (&min, &max) in indices.iter().tuple_windows() {
                let self_subset = subset(self_min, self_max, other_min, other_max);
                let other_subset = subset(self_min, self_max, other_min, other_max);
                match (self_subset, other_subset) {
                    (true, true) => {
                        assert!(intersection_ranges.get(val_char).is_none());
                        intersection_ranges.insert(*val_char, (min, max));
                    }
                    (true, false) => self_remainder_ranges[val_char].push((min, max)),
                    (false, true) => other_remainder_ranges[val_char].push((min, max)),
                    (false, false) => unreachable!(),
                }
            }
        }

        //    let (intersection_min, intersection_max) = (
        //        std::cmp::max(*self_min, *other_min),
        //        std::cmp::min(*self_max, *other_max),
        //    );
        //    let (self_remainder_min, self_remainder_max) = (
        //        std::cmp::min(*self_min, *other_min),
        //        std::cmp::max(*self_min, *other_min),
        //    );
        //    let (other_remainder_min, other_remainder_max) = (
        //        std::cmp::min(*self_max, *other_max),
        //        std::cmp::max(*self_max, *other_max),
        //    );
        //    intersection_ranges.insert(*val_char, (intersection_min, intersection_max));
        //    self_remainder_ranges.insert(*val_char, (self_remainder_min, self_remainder_max));
        //    other_remainder_ranges.insert(*val_char, (other_remainder_min, other_remainder_max));
        //}
        //VAL_CHARS.iter().map(|&val_char| todo!()).collect();
        //let intersection = Self(intersection_ranges);
        //let self_remainder = Self(self_remainder_ranges);
        //let other_remainder = Self(other_remainder_ranges);

        assert!(intersection.subset_eq(self)); // Condition 1
        assert!(intersection.subset_eq(other)); // Condition 1
        assert!(self_remainder.subset_eq(self)); // Condition 2
        assert!(other_remainder.subset_eq(other)); // Condition 3
        assert!(!intersection.overlaps(&self_remainder)); // Condition 4
        assert!(!self_remainder.overlaps(&other_remainder)); // Condition 4
        assert!(!other_remainder.overlaps(&intersection)); // Condition 4

        (intersection, self_remainder, other_remainder)
    }

    fn empty(&self) -> bool {
        let PartRange(ranges) = self;
        ranges.values().any(|(min, max)| min >= max)
    }

    /// Returns true if this PartRange overlaps with another
    fn overlaps(&self, other: &Self) -> bool {
        let PartRange(self_ranges) = self;
        let PartRange(other_ranges) = other;
        for val_char in VAL_CHARS.iter() {
            let (self_min, self_max) = self_ranges.get(val_char).unwrap();
            let (other_min, other_max) = other_ranges.get(val_char).unwrap();
            if !(self_min >= other_max || other_min >= self_max) {
                return true;
            }
        }
        false
    }

    /// Returns true if this PartRange is a subset of, or equal to, another
    fn subset_eq(&self, other: &Self) -> bool {
        let PartRange(self_ranges) = self;
        let PartRange(other_ranges) = other;
        for val_char in VAL_CHARS.iter() {
            let (self_min, self_max) = self_ranges.get(val_char).unwrap();
            let (other_min, other_max) = other_ranges.get(val_char).unwrap();
            let self_empty = self_min >= self_max;
            let other_empty = other_min >= other_max;
            match (self_empty, other_empty) {
                (true, _) => continue,
                (false, true) => return false,
                (false, false) if (self_min < other_min || self_max > other_max) => return false,
                _ => continue,
            }
        }
        true
    }
}

fn main() {
    // Parse the input, counting the numbeof matches per card
    //let input = include_str!("");
    let input = include_str!("../../puzzle_inputs/day_19_test.txt");
    let (workflows, parts) = parse_input(input);

    // Run the simulation for part A
    println!("Part A: {}", solve_part_a(&workflows, &parts));

    // Run the simulation for part B
    println!("Part B: {}", solve_part_b(&workflows, &parts));
}

fn solve_part_a(workflows: &HashMap<WorkflowName, Workflow>, parts: &Vec<Part>) -> usize {
    let mut answer: usize = 0;
    for part in parts {
        let mut workflow = NextWorkflow::Workflow("in");
        while let NextWorkflow::Workflow(workflow_name) = workflow {
            workflow = workflows.get(workflow_name).unwrap().run(part);
        }
        match workflow {
            NextWorkflow::Accept => answer += part.values().sum::<usize>(),
            NextWorkflow::Reject => continue,
            _ => unreachable!("Invalid workflow: {:?}", workflow),
        }
    }
    answer
}

fn solve_part_b(workflows: &HashMap<WorkflowName, Workflow>) -> usize {
    let accepted_ranges = PartSet::all().intersect_workflow("in", workflows);
    println!("accepted_ranges: {:?}", accepted_ranges);

    todo!()
}

fn parse_input(input: &'static str) -> (HashMap<WorkflowName, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    // Parse the workflows
    let instruction_regex = Regex::new(r"(a|s|m|x)([<>])(\d+)\:(\w+)").unwrap();
    let workflows: HashMap<WorkflowName, Workflow> = workflows
        .lines()
        .map(|workflow| {
            //println!("workflow: {:?}", workflow);
            let (name, instructions) = workflow.split_once("{").unwrap();
            let instructions = &instructions[..instructions.len() - 1];
            let mut instructions: Vec<&str> = instructions.split(",").collect();
            let fallback = NextWorkflow::from_str(instructions.pop().unwrap());
            let instructions = instructions
                .into_iter()
                .map(|instruction| {
                    let (val_char, test, val_num, next_workflow): (char, char, usize, &str) =
                        parse_line(&instruction_regex, instruction);
                    let test = match test {
                        '<' => Ordering::Less,
                        '>' => Ordering::Greater,
                        _ => panic!("Invalid test: {:?}", test),
                    };
                    let next_workflow = NextWorkflow::from_str(next_workflow);
                    (val_char, test, val_num, next_workflow)
                })
                .collect();
            (
                name,
                Workflow {
                    instructions,
                    fallback,
                },
            )
        })
        .collect();

    // Parse the parts
    let parts: Vec<Part> = parts
        .lines()
        .map(|part| {
            part[1..part.len() - 1]
                .split(",")
                .map(|val| {
                    let val_char = val.chars().next().unwrap();
                    let val_num = val[2..].parse::<usize>().unwrap();
                    (val_char, val_num)
                })
                .collect()
        })
        .collect();

    (workflows, parts)
}

struct PartRange2 {
    ranges: HashMap<char, (usize, usize)>,
    children: PartRangeChildren,
}

enum PartRangeChildren {
    All,
    Split {
        split_char: char,
        split_val: usize,
        left: Box<PartRange2>,
        right: Box<PartRange2>,
    },
}

#[allow(unused_variables)]
impl PartRange2 {
    /// Intsersects two part ranges and returns
    /// (intserection, self_remainder, other_remainder)
    /// so that:
    ///
    /// 1. `intersection` is the subset of both self and other.
    /// 2. `self_remainder` is a subset of `self`.
    /// 3. `other_remainder` is a subset of `other`.
    /// 4. All three PartRanges are non-overlapping.
    /// 5. The union of the PartRanges equals the union of `self` and `other`.
    fn intersect(&self, other: &Self) -> (Self, Self, Self) {
        todo!()
    }

    /// Finds the union of these two part ranges
    fn union(&self, other: &Self) -> Self {
        todo!()
    }

    /// Returns true if this PartRange is empty
    fn empty(&self) -> bool {
        todo!()
    }
}

//// Solve 19a
//let sol_19a: usize = 12;
//let correct_sol_19a: usize = 32;
//println!("* 19a *");
//println!("My solution: {sol_19a}");
//println!("Correct solution: {correct_sol_19a}");
//println!("Equal: {}\n", sol_19a == correct_sol_19a);
//
//// Solve 19b
//let sol_19b: usize = 56;
//let correct_sol_19b: usize = 79;
//println!("* 19b *");
//println!("My solution: {sol_19b}");
//println!("Correct solution: {correct_sol_19b}");
//
//println!("Equal: {}\n", sol_19b == correct_sol_19b);
