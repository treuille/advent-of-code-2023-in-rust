#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::doc_lazy_continuation
)]

use advent_of_code_2023_in_rust::parse_regex::parse_line;
use cached::proc_macro::cached;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{write, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter;
use std::sync::Arc;

/// The four characters that can be in a part
//const AXES: [char; 4] = ['a', 's', 'm', 'x'];

type WorkflowName = &'static str;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum NextWorkflow {
    Accept,
    Reject,
    Workflow(WorkflowName),
}

//type CubeSet = HashSet<PartCube>;
type CubeSet = Vec<PartCube>;

impl NextWorkflow {
    fn from_str(s: &'static str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(s),
        }
    }

    /// Returns the full set of all possible parts accepted by this next Workflow.
    fn accepts(&self, all_workflows: &AllWorkflows) -> Arc<PartArea> {
        //// debug - begin - test guard
        //let test_part: Part = HashMap::from([('a', 2007), ('s', 537), ('m', 1), ('x', 2440)]);
        //let test_workflow_names = ["px", "rfg"];
        //// debug - end - test guard

        let workflows = &all_workflows.workflows;
        {
            let mut cache = all_workflows.cache.borrow_mut();
            let cached_value = cache.get(self).unwrap();
            println!("next_workflow: {:?} cache_value: {:?}", self, cached_value);
            match cached_value {
                AllWorkflowCacheValue::Solved(area) => return area.clone(),
                AllWorkflowCacheValue::Solving => {
                    panic!("Cycle detected in workflow: {:?}", self);
                }
                AllWorkflowCacheValue::Unsolved => {
                    cache.insert(*self, AllWorkflowCacheValue::Solving);
                }
            }
        }

        let part_area: Arc<PartArea> = match self {
            NextWorkflow::Accept => part_area_all(),
            NextWorkflow::Reject => part_area_none(),
            NextWorkflow::Workflow(workflow_name) => {
                let workflow = workflows.get(workflow_name).unwrap();

                let mut result = part_area_none();
                let mut remaining_area = part_area_all();

                for (instruction, next_workflow) in &workflow.instructions {
                    let (intersection, remainder) =
                        remaining_area.intersect_instruction(instruction);

                    if !intersection.empty() {
                        // TODO: Probably can get rid of this if statement above.
                        result = PartArea::union(
                            result,
                            PartArea::intersect_workflow(
                                intersection.into(),
                                *next_workflow,
                                all_workflows,
                            ),
                        );
                    }

                    if remainder.empty() {
                        break;
                    }
                    remaining_area = remainder.into();
                }

                if !remaining_area.empty() {
                    let accepted_by_fallback =
                        remaining_area.intersect_workflow(workflow.fallback, all_workflows);
                    result = PartArea::union(result, accepted_by_fallback);
                }
                result
            }
        };

        let mut cache = all_workflows.cache.borrow_mut();
        assert_eq!(cache.get(self), Some(&AllWorkflowCacheValue::Solving));
        cache.insert(*self, AllWorkflowCacheValue::Solved(part_area.clone()));
        part_area
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Axis {
    A = 0,
    S,
    M,
    X,
}

impl Axis {
    fn iter() -> impl Iterator<Item = Axis> {
        [Axis::A, Axis::S, Axis::M, Axis::X].into_iter()
    }

    fn from_char(c: char) -> Self {
        match c {
            'a' => Axis::A,
            's' => Axis::S,
            'm' => Axis::M,
            'x' => Axis::X,
            _ => panic!("Invalid axis: {}", c),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    axis: Axis,
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
            let part_val = part[instruction.axis as usize];
            if part_val.cmp(&instruction.value) == instruction.test {
                return *next_workflow;
            }
        }
        self.fallback
    }
}

#[derive(PartialEq, Eq)]
enum AllWorkflowCacheValue {
    Unsolved,
    Solving,
    Solved(Arc<PartArea>),
}

impl Debug for AllWorkflowCacheValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AllWorkflowCacheValue::Unsolved => write!(f, "Unsolved"),
            AllWorkflowCacheValue::Solving => write!(f, "Solving"),
            AllWorkflowCacheValue::Solved(part_area) => {
                write!(f, "Solved({} area)", part_area.n_parts())
            }
        }
    }
}

