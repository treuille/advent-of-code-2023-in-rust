#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::doc_lazy_continuation
)]

use advent_of_code_2023_in_rust::parse_regex::parse_line;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// The four characters that can be in a part
const AXES: [char; 4] = ['a', 's', 'm', 'x'];

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
struct Instruction {
    axis: char,
    test: Ordering,
    value: usize,
}

#[derive(Debug)]
struct Workflow {
    instructions: Vec<(Instruction, NextWorkflow)>,
    fallback: NextWorkflow,
}

impl Workflow {
    fn run(&self, part: &Part) -> NextWorkflow {
        for (instruction, next_workflow) in &self.instructions {
            let part_val = part.get(&instruction.axis).unwrap();
            if part_val.cmp(&instruction.value) == instruction.test {
                return *next_workflow;
            }
        }
        self.fallback
    }
}

type Part = HashMap<char, usize>;

/// A singly hyper-rectangular cube in part space
#[derive(Debug, Clone, PartialEq, Eq)]
struct PartCube(HashMap<char, (usize, usize)>);

/// A disjoint union of PartCubes
#[derive(Debug, Clone, PartialEq, Eq)]
struct PartArea(HashSet<PartCube>);

impl PartCube {
    /// Constructor. None if the part cube is empty.
    /// NOTE: Tested with assertions
    fn new(cube: HashMap<char, (usize, usize)>) -> Option<Self> {
        let cube = Self(cube);
        match cube.empty() {
            true => None,
            false => {
                //cube.sample().for_each(|part| {
                //    assert!(cube.contains_part(&part));
                //});
                Some(cube)
            }
        }
    }

    /// The full area of the part space
    /// WARNING: Not tested
    fn all() -> Self {
        Self::new(AXES.iter().map(|&val_char| (val_char, (1, 4001))).collect()).unwrap()
    }

    /// Returns the number of parts in this area.
    /// WARNING: Not tested
    fn n_parts(&self) -> usize {
        let PartCube(cube) = self;
        cube.values().map(|(min, max)| max - min).product()
    }

