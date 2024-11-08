#![allow(
    unused_imports,
    dead_code,
    clippy::type_complexity,
    unused_variables,
    unreachable_code,
    clippy::four_forward_slashes
)]

use advent_of_code_2023_in_rust::parse_regex::{parse_line, parse_lines};
use itertools::{izip, Itertools};
use regex::Regex;
use std::array;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

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
        split_axis: Axis,
        split_idx: usize,
        children: [Rc<Rule>; 2],
    },
}

/// A Part is a point in the 4D lattice [1,4001)^4
#[derive(Debug)]
struct Part([Pos; 4]);

/// An axis-aligned bounding box indexing positions along each axis
type Bounds = [(usize, usize); 4];

// Represents a volume of space in the part lattice [1,4001)^4
struct Volume {
    bounds: Bounds,
    contents: Contents,
}

// A Volume is either empty, full, or split along an axis
#[derive(Clone)]
enum Contents {
    Empty,
    Full,
    Split((Rc<Volume>, Rc<Volume>)),
}

fn main() {
    //let input = include_str!("../../puzzle_inputs/day_19.txt");
    let input = include_str!("../../puzzle_inputs/day_19_test.txt");
    let (puzzle, parts) = input.split_once("\n\n").unwrap();
    let puzzle = Puzzle::from_str(puzzle);
    let parts = Part::from_str(parts);

    // Solve 19a
    let sol_19a: u64 = solve_part_a(&puzzle, &parts);
    let correct_sol_19a: u64 = 19114;
    //let correct_sol_19a: u64 = 532551;
    println!("* 19a *");
    println!("My solution: {sol_19a}");
    println!("Correct solution: {correct_sol_19a}");
    println!("Equal: {}\n", sol_19a == correct_sol_19a);

    // Solve 19b
    let sol_19b: usize = solve_part_b(&puzzle);
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
            let accepts = puzzle.accepts_part(&puzzle.rule, part);
            if accepts {
                Some(part.score())
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_b(puzzle: &Puzzle) -> usize {
    let vol_full = puzzle.full_volume();
    println!("vol_full");
    println!("- bounds: {:?}", vol_full.bounds);
    println!("- idx_volume: {:?}", vol_full.idx_volume());
    println!("- measure: {:?}", puzzle.measure(&vol_full));
    let vol_soln = puzzle.rule.intersect(&vol_full);
    let answer = puzzle.measure(&vol_soln);
    println!("solve_part_b - Answer: {}", answer);
    todo!(
        "solve_part_b - Original bounds: {:?} Solution bounds: {:?}",
        vol_full.bounds,
        vol_soln.bounds
    );
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

    /// Returns true if the rule accepts the part
    fn accepts_part(&self, rule: &Rc<Rule>, part: &Part) -> bool {
        match &**rule {
            Rule::Accept => true,
            Rule::Reject => false,
            Rule::Split {
                split_axis,
                split_idx,
                children,
            } => {
                let part_pos = part[*split_axis as usize];
                let rule_pos = self.pos[*split_axis as usize][*split_idx];
                //println!(
                //    "testing axis:{:?} part:{:?} < rule:{:?}",
                //    axis, part_pos, rule_pos
                //);
                if part_pos < rule_pos {
                    self.accepts_part(&children[0], part)
                } else {
                    self.accepts_part(&children[1], part)
                }
            }
        }
    }

    /// Returns a volume that represents the entire part space
    fn full_volume(&self) -> Rc<Volume> {
        Rc::new(Volume {
            bounds: [
                (0, self.pos[0].len() - 1),
                (0, self.pos[1].len() - 1),
                (0, self.pos[2].len() - 1),
                (0, self.pos[3].len() - 1),
            ],
            contents: Contents::Full,
        })
    }

    /// Returns the measure (aka volume) of a Volume
    fn measure(&self, vol: &Volume) -> u64 {
        match &vol.contents {
            Contents::Empty => {
                println!("measure - Empty: {:?}", vol.bounds);
                0
            }
            Contents::Full => {
                let volume = izip!(&self.pos, vol.bounds)
                    .map(|(pos, (min_idx, max_idx))| {
                        let Pos(max) = pos[max_idx];
                        let Pos(min) = pos[min_idx];
                        max as u64 - min as u64
                    })
                    .product();

                println!("measure - Empty: {} {:?}", volume, vol.bounds);
                volume
            }
            Contents::Split((vol_a, vol_b)) => {
                let vol_a = self.measure(vol_a);
                let vol_b = self.measure(vol_b);
                let volume = vol_a + vol_b;
                println!(
                    "measure - Split: {} + {} = {} {:?}",
                    vol_a, vol_b, volume, vol.bounds
                );
                volume
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
                        split_axis: *axis,
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
                split_axis,
                split_idx,
                children,
            } => {
                println!(
                    "{}if {:?} < {:?} (idx = {}):",
                    indent, split_axis, self.pos[*split_axis as usize][*split_idx], split_idx
                );
                self.print_rules(&children[0], depth + 1);
                println!("{}else:", indent);
                self.print_rules(&children[1], depth + 1);
            }
        }
    }
}

impl Rule {
    /// Finds the intersection of this rule with another volume
    fn intersect(&self, vol: &Rc<Volume>) -> Rc<Volume> {
        match self {
            Rule::Accept => Rc::clone(vol),
            Rule::Reject => vol.empty(),
            Rule::Split {
                split_axis,
                split_idx,
                children,
            } => {
                let (vol_a, vol_b) = Volume::split_along(vol, *split_axis, *split_idx);
                Volume::union(
                    &children[0].intersect(&vol_a),
                    &children[1].intersect(&vol_b),
                )
            }
        }
    }
}

///// A rule is part of a decision tree that either accepts or rejects parts
//enum Rule {
//    Accept,
//    Reject,
//    Split {
//        axis: Axis,
//        split_idx: usize,
//        children: [Rc<Rule>; 2],
//    },
//}
//
///// A Part is a point in the 4D lattice [1,4001)^4
//#[derive(Debug)]
//struct Part([Pos; 4]);
//
//// Represents a volumne of space in the part lattice [1,4001)^4
//struct Volume {
//    min_max: [(usize, usize); 4],
//    contents: Contents,
//}
//
//// A Volume is either empty, full, or split along an axis
//enum Contents {
//}

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

impl Volume {
    /// Glues two volumes together, or None if they are incompatible
    fn glue(vol_left: Rc<Volume>, vol_right: Rc<Volume>) -> Option<Rc<Volume>> {
        // Compute the glue-together bounds and assert that they are compatible
        let mut split_axis = None;
        let bounds: Bounds = izip!(Axis::iter(), vol_left.bounds, vol_right.bounds)
            .map(|(axis, bounds_left, bounds_right)| {
                if bounds_left.1 == bounds_right.0 {
                    assert!(split_axis.is_none(), "Can only glue along one axis");
                    split_axis = Some(axis);
                    (bounds_left.0, bounds_right.1)
                } else if bounds_left == bounds_right {
                    bounds_left
                } else {
                    panic!(
                        "Incompatible bounds: {:?} and {:?} along axis {:?}",
                        bounds_left, bounds_right, axis
                    );
                }
            })
            .collect_vec()
            .try_into()
            .unwrap();
        split_axis.map(|split_axis| {
            // Glue both sides together, propogaiting Empty and Full contents the tree
            let result = Volume {
                bounds,
                contents: match (&vol_left.contents, &vol_right.contents) {
                    (Contents::Empty, Contents::Empty) => Contents::Empty,
                    (Contents::Full, Contents::Full) => Contents::Full,
                    _ => Contents::Split((vol_left.clone(), vol_right.clone())),
                },
            };

            // debug - begin - assert that the children have the correct bounds
            if let Some((axis, split_idx)) = result.get_split() {
                assert!(
                    axis == split_axis,
                    "Split axis should be the same as the glued axis"
                );
                let (bounds_left, bounds_right) = result.get_chidren_bounds().unwrap();
                assert!(vol_left.bounds == bounds_left);
                assert!(vol_right.bounds == bounds_right);
            }
            // debug - end

            Rc::new(result)
        })
    }

    /// Computes the union of two volumes
    fn union(self: &Rc<Self>, other: &Rc<Self>) -> Rc<Self> {
        let union = match (&self.contents, &other.contents) {
            (Contents::Empty, _) => other.clone(),
            (_, Contents::Empty) => self.clone(),
            (Contents::Full, _) => self.clone(),
            (_, Contents::Full) => other.clone(),
            (
                Contents::Split((self_left, self_right)),
                Contents::Split((other_left, other_right)),
            ) => {
                let left = self_left.union(other_left);
                let right = self_right.union(other_right);
                Volume::glue(left, right).unwrap()
            }
        };

        // debug - begin - sanity checks on the volume
        let vol_idx_self = self.idx_volume();
        let vol_idx_other = other.idx_volume();
        let vol_idx_union = union.idx_volume();
        println!(
            "union - self: {:?} other: {:?} union: {:?}",
            vol_idx_self, vol_idx_other, vol_idx_union
        );
        assert!(vol_idx_self == 0 || vol_idx_union > 0);
        assert!(vol_idx_other == 0 || vol_idx_union > 0);
        assert!(vol_idx_union <= vol_idx_self + vol_idx_other);
        // debug - end

        union
    }

    /// Computes the volume of this Volume in index space
    fn idx_volume(&self) -> usize {
        match &self.contents {
            Contents::Empty => 0,
            Contents::Full => self.bounds.iter().map(|(min, max)| (max - min)).product(),
            Contents::Split((left, right)) => left.idx_volume() + right.idx_volume(),
        }
    }

    /// Splits ths volume along an axis
    fn split_along(self: &Rc<Self>, split_axis: Axis, split_idx: usize) -> (Rc<Self>, Rc<Self>) {
        //println!(
        //    "split_along - split_axis: {:?} split_idx: {:?} bounds: {:?} my_split: {:?}",
        //    split_axis,
        //    split_idx,
        //    self.bounds,
        //    self.get_split()
        //);
        let (min_idx, max_idx) = self.bounds[split_axis as usize];
        let (vol_a, vol_b): (Rc<Volume>, Rc<Volume>) = if split_idx <= min_idx {
            (self.empty(), self.clone())
        } else if split_idx >= max_idx {
            (self.clone(), self.empty())
        } else {
            let (vol_split_axis, vol_split_idx) = self.get_split().unwrap_or_else(|| {
                        panic!(
                            "Splitting without children - Case A. split_axis: {:?} split_idx: {:?} bounds: {:?}",
                            split_axis, split_idx, self.bounds
                        )
                    });
            if split_axis != vol_split_axis {
                let (child_left, child_right) = self.get_children().unwrap();
                let (child_left_a, child_left_b) = child_left.split_along(split_axis, split_idx);
                let (child_right_a, child_right_b) = child_right.split_along(split_axis, split_idx);
                (
                    Volume::glue(child_left_a, child_right_a).unwrap(),
                    Volume::glue(child_left_b, child_right_b).unwrap(),
                )
            } else if vol_split_idx < split_idx {
                assert!(split_axis == vol_split_axis);
                let (child_left, child_right) = self.get_children().unwrap();
                let (child_left_a, child_left_b) = child_left.split_along(split_axis, split_idx);
                (
                    Volume::glue(child_left_a, child_right.empty()).unwrap(),
                    Volume::glue(child_left_b, child_right.empty()).unwrap(),
                )
            } else if vol_split_idx > split_idx {
                assert!(split_axis == vol_split_axis);
                let (child_left, child_right) = self.get_children().unwrap();
                let (child_right_a, child_right_b) = child_right.split_along(split_axis, split_idx);
                let (child_left, child_right) = self.get_children().unwrap();
                (
                    Volume::glue(child_left.empty(), child_right_a).unwrap(),
                    Volume::glue(child_left.empty(), child_right_b).unwrap(),
                )
            } else {
                assert!(split_axis == vol_split_axis);
                assert!(vol_split_idx == split_idx);
                let (child_left, child_right) = self.get_children().unwrap();
                (
                    Volume::glue(child_left.clone(), child_right.empty()).unwrap(),
                    Volume::glue(child_left.empty(), child_right.clone()).unwrap(),
                )
            }
        };

        // debug - being - assert that the bounds make sense
        assert!(vol_a.bounds == self.bounds);
        assert!(vol_b.bounds == self.bounds);
        (vol_a, vol_b)
        // debug - end
    }

    /// Returns chileren
    fn get_children(&self) -> Option<(Rc<Volume>, Rc<Volume>)> {
        if let Contents::Split(children) = &self.contents {
            Some(children.clone())
        } else {
            self.get_chidren_bounds()
                .map(|(min_max_idx_left, min_max_idx_right)| {
                    let left = Rc::new(Volume {
                        bounds: min_max_idx_left,
                        contents: self.contents.clone(),
                    });
                    let right = Rc::new(Volume {
                        bounds: min_max_idx_right,
                        contents: self.contents.clone(),
                    });
                    (left, right)
                })
        }

        //// debug - begin - assert that the children have the correct bounds
        //if let Some((child_left, child_right)) = &children {
        //    let (bounds_left, bounds_right) = self.get_chidren_bounds().unwrap();
        //    assert!(child_left.bounds == bounds_left);
        //    assert!(child_right.bounds == bounds_right);
        //}
        //// debug - end
    }

    /// If the Volume can be further split, then return its split axis and index.
    /// An invariant is that the children must respect this split.
    fn get_split(&self) -> Option<(Axis, usize)> {
        let (axis, min_idx, max_idx) = Axis::iter()
            .map(|axis| {
                let (min_idx, max_idx) = self.bounds[axis as usize];
                (axis, min_idx, max_idx)
            })
            .max_by_key(|(_, min_idx, max_idx)| max_idx - min_idx)
            .unwrap();
        if min_idx + 1 < max_idx {
            let mid_idx = (min_idx + max_idx) / 2;
            assert!(min_idx < mid_idx);
            assert!(mid_idx < max_idx);
            Some((axis, mid_idx))
        } else {
            assert!(self.bounds[0].0 + 1 == self.bounds[0].1);
            assert!(self.bounds[1].0 + 1 == self.bounds[1].1);
            assert!(self.bounds[2].0 + 1 == self.bounds[2].1);
            assert!(self.bounds[3].0 + 1 == self.bounds[3].1);
            None
        }
    }

    fn get_chidren_bounds(&self) -> Option<(Bounds, Bounds)> {
        self.get_split().map(|(axis, split_idx)| {
            let mut min_max_idx_left = self.bounds;
            min_max_idx_left[axis as usize].1 = split_idx;
            let mut min_max_idx_right = self.bounds;
            min_max_idx_right[axis as usize].0 = split_idx;
            (min_max_idx_left, min_max_idx_right)
        })
    }

    /// Returns an empty version of this volume
    fn empty(&self) -> Rc<Volume> {
        Rc::new(Volume {
            bounds: self.bounds,
            contents: Contents::Empty,
        })
    }
}

impl Contents {}