/// The set of all workflows.
struct AllWorkflows {
    workflows: HashMap<WorkflowName, Workflow>,
    cache: RefCell<HashMap<NextWorkflow, AllWorkflowCacheValue>>,
}

impl AllWorkflows {
    fn new(workflows: HashMap<WorkflowName, Workflow>) -> Self {
        let all_possible_workflows = workflows
            .keys()
            .map(|&name| NextWorkflow::Workflow(name))
            .chain([NextWorkflow::Accept, NextWorkflow::Reject]);
        let cache = RefCell::new(
            all_possible_workflows
                .map(|next_workflow| (next_workflow, AllWorkflowCacheValue::Unsolved))
                .collect(),
        );
        Self { workflows, cache }
    }
}

type Part = [usize; 4];

/// A singly hyper-rectangular cube in part space
#[derive(Debug, Clone, PartialEq, Eq)]
struct PartCube([(usize, usize); 4]);

/// A disjoint union of PartCubes
#[derive(Debug, Clone, PartialEq, Eq)]
struct PartArea(CubeSet);

impl PartCube {
    /// Constructor. None if the part cube is empty.
    /// NOTE: Tested with assertions
    fn new(cube: [(usize, usize); 4]) -> Option<Self> {
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
        //Self::new(AXES.iter().map(|&val_char| (val_char, (1, 4001))).collect()).unwrap()
        Self::new([(1, 4001); 4]).unwrap()
    }

    /// Returns the number of parts in this area.
    /// WARNING: Not tested
    fn n_parts(&self) -> usize {
        let PartCube(cube) = self;
        cube.iter().map(|(min, max)| max - min).product()
    }

    /// Sample usage
    /// NOTE: Tested with assertions
    #[allow(unreachable_code)]
    fn sample(&self) -> impl Iterator<Item = Part> + '_ {
        todo!("Need to update sample to work with the new PartCube");
        iter::empty()
        //assert!(!self.empty());
        //let PartCube(cube) = self;
        //(0..4)
        //    .map(|axis| {
        //        let (min, max) = cube[axis];
        //        [min, (min + max) / 2, max - 1]
        //    })
        //    .multi_cartesian_product()
        //    .map(|vals| {
        //        let part = AXES
        //            .iter()
        //            .copied()
        //            .zip(vals)
        //            .collect::<HashMap<char, usize>>();
        //
        //        //assert!(self.contains_part(&part));
        //
        //        part
        //    })
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
        let (min, max) = cube[instruction.axis as usize];
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
            let mut new_cube = *cube;
            new_cube[instruction.axis as usize] = (new_min, new_max);
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
        cube.iter().enumerate().all(|(axis, &(min, max))| {
            let part_val = part[axis];
            min <= part_val && max > part_val
        })
    }

    /// NOTE: Tested with assertions
    fn contains_cube(&self, other: &PartCube) -> bool {
        let PartCube(self_cube) = self;
        let PartCube(other_cube) = other;
        let contains_cube = self_cube.iter().zip(other_cube.iter()).all(
            |((self_min, self_max), (other_min, other_max))| {
                self_min <= other_min && self_max >= other_max
            },
        );
        //let contains_cube = AXES.iter().all(|axis| {
        //    let (self_min, self_max) = self_cube.get(axis).unwrap();
        //    let (other_min, other_max) = other_cube.get(axis).unwrap();
        //    self_min <= other_min && self_max >= other_max
        //});
        //if contains_cube {
        //    assert!(other.sample().all(|part| self.contains_part(&part)));
        //}
        contains_cube
    }

    /// NOTE: Tested with assertions
    fn intersects(&self, other: &Self) -> bool {
        let PartCube(self_cube) = self;
        let PartCube(other_cube) = other;
        let intersects = self_cube.iter().zip(other_cube.iter()).all(
            |((self_min, self_max), (other_min, other_max))| {
                self_min < other_max && self_max > other_min
            },
        );
        //let intersects = AXES.iter().all(|axis| {
        //    let (self_min, self_max) = self_cube.get(axis).unwrap();
        //    let (other_min, other_max) = other_cube.get(axis).unwrap();
        //    self_min < other_max && self_max > other_min
        //});
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
        cube.iter().any(|(min, max)| min >= max)
    }
}