    /// Sample usage
    /// NOTE: Tested with assertions
    fn sample(&self) -> impl Iterator<Item = Part> + '_ {
        assert!(!self.empty());
        let PartCube(cube) = self;
        AXES.iter()
            .map(|&axis| {
                let &(min, max) = cube.get(&axis).unwrap();
                [min, (min + max) / 2, max - 1]
            })
            .multi_cartesian_product()
            .map(|vals| {
                let part = AXES
                    .iter()
                    .copied()
                    .zip(vals)
                    .collect::<HashMap<char, usize>>();

                //assert!(self.contains_part(&part));

                part
            })
    }

    /// Intersects the PartCue with an Instruction, returning
    /// (intersection, remainder) where
    /// 1. intersection is the subset of self that would be accepted by the instructiono
    /// 2. remainder is the subset of self that would be rejected by the instruction
    /// 3. The union of intersection and remainder equals self
    /// 4. If either value is empty, the corresponding value is None
    /// NOTE: Tested with assertions
    fn intersect_instruction(&self, instruction: &Instruction) -> (Option<Self>, Option<Self>) {
        // test_part: {'s': 537, 'x': 2440, 'a': 2007, 'm': 1}
        // instruction: Instruction { axis: 'x', test: Greater, value: 2440 }

        let PartCube(cube) = self;
        let &(min, max) = cube.get(&instruction.axis).unwrap();
        let (intersection_min, intersection_max, remainder_min, remainder_max) =
            match instruction.test {
                Ordering::Less => (
                    min.min(instruction.value),
                    max.min(instruction.value),
                    min.max(instruction.value),
                    max.max(instruction.value),
                ),
                Ordering::Greater => (
                    min.max(instruction.value + 1),
                    max.max(instruction.value + 1),
                    min.min(instruction.value + 1),
                    max.min(instruction.value + 1),
                ),
                _ => panic!("Invalid test: {:?}", instruction.test),
            };
        let new_cube = |new_min, new_max| {
            let mut new_cube = cube.clone();
            new_cube.insert(instruction.axis, (new_min, new_max));
            Self::new(new_cube)
        };

        let intersection_cube = new_cube(intersection_min, intersection_max);
        let remainder_cube = new_cube(remainder_min, remainder_max);

        //// debug - begin - test the results through sampling
        //if let Some(intersection_cube) = &intersection_cube {
        //    assert!(intersection_cube
        //        .sample()
        //        .all(|part| self.contains_part(&part)
        //            && intersection_cube.contains_part(&part)
        //            && part[&instruction.axis].cmp(&instruction.value) == instruction.test));
        //}
        //if let Some(remainder_cube) = &remainder_cube {
        //    assert!(remainder_cube.sample().all(|part| self.contains_part(&part)
        //        && remainder_cube.contains_part(&part)
        //        && part[&instruction.axis].cmp(&instruction.value) != instruction.test));
        //}
        //// debug - end

        (intersection_cube, remainder_cube)
    }

    /// WARNING: Not tested
    fn contains_part(&self, part: &Part) -> bool {
        let PartCube(cube) = self;
        cube.iter().all(|(axis, (min, max))| {
            let part_val = part.get(axis).unwrap();
            min <= part_val && max > part_val
        })
    }

    /// NOTE: Tested with assertions
    fn contains_cube(&self, other: &PartCube) -> bool {
        let PartCube(self_cube) = self;
        let PartCube(other_cube) = other;
        let contains_cube = AXES.iter().all(|axis| {
            let (self_min, self_max) = self_cube.get(axis).unwrap();
            let (other_min, other_max) = other_cube.get(axis).unwrap();
            self_min <= other_min && self_max >= other_max
        });
        //if contains_cube {
        //    assert!(other.sample().all(|part| self.contains_part(&part)));
        //}
        contains_cube
    }

    /// NOTE: Tested with assertions
    fn intersects(&self, other: &Self) -> bool {
        let PartCube(self_cube) = self;
        let PartCube(other_cube) = other;
        let intersects = AXES.iter().all(|axis| {
            let (self_min, self_max) = self_cube.get(axis).unwrap();
            let (other_min, other_max) = other_cube.get(axis).unwrap();
            self_min < other_max && self_max > other_min
        });
        //if intersects {
        //    assert!(self
        //        .sample()
        //        .chain(other.sample())
        //        .all(|part| { self.contains_part(&part) && other.contains_part(&part) }));
        //}
        intersects
    }

    /// WARNING: Not tested
    fn empty(&self) -> bool {
        let PartCube(cube) = self;
        cube.values().any(|(min, max)| min >= max)
    }
}

impl Hash for PartCube {
    /// WARNING: Not tested
    fn hash<H: Hasher>(&self, state: &mut H) {
        let PartCube(cube) = self;
        for axis in AXES.iter() {
            cube[axis].hash(state);
        }
    }
}

#[allow(unused_variables)]
impl PartArea {
    /// Construct a PartArea from a list of PartCubes.
    /// Also runs a bunch of validation checks.
    /// NOTE: Tested with assertions
    fn new(cubes: HashSet<PartCube>) -> Self {
        //// Make sure that the all the cubes are non-empty
        //assert!(cubes.iter().all(|cube| !cube.empty()));
        //
        //// Make sure that none of the cubes overlap
        //for (i, cube) in cubes.iter().enumerate() {
        //    for other_cube in cubes.iter().skip(i + 1) {
        //        assert!(!cube.intersects(other_cube));
        //    }
        //}

        Self(cubes)
    }

    /// The full area of the part space
    /// WARNING: Not tested
    fn all() -> Self {
        Self::new(HashSet::from([PartCube::all()]))
    }

    /// An empty area of the part space
    /// WARNING: Not tested
    fn none() -> Self {
        Self::new(HashSet::new())
    }

    /// Returns the number of parts in this area.
    /// WARNING: Not tested
    fn n_parts(&self) -> usize {
        let PartArea(cubes) = self;
        cubes.iter().map(|part_cube| part_cube.n_parts()).sum()
    }

