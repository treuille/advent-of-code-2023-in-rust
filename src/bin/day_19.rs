#![allow(
    unused_imports,
    dead_code,
    clippy::type_complexity,
    unused_variables,
    unreachable_code,
    clippy::four_forward_slashes,
    clippy::let_and_return
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

type Rect = [(Pos, Pos); 4];

fn rect_vol(rect: &Rect) -> u64 {
    rect.iter()
        .map(|(min_pos, max_pos)| (max_pos.0 as u64).checked_sub(min_pos.0 as u64).unwrap())
        .product()
}
//
// Represents a volume of space in the part lattice [1,4001)^4
#[derive(PartialEq, Eq)]
struct Volume {
    bounds: Bounds,
    contents: Contents,
}

// A Volume is either empty, full, or split along an axis
#[derive(Clone, PartialEq, Eq)]
enum Contents {
    Empty,
    Full,
    Split((Rc<Volume>, Rc<Volume>)),
}

fn main() {
    let input = include_str!("../../puzzle_inputs/day_19.txt");
    //let input = include_str!("../../puzzle_inputs/day_19_test.txt");
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
    let sol_19b: u64 = solve_part_b(&puzzle);
    let correct_sol_19b: u64 = 167409079868000;
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

fn solve_part_b(puzzle: &Puzzle) -> u64 {
    let vol_full = puzzle.full_volume();
    println!("vol_full:\n{:?}\n", vol_full);

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

    println!("vol_full");
    println!("- measure: {:?}", puzzle.measure(&vol_full));

    let rect: Rect = izip!(&puzzle.pos, vol_full.bounds)
        .map(|(pos, (min, max))| (pos[min], pos[max]))
        .collect_vec()
        .try_into()
        .unwrap();
    //println!("rect: {:?}", rect);
    //println!("vol: {}", rect_vol(&rect));
    let vol_soln_2 = puzzle.accepts_vol(&rect, &puzzle.rule);

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

    /// Returns the volume of the subspace of the bounds that this rule accepts
    fn accepts_vol(&self, rect: &Rect, rule: &Rc<Rule>) -> u64 {
        if rect_vol(rect) == 0 {
            0
        } else {
            match &**rule {
                Rule::Accept => rect_vol(rect),
                Rule::Reject => 0,
                Rule::Split {
                    split_axis,
                    split_idx,
                    children,
                } => {
                    let split_axis = *split_axis as usize;
                    let split_pos = self.pos[split_axis][*split_idx];
                    //let split_pos = Pos(split_pos.0 + 1);

                    let mut rect_left = *rect;
                    assert!(rect_left == *rect);

                    let mut rect_right = *rect;
                    assert!(rect_right == *rect);

                    rect_left[split_axis].1 = split_pos;
                    assert!(rect_left != *rect);
                    assert!(rect_right == *rect);

                    rect_right[split_axis].0 = split_pos;
                    assert!(rect_left != *rect);
                    assert!(rect_right != *rect);
                    assert!(rect_vol(&rect_left) + rect_vol(&rect_right) == rect_vol(rect));

                    let vol_left = self.accepts_vol(&rect_left, &children[0]);
                    let vol_right = self.accepts_vol(&rect_right, &children[1]);

                    vol_left + vol_right
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
                //println!("measure - Empty: {:?}", vol.bounds);
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

                //println!("measure - Full: {} {:?}", volume, vol.bounds);
                volume
            }
            Contents::Split((vol_a, vol_b)) => {
                let vol_a = self.measure(vol_a);
                let vol_b = self.measure(vol_b);
                let volume = vol_a + vol_b;
                //println!(
                //    "measure - Split: {} + {} = {} {:?}",
                //    vol_a, vol_b, volume, vol.bounds
                //);
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
    fn intersect(&self, vol: &Rc<Volume>, depth: usize) -> Rc<Volume> {
        let prefix = "  ".repeat(depth);
        println!();
        println!("{}intersect - ENTER - rule: {:?}", prefix, self);
        println!(
            "{}intersect - ENTER - vol: {} nodes: {}",
            prefix,
            vol.idx_volume(),
            vol.nodes()
        );
        if vol.idx_volume() == 0 {
            assert!(vol.contents == Contents::Empty);
            println!("{}intersect - EXIT - rule: {:?}", prefix, self);
            println!(
                "{}intersect - EXIT EMPTY - vol: {}",
                prefix,
                vol.idx_volume()
            );
            //println!();
            return Rc::clone(vol);
        };

        match self {
            Rule::Accept => Rc::clone(vol),
            Rule::Reject => Volume::empty(&vol.bounds),
            Rule::Split {
                split_axis,
                split_idx,
                children,
            } => {
                let [half_a, half_b] = Volume::halfspaces(&vol.bounds, *split_axis, *split_idx);

                // debug - begin
                let half_a_nodes = half_a.nodes();
                let half_b_nodes = half_b.nodes();
                // debug - end

                // Intersect the incoming volume with the halfspaces
                let vol_a = Volume::intersect(vol, &half_a);
                let vol_b = Volume::intersect(vol, &half_b);

                // debug - begin - make sure that we've split vol porperly
                let vol_nodes = vol.nodes();
                let vol_a_nodes = vol_a.nodes();
                let vol_b_nodes = vol_b.nodes();
                println!(
                    "{}rule split: {} /\\ ({} + {}) => ({} + {}) = {}",
                    prefix,
                    vol_nodes,
                    half_a_nodes,
                    half_b_nodes,
                    vol_a_nodes,
                    vol_b_nodes,
                    vol_a_nodes + vol_b_nodes,
                );
                //assert!(vol_nodes == vol_a_nodes + vol_b_nodes);
                // debug - end

                // Now recurse into the children
                let vol_a = children[0].intersect(&vol_a, depth + 1);
                let vol_b = children[1].intersect(&vol_b, depth + 1);

                // Union the results
                let result = Volume::union(&vol_a, &vol_b);

                println!("{}intersect - EXIT - rule: {:?}", prefix, self);
                println!(
                    "{}intersect - EXIT UNION vol: {} nodes: {}",
                    prefix,
                    result.idx_volume(),
                    result.nodes()
                );
                //println!();

                // all done
                result
            }
        }
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn child_str(rule: &Rc<Rule>) -> &str {
            match &**rule {
                Rule::Accept => "Accept",
                Rule::Reject => "Reject",
                _ => "Split",
            }
        }
        match self {
            Rule::Accept => write!(f, "Accept"),
            Rule::Reject => write!(f, "Reject"),
            Rule::Split {
                split_axis,
                split_idx,
                children,
            } => write!(
                f,
                "if {:?} < {:?} : {:?} else: {:?}",
                split_axis,
                split_idx,
                child_str(&children[0]),
                child_str(&children[1])
            ),
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
            if let Some((axis, split_idx)) = Volume::get_split(&result.bounds) {
                assert!(
                    axis == split_axis,
                    "Split axis should be the same as the glued axis"
                );
                let (bounds_left, bounds_right) =
                    Volume::get_chidren_bounds(&result.bounds).unwrap();
                assert!(vol_left.bounds == bounds_left);
                assert!(vol_right.bounds == bounds_right);
            }
            // debug - end

            Rc::new(result)
        })
    }

    /// Computes the union of two volumes
    fn union(self: &Rc<Self>, other: &Rc<Self>) -> Rc<Self> {
        assert_eq!(
            self.bounds, other.bounds,
            "Bounds {:?} and {:?} must be the same",
            self.bounds, other.bounds
        );

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
        //println!(
        //    "union - self: {:?} other: {:?} union: {:?}",
        //    vol_idx_self, vol_idx_other, vol_idx_union
        //);
        assert!(vol_idx_self == 0 || vol_idx_union > 0);
        assert!(vol_idx_other == 0 || vol_idx_union > 0);
        assert!(vol_idx_union <= vol_idx_self + vol_idx_other);
        assert!(vol_idx_union >= vol_idx_self.min(vol_idx_other));
        //println!(
        //    "union: {} \\/ {} => {} (min: {})",
        //    vol_idx_self,
        //    vol_idx_other,
        //    vol_idx_union,
        //    vol_idx_self.min(vol_idx_other)
        //);
        // debug - end

        union
    }

    /// Computes the intersection of this volume with another
    fn intersect(self: &Rc<Self>, other: &Rc<Self>) -> Rc<Self> {
        assert_eq!(
            self.bounds, other.bounds,
            "Bounds {:?} and {:?} must be the same",
            self.bounds, other.bounds
        );

        let intersection = match (&self.contents, &other.contents) {
            (Contents::Empty, _) => self.clone(),
            (_, Contents::Empty) => other.clone(),
            (Contents::Full, _) => other.clone(),
            (_, Contents::Full) => self.clone(),
            (
                Contents::Split((self_left, self_right)),
                Contents::Split((other_left, other_right)),
            ) => {
                let left = self_left.intersect(other_left);
                let right = self_right.intersect(other_right);
                Volume::glue(left, right).unwrap()
            }
        };

        // debug - begin - sanity checks on the volume
        let vol_idx_self = self.idx_volume();
        let vol_idx_other = other.idx_volume();
        let vol_idx_intersection = intersection.idx_volume();
        //println!(
        //    "intersection - self: {:?} other: {:?} union: {:?}",
        //    vol_idx_self, vol_idx_other, vol_idx_intersection
        //);
        assert!(vol_idx_self > 0 || vol_idx_intersection == 0);
        assert!(vol_idx_other > 0 || vol_idx_intersection == 0);
        assert!(vol_idx_intersection <= vol_idx_self);
        assert!(vol_idx_intersection <= vol_idx_other);
        assert!(vol_idx_intersection <= vol_idx_self.max(vol_idx_other));
        //println!(
        //    "intersect: {} /\\ {} => {} (max: {})",
        //    vol_idx_self,
        //    vol_idx_other,
        //    vol_idx_intersection,
        //    vol_idx_self.max(vol_idx_other)
        //);
        // debug - end

        intersection
    }

    /// Computes the volume of this Volume in index space
    fn idx_volume(&self) -> usize {
        match &self.contents {
            Contents::Empty => 0,
            Contents::Full => self.bounds.iter().map(|(min, max)| (max - min)).product(),
            Contents::Split((left, right)) => left.idx_volume() + right.idx_volume(),
        }
    }

    /// Computes the number of nodes in this `Volume`
    fn nodes(&self) -> usize {
        match &self.contents {
            Contents::Split((left, right)) => left.nodes() + right.nodes(),
            _ => 1,
        }
    }

    /// Splits ths volume along an axis
    //fn split_along(self: &Rc<Self>, split_axis: Axis, split_idx: usize) -> (Rc<Self>, Rc<Self>) {
    //    println!(
    //        "split_along - split_axis: {:?} split_idx: {:?} bounds: {:?} my_split: {:?}",
    //        split_axis,
    //        split_idx,
    //        self.bounds,
    //        self.get_split()
    //    );
    //    let (vol_a, vol_b): (Rc<Volume>, Rc<Volume>) =
    //        if let Some((vol_split_axis, vol_split_idx)) = self.get_split() {
    //            let (min_idx, max_idx) = self.bounds[split_axis as usize];
    //            if split_axis != vol_split_axis {
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                let (child_left_a, child_left_b) =
    //                    child_left.split_along(split_axis, split_idx);
    //                let (child_right_a, child_right_b) =
    //                    child_right.split_along(split_axis, split_idx);
    //                (
    //                    Volume::glue(child_left_a, child_right_a).unwrap(),
    //                    Volume::glue(child_left_b, child_right_b).unwrap(),
    //                )
    //            } else if split_idx <= min_idx {
    //                assert!(split_axis == vol_split_axis);
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                let (_, child_left_b) = child_left.split_along(split_axis, split_idx);
    //                let (_, child_right_b) = child_right.split_along(split_axis, split_idx);
    //                (
    //                    self.empty(),
    //                    Volume::glue(child_left_b, child_right_b).unwrap(),
    //                )
    //            } else if split_idx >= max_idx {
    //                assert!(split_axis == vol_split_axis);
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                let (child_left_a, _) = child_left.split_along(split_axis, split_idx);
    //                let (child_right_a, _) = child_right.split_along(split_axis, split_idx);
    //                (
    //                    Volume::glue(child_left_a, child_right_a).unwrap(),
    //                    self.empty(),
    //                )
    //            } else if vol_split_idx < split_idx {
    //                assert!(split_axis == vol_split_axis);
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                let (child_left_a, child_left_b) =
    //                    child_left.split_along(split_axis, split_idx);
    //                (
    //                    Volume::glue(child_left_a, child_right.empty()).unwrap(),
    //                    Volume::glue(child_left_b, child_right.empty()).unwrap(),
    //                )
    //            } else if vol_split_idx > split_idx {
    //                assert!(split_axis == vol_split_axis);
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                let (child_right_a, child_right_b) =
    //                    child_right.split_along(split_axis, split_idx);
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                (
    //                    Volume::glue(child_left.empty(), child_right_a).unwrap(),
    //                    Volume::glue(child_left.empty(), child_right_b).unwrap(),
    //                )
    //            } else {
    //                assert!(split_axis == vol_split_axis);
    //                assert!(vol_split_idx == split_idx);
    //                let (child_left, child_right) = self.get_children().unwrap();
    //                (
    //                    Volume::glue(child_left.clone(), child_right.empty()).unwrap(),
    //                    Volume::glue(child_left.empty(), child_right.clone()).unwrap(),
    //                )
    //            }
    //        } else {
    //            panic!("This is wrong.. thie logic in this function is all wrong...");
    //            (self.clone(), self.clone()
    //        };
    //
    //    // debug - being - assert that the bounds make sense
    //    assert!(vol_a.bounds == self.bounds);
    //    assert!(vol_b.bounds == self.bounds);
    //    (vol_a, vol_b)
    //    // debug - end
    //}

    /// Returns chileren
    fn get_children(&self) -> Option<(Rc<Volume>, Rc<Volume>)> {
        if let Contents::Split(children) = &self.contents {
            Some(children.clone())
        } else {
            Volume::get_chidren_bounds(&self.bounds).map(|(min_max_idx_left, min_max_idx_right)| {
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

    fn halfspaces(bounds: &Bounds, axis: Axis, split_idx: usize) -> [Rc<Volume>; 2] {
        if split_idx < bounds[axis as usize].0 {
            return [Volume::empty(bounds), Volume::full(bounds)];
        } else if split_idx >= bounds[axis as usize].1 {
            return [Volume::full(bounds), Volume::empty(bounds)];
        }

        // debug - begin - assert that the split is within the bounds
        assert!(
            bounds[axis as usize].0 < split_idx,
            "Split index too low - split_idx: {:?} axis: {:?} bounds: {:?}",
            split_idx,
            axis,
            bounds
        );
        assert!(
            split_idx < bounds[axis as usize].1,
            "Split index too high - {:?} < {:?} bounds: {:?}",
            axis,
            split_idx,
            bounds
        );
        // debug - end

        let (bounds_left, bounds_right) = Volume::get_chidren_bounds(bounds).unwrap();

        //// debug - begin - assert that the split is within the bounds
        //assert!(
        //    bounds_left[axis as usize].0 <= split_idx,
        //    "Split index too low - {:?} < {:?} bounds: {:?} - split LEFT from: {:?}",
        //    axis,
        //    split_idx,
        //    bounds_left,
        //    bounds
        //);
        //assert!(
        //    split_idx <= bounds_left[axis as usize].1,
        //    "Split index too high - {:?} < {:?} bounds: {:?} - split LEFT from: {:?}",
        //    axis,
        //    split_idx,
        //    bounds_left,
        //    bounds
        //);
        //assert!(
        //    bounds_right[axis as usize].0 <= split_idx,
        //    "Split index too low - {:?} < {:?} bounds: {:?} - split RIGHT from: {:?}",
        //    axis,
        //    split_idx,
        //    bounds_right,
        //    bounds
        //);
        //assert!(
        //    split_idx <= bounds_right[axis as usize].1,
        //    "Split index too high - {:?} < {:?} bounds: {:?} - split RIGHT from: {:?}",
        //    axis,
        //    split_idx,
        //    bounds_right,
        //    bounds
        //);
        //// debug - end

        let (left_a, right_a, left_b, right_b) =
            if Some((axis, split_idx)) == Volume::get_split(bounds) {
                (
                    Volume::full(&bounds_left),
                    Volume::empty(&bounds_right),
                    Volume::empty(&bounds_left),
                    Volume::full(&bounds_right),
                )
            } else {
                let [left_a, left_b] = Volume::halfspaces(&bounds_left, axis, split_idx);
                let [right_a, right_b] = Volume::halfspaces(&bounds_right, axis, split_idx);
                (left_a, right_a, left_b, right_b)
            };
        let vol_a = Volume::glue(left_a, right_a).unwrap();
        let vol_b = Volume::glue(left_b, right_b).unwrap();

        //// debug - begin - test that the union is correct
        //let union = Volume::union(&vol_a, &vol_b);
        //assert!(union.bounds == *bounds);
        //assert!(union.contents == Contents::Full);
        //// debug - end
        //
        //// debug - begin - test that the intersection is correct
        //let intersection = Volume::intersect(&vol_a, &vol_b);
        //assert!(intersection.bounds == *bounds);
        //assert!(intersection.contents == Contents::Empty);
        //// debug - end
        //
        // All done
        [vol_a, vol_b]
    }

    /// If the Volume can be further split, then return its split axis and index.
    /// An invariant is that the children must respect this split.
    fn get_split(bounds: &Bounds) -> Option<(Axis, usize)> {
        let (axis, min_idx, max_idx) = Axis::iter()
            .map(|axis| {
                let (min_idx, max_idx) = bounds[axis as usize];
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
            assert!(bounds[0].0 + 1 == bounds[0].1);
            assert!(bounds[1].0 + 1 == bounds[1].1);
            assert!(bounds[2].0 + 1 == bounds[2].1);
            assert!(bounds[3].0 + 1 == bounds[3].1);
            None
        }
    }

    fn get_chidren_bounds(bounds: &Bounds) -> Option<(Bounds, Bounds)> {
        Volume::get_split(bounds).map(|(axis, split_idx)| {
            let mut min_max_idx_left = *bounds;
            min_max_idx_left[axis as usize].1 = split_idx;
            let mut min_max_idx_right = *bounds;
            min_max_idx_right[axis as usize].0 = split_idx;
            (min_max_idx_left, min_max_idx_right)
        })
    }

    /// Returns an empty version of this volume
    fn empty(&bounds: &Bounds) -> Rc<Volume> {
        Rc::new(Volume {
            bounds,
            contents: Contents::Empty,
        })
    }

    /// Returns an empty version of this volume
    fn full(&bounds: &Bounds) -> Rc<Volume> {
        Rc::new(Volume {
            bounds,
            contents: Contents::Full,
        })
    }
}

impl Debug for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn fmt(vol: &Volume, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
            let prefix = "  ".repeat(depth);
            write!(f, "{}vol {:?} ", prefix, vol.bounds)?;
            if let Some((axis, split_idx)) = Volume::get_split(&vol.bounds) {
                writeln!(f, "split: {:?} < {:?}", axis, split_idx)?;
            } else {
                writeln!(f, "leaf")?;
            };
            match &vol.contents {
                Contents::Empty => writeln!(f, "{} Empty", prefix),
                Contents::Full => writeln!(f, "{} Full", prefix),
                Contents::Split((vol_a, vol_b)) => {
                    writeln!(f, " Split left:")?;
                    fmt(vol_a, f, depth + 1)?;
                    writeln!(f, " Split right:")?;
                    fmt(vol_b, f, depth + 1)?;
                    Ok(())
                }
            }
        }
        fmt(self, f, 0)
        //struct Volume {
        //    bounds: Bounds,
        //    contents: Contents,
        //}
    }
}

impl Contents {}