//impl Hash for PartCube {
//    /// WARNING: Not tested
//    fn hash<H: Hasher>(&self, state: &mut H) {
//        let PartCube(cube) = self;
//        for axis in AXES.iter() {
//            cube[axis].hash(state);
//        }
//    }
//}

/// The full area of the part space
/// WARNING: Not tested
#[cached]
fn part_area_all() -> Arc<PartArea> {
    Arc::new(PartArea::new([PartCube::all()].into_iter().collect()))
}

/// An empty area of the part space
/// WARNING: Not tested
#[cached]
fn part_area_none() -> Arc<PartArea> {
    Arc::new(PartArea::new(CubeSet::new()))
}

#[allow(unused_variables)]
impl PartArea {
    /// Construct a PartArea from a list of PartCubes.
    /// Also runs a bunch of validation checks.
    /// NOTE: Tested with assertions
    fn new(cubes: CubeSet) -> Self {
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
    #[allow(unreachable_code, clippy::let_and_return)]
    fn old_intersect(self: Arc<Self>, other: Arc<Self>) -> Arc<Self> {
        //println!("entered PartArea::intersect(..)");
        let intersection = if self == part_area_none() || other == part_area_none() {
            part_area_none()
        } else if self == part_area_all() {
            other.clone()
        } else if other == part_area_all() {
            self.clone()
        } else {
            assert!(!self.0.is_empty());
            assert!(!other.0.is_empty());
            Self::new(
                self.outer(&other)
                    .filter(|part_cube| {
                        self.contains_cube(part_cube) && other.contains_cube(part_cube)
                    })
                    .collect(),
            )
            .into()
        };
        //println!("- exited PartArea::intersect(..)");
        intersection
    }

    // NOTE: This is the old implementation, which works but is slow
    /// Returns the union of two PartAreas.
    /// NOTE: Tested with assertions
    #[allow(unreachable_code, clippy::let_and_return)]
    fn old_union(self: Arc<Self>, other: Arc<Self>) -> Arc<Self> {
        //println!("entered PartArea::union(..)");
        let union = if self == part_area_all() || other == part_area_all() {
            panic!("union case A");
            part_area_all()
        } else if self == part_area_none() {
            other.clone()
        } else if other == part_area_none() {
            self.clone()
        } else {
            assert!(!self.0.is_empty());
            assert!(!other.0.is_empty());
            Self::new(
                self.outer(&other)
                    .filter(|part_cube| {
                        self.contains_cube(part_cube) || other.contains_cube(part_cube)
                    })
                    .collect(),
            )
            .into()
        };
        //println!("- exited PartArea::union(..)");
        union
    }

    /// Returns the union of two PartAreas.
    /// NOTE: Tested with assertions
    fn union(self: Arc<Self>, other: Arc<Self>) -> Arc<Self> {
        //fn union(&self, other: &Self) -> Self {
        // TODO: Could I make this even faster by testing agains para_area_all() and part_area_none()?
        //PartArea::old_union(self, other)
        println!(
            "entered PartArea::union(..) self_cubes: {} other_cubes: {}",
            self.0.len(),
            other.0.len()
        );
        let union = PartArea::recursive(self, other, 0, PartArea::old_union);
        println!(
            "- exited PartArea::union(..): parts: {} cubes: {}",
            union.n_parts(),
            union.0.len()
        );
        union
    }

    /// Returns the union of two PartAreas.
    /// NOTE: Tested with assertions
    fn intersect(self: Arc<Self>, other: Arc<Self>) -> Arc<Self> {
        //fn union(&self, other: &Self) -> Self {
        // TODO: Could I make this even faster by testing agains para_area_all() and part_area_none()?
        //PartArea::old_union(self, other)
        println!(
            "entered PartArea::intersect(..) self_cubes: {} other_cubes: {}",
            self.0.len(),
            other.0.len()
        );
        let intersection = PartArea::recursive(self, other, 0, PartArea::old_intersect);
        println!(
            "- exited PartArea::intersect(..): parts: {} cubes: {}",
            intersection.n_parts(),
            intersection.0.len()
        );
        intersection
    }

    fn recursive(
        self: Arc<Self>,
        other: Arc<Self>,
        depth: usize,
        operation: fn(Arc<Self>, Arc<Self>) -> Arc<Self>,
    ) -> Arc<Self> {
        let self_cubes = &self.as_ref().0;
        let other_cubes = &other.as_ref().0;
        let n_self_cubes = self_cubes.len();
        let n_other_cubes = other_cubes.len();
        let max_depth = 100;
        let max_cells = 20;

        if depth >= max_depth || n_self_cubes * n_other_cubes < max_cells {
            return operation(self.clone(), other.clone());
        }

        // Find the split with the best balance
        let (self_left, self_right, other_left, other_right) = Axis::iter()
            .map(|axis| {
                // Get the min and max along this axis
                let mut indices: Vec<usize> = self_cubes
                    .iter()
                    .flat_map(|PartCube(cube)| {
                        let (min, max) = cube[axis as usize];
                        [min, max]
                    })
                    .chain(other_cubes.iter().flat_map(|PartCube(cube)| {
                        let (min, max) = cube[axis as usize];
                        [min, max]
                    }))
                    .collect();
                indices.sort_unstable();

                // Split the cubes along this axis
                let split_at = indices[indices.len() / 2];
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

                left_balance + right_balance
            })
            .unwrap();

        //assert!(
        //    self_left.0.len() != 0,
        //    "Error: self_left_len: {}, self_right_len: {}, other_left_len: {}, other_right_len: {}",
        //    self_left.0.len(),
        //    self_right.0.len(),
        //    other_left.0.len(),
        //    other_right.0.len()
        //);
        //assert!(
        //    self_right.0.len() != 0,
        //    "Error: self_left_len: {}, self_right_len: {}, other_left_len: {}, other_right_len: {}",
        //    self_left.0.len(),
        //    self_right.0.len(),
        //    other_left.0.len(),
        //    other_right.0.len()
        //);
        //assert!(
        //    other_left.0.len() != 0,
        //    "Error: self_left_len: {}, self_right_len: {}, other_left_len: {}, other_right_len: {}",
        //    self_left.0.len(),
        //    self_right.0.len(),
        //    other_left.0.len(),
        //    other_right.0.len()
        //);
        //assert!(
        //    other_right.0.len() != 0,
        //    "Error: self_left_len: {}, self_right_len: {}, other_left_len: {}, other_right_len: {}",
        //    self_left.0.len(),
        //    self_right.0.len(),
        //    other_left.0.len(),
        //    other_right.0.len()
        //);

        let self_left = Arc::new(self_left);
        let other_left = Arc::new(other_left);
        let result_left = PartArea::recursive(self_left, other_left, depth + 1, operation);

        let self_right = Arc::new(self_right);
        let other_right = Arc::new(other_right);
        let result_right = PartArea::recursive(self_right, other_right, depth + 1, operation);

        let left_iter = result_left.as_ref().0.iter();
        let right_iter = result_right.as_ref().0.iter();
        let result: CubeSet = left_iter.chain(right_iter).cloned().collect();
        Arc::new(PartArea::new(result))
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
        Axis::iter()
            .map(|axis| {
                let get_indices = |PartArea(cubes): &PartArea| {
                    cubes
                        .iter()
                        .flat_map(|PartCube(cube): &PartCube| {
                            let (min, max) = cube[axis as usize];
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
            .map(|ranges| PartCube::new([ranges[0], ranges[1], ranges[2], ranges[3]]).unwrap())
    }

    //fn contains_part(&self, part: &Part) -> bool {
    //    let PartArea(cubes) = self;
    //    cubes.iter().any(|cube| cube.contains_part(part))
    //}

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
        self: Arc<PartArea>,
        next_workflow: NextWorkflow,
        all_workflows: &AllWorkflows,
    ) -> Arc<Self> {
        PartArea::intersect(self, next_workflow.accepts(all_workflows))

        ////// debug - begin - test guard
        ////let test_part: Part = HashMap::from([('a', 2007), ('s', 537), ('m', 1), ('x', 2440)]);
        ////let test_workflow_names = ["px", "rfg"];
        ////// debug - end - test guard
        //
        //let AllWorkflows(workflows) = all_workflows;
        //
        //match next_workflow {
        //    NextWorkflow::Accept => self.clone(),
        //    NextWorkflow::Reject => part_area_none(),
        //    NextWorkflow::Workflow(workflow_name) => {
        //        let workflow = workflows.get(workflow_name).unwrap();
        //
        //        //// debug - begin - test test_part for the "in" workflow
        //        //if test_workflow_names.iter().contains(&workflow_name) {
        //        //    println!("\nworkflow_name: {}", workflow_name);
        //        //    println!("test_part: {:?}", test_part);
        //        //    println!("in self: {}", self.contains_part(&test_part));
        //        //    println!("workflow: {:?}", workflow);
        //        //}
        //        //// debug - end
        //
        //        let mut result = part_area_none();
        //        let mut remaining_area = self.clone();
        //
        //        //println!(
        //        //    "In workflow: {:?} result.size: {} remaining_area.size: {}",
        //        //    workflow_name,
        //        //    result.n_parts(),
        //        //    remaining_area.n_parts()
        //        //);
        //
        //        for (instruction, next_workflow) in &workflow.instructions {
        //            let (intersection, remainder) =
        //                remaining_area.intersect_instruction(instruction);
        //
        //            if !intersection.empty() {
        //                // TODO: Probably can get rid of this if statement above.
        //                result = result
        //                    .union(&intersection.intersect_workflow(*next_workflow, all_workflows))
        //            }
        //
        //            //// debug - begin - test test_part for the "in" workflow
        //            //if test_workflow_names.iter().contains(&workflow_name) {
        //            //    println!("\nworkflow_name: {}", workflow_name);
        //            //    println!("instruction: {:?}", instruction);
        //            //    println!(
        //            //        "in intersection: {}",
        //            //        intersection.contains_part(&test_part)
        //            //    );
        //            //    println!("in remainder: {}", remainder.contains_part(&test_part));
        //            //    println!("in result: {}", result.contains_part(&test_part));
        //            //    println!(
        //            //        "in reamining_area: {}",
        //            //        remaining_area.contains_part(&test_part)
        //            //    );
        //            //}
        //            //// debug - end
        //
        //            if remainder.empty() {
        //                return result;
        //            }
        //            remaining_area = remainder;
        //            //println!(
        //            //    "In workflow: {:?} result.size: {} remaining_area.size: {}",
        //            //    workflow_name,
        //            //    result.n_parts(),
        //            //    remaining_area.n_parts()
        //            //);
        //        }
        //
        //        //// debug - begin - test test_part for the "in" workflow
        //        //if test_workflow_names.iter().contains(&workflow_name) {
        //        //    println!("\nworkflow_name: {}", workflow_name);
        //        //    println!("about to compute fallback: {:?}", workflow.fallback);
        //        //    println!("in result: {}", result.contains_part(&test_part));
        //        //    println!(
        //        //        "in reamining_area: {}",
        //        //        remaining_area.contains_part(&test_part)
        //        //    );
        //        //}
        //        //// debug - end
        //
        //        if !remaining_area.empty() {
        //            let accepted_by_fallback =
        //                remaining_area.intersect_workflow(workflow.fallback, all_workflows);
        //
        //            //// debug - begin
        //            //if test_workflow_names.iter().contains(&workflow_name) {
        //            //    println!("\nworkflow_name: {}", workflow_name);
        //            //    println!("just checked accepted by fallback: {:?}", workflow.fallback);
        //            //    println!(
        //            //        "in accepted_by_fallback: {}",
        //            //        accepted_by_fallback.contains_part(&test_part)
        //            //    );
        //            //    println!("in result: {}", result.contains_part(&test_part));
        //            //}
        //            //// debug - end
        //
        //            result = result.union(&accepted_by_fallback);
        //
        //            //// debug - begin
        //            //if test_workflow_names.iter().contains(&workflow_name) {
        //            //    println!("in result (after): {}", result.contains_part(&test_part));
        //            //}
        //            // debug - end
        //        }
        //
        //        //// debug - begin - test test_part for the "in" workflow
        //        //if test_workflow_names.iter().contains(&workflow_name) {
        //        //    println!("\nworkflow_name: {}", workflow_name);
        //        //    println!("just computed fallback: {:?}", workflow.fallback);
        //        //    println!("in result: {}", result.contains_part(&test_part));
        //        //    println!(
        //        //        "in reamining_area: {}",
        //        //        remaining_area.contains_part(&test_part)
        //        //    );
        //        //}
        //        //// debug - end
        //
        //        //// debug - begin - test the results through sampling
        //        //assert!(result.sample().all(|part| self.contains_part(&part)
        //        //    && part_accepted_by_workflow(&part, next_workflow, workflows)));
        //        //let remaining_area = remaining_area.subtract(&result);
        //        //assert!(remaining_area.sample().all(|part| {
        //        //    if !self.contains_part(&part) {
        //        //        println!("part: {:?} not in self", part);
        //        //        return false;
        //        //    } else if part_accepted_by_workflow(&part, next_workflow, workflows) {
        //        //        println!("part: {:?} accepted by workflow", part);
        //        //        println!("workflow_name: {:?}", workflow_name);
        //        //        println!("workflow: {:?}", workflow);
        //        //        return false;
        //        //    }
        //        //    true
        //        //}));
        //        //println!(
        //        //    "Sucessfully tested {} samples in intersect_workflow(..)",
        //        //    result.sample().count() + remaining_area.sample().count()
        //        //);
        //        //// debug - end
        //
        //        result
        //}
        //}
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
    all_workflows: &AllWorkflows,
) -> bool {
    let workflows = &all_workflows.workflows;

    while let NextWorkflow::Workflow(workflow_name) = workflow {
        workflow = workflows.get(workflow_name).unwrap().run(part);
    }
    match workflow {
        NextWorkflow::Accept => true,
        NextWorkflow::Reject => false,
        _ => unreachable!("Invalid workflow: {:?}", workflow),
    }
}

fn solve_part_a(workflows: &AllWorkflows, parts: &Vec<Part>) -> usize {
    let mut answer: usize = 0;
    let workflow = NextWorkflow::Workflow("in");
    for part in parts {
        if part_accepted_by_workflow(part, workflow, workflows) {
            answer += part.iter().sum::<usize>();
        }
    }
    answer
}

#[allow(unused_variables)]
fn solve_part_b(workflows: &AllWorkflows) -> usize {
    let parts = part_area_all();
    let start_workflow = NextWorkflow::Workflow("in");
    let parts: Arc<PartArea> = parts.intersect_workflow(start_workflow, workflows);
    //println!("parts: {:?}", parts);
    let answer: usize = parts.n_parts();
    //println!("answer: {}", answer);
    answer
}

fn parse_input(input: &'static str) -> (AllWorkflows, Vec<Part>) {
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
                    let axis = Axis::from_char(axis);
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
            let part: HashMap<char, usize> = part[1..part.len() - 1]
                .split(",")
                .map(|val| {
                    let val_char = val.chars().next().unwrap();
                    let val_num = val[2..].parse::<usize>().unwrap();
                    (val_char, val_num)
                })
                .collect();
            [part[&'a'], part[&'s'], part[&'m'], part[&'x']]
        })
        .collect();

    (AllWorkflows::new(workflows), parts)
}