    /// Intersects two parts and return the triple:
    /// `(intersection, self_remainder, other_remainder)`
    /// with the following properties:
    /// 1. `intersection` is the subset of both `self` and `other`.
    /// 2. `self_remainder` is a subset of `self`.
    /// 3. `other_remainder` is a subset of `other`.
    /// 4. The three piecesw are non-overlapping
    /// 5. The union of the three pieces equals the union of `self` and `other`.
    fn intersect(&self, other: &Self) -> (Self, Self, Self) {
        todo!("implement PartArea::intersect(..)")
    }

    // NOTE: This is the old implementation, which works but is slow
    /// Returns the union of two PartAreas.
    /// NOTE: Tested with assertions
    fn old_union(&self, other: &Self) -> Self {
        Self::new(
            self.outer(other)
                .filter(|part_cube| self.contains_cube(part_cube) || other.contains_cube(part_cube))
                .collect(),
        )
    }

    /// Returns the union of two PartAreas.
    /// NOTE: Tested with assertions
    fn union(&self, other: &Self) -> Self {
        //self.old_union(other)
        self.recursive_union(other, 0)
    }

    fn recursive_union(&self, other: &Self, depth: usize) -> Self {
        if self.empty() {
            return other.clone();
        } else if other.empty() {
            return self.clone();
        }
        let PartArea(self_cubes) = self;
        let PartArea(other_cubes) = other;
        let n_self_cubes = self_cubes.len();
        let n_other_cubes = other_cubes.len();
        let max_depth = 10;
        let max_cells = 20;

        if depth >= max_depth || n_self_cubes * n_other_cubes < max_cells {
            return self.old_union(other);
        }

        // Find the split with the best balance
        let (self_left, self_right, other_left, other_right) = AXES
            .iter()
            .map(|&axis| {
                // Get the min and max along this axis
                let (min, max) = self_cubes
                    .iter()
                    .flat_map(|PartCube(cube)| {
                        let &(min, max) = cube.get(&axis).unwrap();
                        [min, max]
                    })
                    .chain(other_cubes.iter().flat_map(|PartCube(cube)| {
                        let &(min, max) = cube.get(&axis).unwrap();
                        [min, max]
                    }))
                    .minmax()
                    .into_option()
                    .unwrap();

                // Split the cubes along this axis
                let split_at = (min + max) / 2;
                let instruction = Instruction {
                    axis,
                    test: Ordering::Less,
                    value: split_at,
                };
                let (self_left, self_right) = self.intersect_instruction(&instruction);
                let (other_left, other_right) = other.intersect_instruction(&instruction);
                (self_left, self_right, other_left, other_right)
            })
            .min_by_key(|(self_left, self_right, other_left, other_right)| {
                let left_balance = usize::abs_diff(self_left.0.len(), self_right.0.len());
                let right_balance = usize::abs_diff(other_left.0.len(), other_right.0.len());

                //// debug - begin - show balance stats
                //println!("\ndepth: {}", depth);
                //println!(
                //    "self_left.len(): {} vs self_right.len(): {}",
                //    self_left.0.len(),
                //    self_right.0.len()
                //);
                //println!(
                //    "other_left.len(): {} vs other_right.len(): {}",
                //    other_left.0.len(),
                //    other_right.0.len()
                //);
                //println!(
                //    "left_balance: {} right_balance: {}",
                //    left_balance, right_balance
                //);
                //// debug - end

                left_balance + right_balance
            })
            .unwrap();

        let PartArea(union_left) = self_left.recursive_union(&other_left, depth + 1);

        //// debug - begin - test the results through sampling
        //let union_left_clone = PartArea::new(union_left.clone());
        //assert!(union_left_clone
        //    .sample()
        //    .all(|part| self_left.contains_part(&part) || other_left.contains_part(&part)));
        //println!(
        //    "Sucessfully tested {} samples in union(..)",
        //    union_left_clone.sample().count()
        //);
        //// debug - end

        let PartArea(union_right) = self_right.recursive_union(&other_right, depth + 1);

        //// debug - begin - test the results through sampling let union_left_clone = PartArea::new(union_left.clone());
        //let union_right_clone = PartArea::new(union_right.clone());
        //assert!(union_right_clone
        //    .sample()
        //    .all(|part| self_right.contains_part(&part) || other_right.contains_part(&part)));
        //println!(
        //    "Sucessfully tested {} samples in union(..)",
        //    union_right_clone.sample().count()
        //);
        //// debug - end

        let result = Self::new(union_left.into_iter().chain(union_right).collect());

        //// debug - begin - test the results through sampling
        //assert!(result
        //    .sample()
        //    .all(|part| self.contains_part(&part) || other.contains_part(&part)));
        //println!(
        //    "Sucessfully tested {} samples in union(..)",
        //    result.sample().count()
        //);
        //// debug - end

        #[allow(clippy::let_and_return)]
        result
    }

    /// Subtracts other from self, returning the subset of self that is not in other.
    /// NOTE: Tested with assertions
    fn subtract(&self, other: &Self) -> Self {
        Self::new(
            self.outer(other)
                .filter(|part_cube| {
                    self.contains_cube(part_cube) && !other.contains_cube(part_cube)
                })
                .collect(),
        )

        //// debug - begin - test the results through sampling
        //assert!(difference
        //    .sample()
        //    .all(|part| self.contains_part(&part) && !other.contains_part(&part)));
        //println!(
        //    "Sucessfully tested {} samples in subtract(..)",
        //    difference.sample().count()
        //);
        //// debug - end
    }

    /// Returns a sequence of PartCubes where
    /// 1. Each part cube is either strictly inside or strictly outside of self and other
    /// 2. The union of the PartCubes is a superset of self and other
    fn outer(&self, other: &Self) -> impl Iterator<Item = PartCube> {
        AXES.iter()
            .map(|&axis| {
                let get_indices = |PartArea(cubes): &PartArea| {
                    cubes
                        .iter()
                        .flat_map(|PartCube(cube): &PartCube| {
                            let &(min, max) = cube.get(&axis).unwrap();
                            [min, max]
                        })
                        .collect::<Vec<usize>>()
                };
                get_indices(self)
                    .into_iter()
                    .chain(get_indices(other))
                    .sorted()
                    .dedup()
                    .tuple_windows()
            })
            .multi_cartesian_product()
            .map(|ranges| {
                PartCube::new(HashMap::from([
                    ('a', ranges[0]),
                    ('s', ranges[1]),
                    ('m', ranges[2]),
                    ('x', ranges[3]),
                ]))
                .unwrap()
            })
    }

    fn contains_part(&self, part: &Part) -> bool {
        let PartArea(cubes) = self;
        cubes.iter().any(|cube| cube.contains_part(part))
    }

    fn contains_cube(&self, part_cube: &PartCube) -> bool {
        let PartArea(cubes) = self;
        cubes.iter().any(|cube| cube.contains_cube(part_cube))
    }

    fn empty(&self) -> bool {
        let PartArea(cubes) = self;
        cubes.is_empty()
    }

    /// Returns the subset of &self that would be accepted by the workflow.
    fn intersect_workflow(
        &self,
        next_workflow: NextWorkflow,
        workflows: &HashMap<WorkflowName, Workflow>,
    ) -> Self {
        // debug - begin - test guard
        let test_part: Part = HashMap::from([('a', 2007), ('s', 537), ('m', 1), ('x', 2440)]);
        let test_workflow_names = ["px", "rfg"];
        // debug - end - test guard

        match next_workflow {
            NextWorkflow::Accept => self.clone(),
            NextWorkflow::Reject => PartArea::none(),
            NextWorkflow::Workflow(workflow_name) => {
                let workflow = workflows.get(workflow_name).unwrap();

                // debug - begin - test test_part for the "in" workflow
                if test_workflow_names.iter().contains(&workflow_name) {
                    println!("\nworkflow_name: {}", workflow_name);
                    println!("test_part: {:?}", test_part);
                    println!("in self: {}", self.contains_part(&test_part));
                    println!("workflow: {:?}", workflow);
                }
                // debug - end

                let mut result = PartArea::none();
                let mut remaining_area = self.clone();

                println!(
                    "In workflow: {:?} result.size: {} remaining_area.size: {}",
                    workflow_name,
                    result.n_parts(),
                    remaining_area.n_parts()
                );

                for (instruction, next_workflow) in &workflow.instructions {
                    let (intersection, remainder) =
                        remaining_area.intersect_instruction(instruction);

                    if !intersection.empty() {
                        // TODO: Probably can get rid of this if statement above.
                        result = result
                            .union(&intersection.intersect_workflow(*next_workflow, workflows))
                    }

                    // debug - begin - test test_part for the "in" workflow
                    if test_workflow_names.iter().contains(&workflow_name) {
                        println!("\nworkflow_name: {}", workflow_name);
                        println!("instruction: {:?}", instruction);
                        println!(
                            "in intersection: {}",
                            intersection.contains_part(&test_part)
                        );
                        println!("in remainder: {}", remainder.contains_part(&test_part));
                        println!("in result: {}", result.contains_part(&test_part));
                        println!(
                            "in reamining_area: {}",
                            remaining_area.contains_part(&test_part)
                        );
                    }
                    // debug - end

                    if remainder.empty() {
                        return result;
                    }
                    remaining_area = remainder;
                    println!(
                        "In workflow: {:?} result.size: {} remaining_area.size: {}",
                        workflow_name,
                        result.n_parts(),
                        remaining_area.n_parts()
                    );
                }

                // debug - begin - test test_part for the "in" workflow
                if test_workflow_names.iter().contains(&workflow_name) {
                    println!("\nworkflow_name: {}", workflow_name);
                    println!("about to compute fallback: {:?}", workflow.fallback);
                    println!("in result: {}", result.contains_part(&test_part));
                    println!(
                        "in reamining_area: {}",
                        remaining_area.contains_part(&test_part)
                    );
                }
                // debug - end

                if !remaining_area.empty() {
                    let accepted_by_fallback =
                        remaining_area.intersect_workflow(workflow.fallback, workflows);
                    if test_workflow_names.iter().contains(&workflow_name) {
                        println!("\nworkflow_name: {}", workflow_name);
                        println!("just checked accepted by fallback: {:?}", workflow.fallback);
                        println!(
                            "in accepted_by_fallback: {}",
                            accepted_by_fallback.contains_part(&test_part)
                        );
                        println!("in result: {}", result.contains_part(&test_part));
                    }
                    result = result.union(&accepted_by_fallback);
                    if test_workflow_names.iter().contains(&workflow_name) {
                        println!("in result (after): {}", result.contains_part(&test_part));
                    }
                }

                // debug - begin - test test_part for the "in" workflow
                if test_workflow_names.iter().contains(&workflow_name) {
                    println!("\nworkflow_name: {}", workflow_name);
                    println!("just computed fallback: {:?}", workflow.fallback);
                    println!("in result: {}", result.contains_part(&test_part));
                    println!(
                        "in reamining_area: {}",
                        remaining_area.contains_part(&test_part)
                    );
                }
                // debug - end

                //// debug - begin - test the results through sampling
                //assert!(result.sample().all(|part| self.contains_part(&part)
                //    && part_accepted_by_workflow(&part, next_workflow, workflows)));
                //let remaining_area = remaining_area.subtract(&result);
                //assert!(remaining_area.sample().all(|part| {
                //    if !self.contains_part(&part) {
                //        println!("part: {:?} not in self", part);
                //        return false;
                //    } else if part_accepted_by_workflow(&part, next_workflow, workflows) {
                //        println!("part: {:?} accepted by workflow", part);
                //        println!("workflow_name: {:?}", workflow_name);
                //        println!("workflow: {:?}", workflow);
                //        return false;
                //    }
                //    true
                //}));
                //println!(
                //    "Sucessfully tested {} samples in intersect_workflow(..)",
                //    result.sample().count() + remaining_area.sample().count()
                //);
                //// debug - end

                result
            }
        }
    }

    /// Intersects the PartArea with an Instruction, returning
    /// (intersection, remainder) where:
    /// 1. `intersection` is the subset of `self` that would be accepted by the instruction
    /// 2. `remainder` is the subset of `self` that would be rejected by the instruction
    /// 3. The union of `intersection` and `remainder` equals `self`
    /// NOTE: Tested with assertions
    fn intersect_instruction(&self, instruction: &Instruction) -> (Self, Self) {
        let PartArea(cubes) = self;
        let (intersection, remainder): (Vec<Option<PartCube>>, Vec<Option<PartCube>>) = cubes
            .iter()
            .map(|cube| cube.intersect_instruction(instruction))
            .unzip();

        let intersection_area = Self::new(intersection.into_iter().flatten().collect());
        let remainder_area = Self::new(remainder.into_iter().flatten().collect());

        //// debug - begin - assert that the split happened correctly
        //let Instruction { axis, test, value } = instruction;
        //assert!(intersection_area
        //    .sample()
        //    .all(|part| self.contains_part(&part)
        //        && intersection_area.contains_part(&part)
        //        && !remainder_area.contains_part(&part)
        //        && part[axis].cmp(value) == *test));
        //assert!(remainder_area.sample().all(|part| self.contains_part(&part)
        //    && !intersection_area.contains_part(&part)
        //    && remainder_area.contains_part(&part)
        //    && part[axis].cmp(value) != *test));
        //println!(
        //    "Sucessfully tested {} kamples in intersect_instruction(..)",
        //    intersection_area.sample().count() + remainder_area.sample().count()
        //);
        //// debug - end

        (intersection_area, remainder_area)
    }

    /// NOTE: Tested with assertions
    fn sample(&self) -> impl Iterator<Item = Part> + '_ {
        let PartArea(cubes) = self;
        //assert!(cubes
        //    .iter()
        //    .flat_map(|cube| cube.sample())
        //    .all(|part| self.contains_part(&part)));
        cubes.iter().flat_map(|cube| cube.sample())
    }
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    //let input = include_str!("../../puzzle_inputs/day_19_test.txt");

    // Parse the input
    let (workflows, parts) = parse_input(input);

    // Solve 19a
    let sol_19a: usize = solve_part_a(&workflows, &parts);
    let correct_sol_19a: usize = 19114;
    println!("* 19a *");
    println!("My solution: {sol_19a}");
    println!("Correct solution: {correct_sol_19a}");
    println!("Equal: {}\n", sol_19a == correct_sol_19a);

    // Solve 19b
    let sol_19b: usize = solve_part_b(&workflows);
    let correct_sol_19b: usize = 167409079868000;
    println!("* 19b *");
    println!("My solution: {sol_19b}");
    println!("Correct solution: {correct_sol_19b}");
    println!("Equal: {:?}\n", sol_19b.cmp(&correct_sol_19b));
}

fn part_accepted_by_workflow(
    part: &Part,
    mut workflow: NextWorkflow,
    workflows: &HashMap<WorkflowName, Workflow>,
) -> bool {
    while let NextWorkflow::Workflow(workflow_name) = workflow {
        workflow = workflows.get(workflow_name).unwrap().run(part);
    }
    match workflow {
        NextWorkflow::Accept => true,
        NextWorkflow::Reject => false,
        _ => unreachable!("Invalid workflow: {:?}", workflow),
    }
}

fn solve_part_a(workflows: &HashMap<WorkflowName, Workflow>, parts: &Vec<Part>) -> usize {
    let mut answer: usize = 0;
    let workflow = NextWorkflow::Workflow("in");
    for part in parts {
        if part_accepted_by_workflow(part, workflow, workflows) {
            answer += part.values().sum::<usize>();
        }
    }
    answer
}

#[allow(unused_variables)]
fn solve_part_b(workflows: &HashMap<WorkflowName, Workflow>) -> usize {
    let parts = PartArea::all();
    let start_workflow = NextWorkflow::Workflow("in");
    let parts: PartArea = parts.intersect_workflow(start_workflow, workflows);
    //println!("parts: {:?}", parts);
    let answer: usize = parts.n_parts();
    //println!("answer: {}", answer);
    answer
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
                    let (axis, test, value, next_workflow): (char, char, usize, &str) =
                        parse_line(&instruction_regex, instruction);
                    let test = match test {
                        '<' => Ordering::Less,
                        '>' => Ordering::Greater,
                        _ => panic!("Invalid test: {:?}", test),
                    };
                    let next_workflow = NextWorkflow::from_str(next_workflow);
                    (Instruction { axis, test, value }, next_workflow)
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
