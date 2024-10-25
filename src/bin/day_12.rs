#![allow(clippy::obfuscated_if_else)]
use itertools::Itertools;
use std::iter;

/// A row of springs, eiher '.' (operational), '#' (damaged), or '?' (unknown)
type Row = String;

/// A list of contiguous damaged springs in a row
type DamagedSprings = Vec<usize>;

/// I discovered two solutions to this problem:
///
/// 1. A dynamic programming approach, where you scan from the left
///    matching damaged springs and branching on every '?'
///
/// 2. A divide-and-conquer approach, where you split the row into three
///    parts. The split occurs implicitly at a string of the form
///    ".#####." where the "#"s match the length of center-most element
///    of `damaged_springs`.
///
/// The first solution is too slow unless you memoize the results. (I put
/// that first solution in the `fast_dynamic_memoize` branch.) This file
/// contains the second solution. It turns out it's slower than the first,
/// but in some ways more interesting because it reflects the combinatorial
/// structure of the problem more deeply.
///
/// Now I'm going to work on a version which doesn't do any allocations.
/// Before: 5.55s
fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_12.txt");

    // Solve 12a
    let puzzle = parse_input(input);
    let sol_12a: usize = solve(&puzzle);
    let correct_sol_12a: usize = 7622;
    println!("* 12a *");
    println!("My solution: {sol_12a}");
    println!("Correct solution: {correct_sol_12a}");
    println!("Equal: {}\n", sol_12a == correct_sol_12a);

    // Solve 12b
    let puzzle = increase_puzzle_size(puzzle);
    let sol_12b: usize = solve(&puzzle);
    let correct_sol_12b: usize = 4964259839627;
    println!("* 12b *");
    println!("My solution: {sol_12b}");
    println!("Correct solution: {correct_sol_12b}");
    println!("Equal: {}\n", sol_12b == correct_sol_12b);
}

fn solve(puzzle: &[(Row, DamagedSprings)]) -> usize {
    puzzle
        .iter()
        .map(|(row, damaged_springs)| {
            let spring_sums = SpringSums::new(row);
            count_arrangements(
                row,
                damaged_springs.clone(),
                &SpringSumsSlice::new(&spring_sums),
            )
        })
        .sum()
}

struct SpringSums {
    num_not_operational: Vec<usize>, // Running sum of '.' (from the right)
    num_not_damaged: Vec<usize>,     // Running sum of '.' (from the right)
                                     //n_operational: Vec<usize>,       // Running sum of '.' (from the right)
                                     //n_damaged: Vec<usize>,           // Running sum of '#' (from the right)
                                     //n_unknown: Vec<usize>,           // Running sum of '?' (from the right)
}

// This is an acceleration data structure which lets met quickly count the number of
// ?s .? and #? in a row of springs. It stores running sums starting from the right of the
// row.
#[allow(dead_code)]
struct SpringSumsSlice<'a> {
    owned_vecs: &'a SpringSums,
    start: usize,
    end: usize,
}
impl SpringSums {
    fn new(row: &str) -> Self {
        let mut num_not_operational: Vec<usize> = vec![0; row.len() + 1];
        let mut num_not_damaged: Vec<usize> = vec![0; row.len() + 1];
        for (i, c) in row.chars().rev().enumerate() {
            let i = row.len() - i;
            num_not_operational[i - 1] = num_not_operational[i] + (c != '.') as usize;
            num_not_damaged[i - 1] = num_not_damaged[i] + (c != '#') as usize;
        }

        //// TODO: I can maybe make this more elegantly by doing a reverse loop
        //// with a match statement over the characater.
        //let n_operational: Vec<usize> = iter::once(0)
        //    .chain(row.chars().rev().scan(0, |acc, c| {
        //        *acc += (c == '.').then_some(1).unwrap_or(0);
        //        Some(*acc)
        //    }))
        //    .collect();
        //
        //let n_damaged: Vec<usize> = iter::once(0)
        //    .chain(row.chars().rev().scan(0, |acc, c| {
        //        *acc += (c == '#').then_some(1).unwrap_or(0);
        //        Some(*acc)
        //    }))
        //    .collect();
        //
        //let n_unknown: Vec<usize> = iter::once(0)
        //    .chain(row.chars().rev().scan(0, |acc, c| {
        //        *acc += (c == '?').then_some(1).unwrap_or(0);
        //        Some(*acc)
        //    }))
        //    .collect();

        SpringSums {
            num_not_operational,
            num_not_damaged,
            //n_operational: n_operational.into_iter().rev().collect(),
            //n_damaged: n_damaged.into_iter().rev().collect(),
            //n_unknown: n_unknown.into_iter().rev().collect(),
        }
    }
}

#[allow(dead_code)]
impl<'a> SpringSumsSlice<'a> {
    fn new(owned_vecs: &'a SpringSums) -> Self {
        // There is a single length
        let len = owned_vecs.num_not_operational.len();
        assert_eq!(owned_vecs.num_not_damaged.len(), len);
        //assert_eq!(owned_vecs.n_unknown.len(), len);

        SpringSumsSlice {
            owned_vecs,
            start: 0,
            end: len,
        }
    }

    fn len(&self) -> usize {
        assert!(self.end >= self.start);
        self.end - self.start
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        assert!(start <= end);
        assert!(end <= self.owned_vecs.num_not_operational.len());

        SpringSumsSlice {
            owned_vecs: self.owned_vecs,
            start: self.start + start,
            end: self.start + end,
        }
    }

    fn slice_left(&self, n: usize) -> Self {
        let slice = SpringSumsSlice {
            owned_vecs: self.owned_vecs,
            start: self.start,
            end: self.start + n,
        };
        //println!(
        //    "slice_left: ({} {}) -{}-> ({} {})",
        //    self.start, self.end, n, slice.start, slice.end
        //);
        assert!(slice.start <= slice.end);
        slice
    }

    fn slice_right(&self, n: usize) -> Self {
        let slice = SpringSumsSlice {
            owned_vecs: self.owned_vecs,
            start: self.start + n,
            end: self.end,
        };
        //println!(
        //    "slice_right: ({} {}) -{}-> ({} {})",
        //    self.start, self.end, n, slice.start, slice.end
        //);
        assert!(slice.start <= slice.end);
        slice
    }

    fn slice_center(&self, start: usize, end: usize) -> Self {
        let slice = SpringSumsSlice {
            owned_vecs: self.owned_vecs,
            start: self.start + start,
            end: self.start + end,
        };
        assert!(slice.start <= slice.end);
        slice
    }

    fn n_not_damaged(&self) -> usize {
        //let n_operational =
        //    self.owned_vecs.n_operational[self.start] - self.owned_vecs.n_operational[self.end - 1];
        //let n_unknown =
        //    self.owned_vecs.n_unknown[self.start] - self.owned_vecs.n_unknown[self.end - 1];
        //let answer = n_operational + n_unknown;
        //assert_eq!(
        //    answer,
        self.owned_vecs.num_not_damaged[self.start] - self.owned_vecs.num_not_damaged[self.end - 1]
        //);
        //answer
    }

    fn n_not_operational(&self) -> usize {
        //let n_damaged =
        //    self.owned_vecs.n_damaged[self.start] - self.owned_vecs.n_damaged[self.end - 1];
        //let n_unknown =
        //    self.owned_vecs.n_unknown[self.start] - self.owned_vecs.n_unknown[self.end - 1];
        //n_damaged + n_unknown
        self.owned_vecs.num_not_operational[self.start]
            - self.owned_vecs.num_not_operational[self.end - 1]
    }

    //fn count(&self, i: usize, j: usize) -> (usize, usize, usize) {
    //    (
    //        self.n_operational[i] - self.n_operational[j],
    //        self.n_damaged[i] - self.n_damaged[j],
    //        self.n_unknown[i] - self.n_unknown[j],
    //    )
    //}
}

fn count_arrangements(
    row: &str,
    damaged_springs: DamagedSprings,
    spring_sums: &SpringSumsSlice,
) -> usize {
    //println!("row: {}", row);
    //println!("     {}", (0..row.len()).map(|i| i % 10).join(""));
    //println!(
    //    "len: {} start: {} end: {} spring_len: {}",
    //    row.len(),
    //    spring_sums.start,
    //    spring_sums.end,
    //    spring_sums.len()
    //);

    // The SprintSumsSlice should have length 1+ the row length
    assert_eq!(row.len() + 1, spring_sums.end - spring_sums.start);

    if damaged_springs.is_empty() {
        // TODO: Counting . and ?
        //let old_way = row.chars().all(|c| c != '#');
        let new_way = spring_sums.n_not_damaged() == row.len();
        //assert_eq!(old_way, new_way);
        //let status = match old_way == new_way {
        //    true => "It worked",
        //    false => "PROBLEM",
        //};
        //println!(
        //    "old_way: {} new_way: {} status: {}",
        //    old_way, new_way, status
        //);
        //return row.chars().all(|c| c != '#') as usize;
        return new_way as usize;
    } else {
        //let old_way = row.chars().filter(|&c| c != '.').count();
        let new_way = spring_sums.n_not_operational();
        //if old_way != new_way {
        //    println!("row: {}", row);
        //    println!("     {}", (0..row.len()).map(|i| i % 10).join(""));
        //    println!("start: {} end: {}", spring_sums.start, spring_sums.end);
        //    println!("old_way: {}", old_way);
        //    println!("new_way: {}", new_way);
        //    println!(
        //        "n_damaged: {:?} ({})",
        //        &spring_sums.owned_vecs.n_damaged[spring_sums.start..spring_sums.end],
        //        spring_sums.len()
        //    );
        //    println!(
        //        "n_unknown: {:?} ({})",
        //        &spring_sums.owned_vecs.n_unknown[spring_sums.start..spring_sums.end],
        //        spring_sums.len()
        //    );
        //}
        //assert_eq!(old_way, new_way);
        //if old_way < damaged_springs.iter().sum() {
        //    // TODO: Counting # and ?
        //    return 0;
        //}
        if new_way < damaged_springs.iter().sum() {
            // TODO: Counting # and ?
            return 0;
        }
    }

    // Split the damaged_springs into three parts damaged_springs[..,split,..]
    let split = damaged_springs.len() / 2;
    let left_damaged_springs = &damaged_springs[..split];
    let right_damaged_springs = &damaged_springs[(split + 1)..];

    // The split occurs implicitly at a string of the form ".#####."
    // where the "#"s match the length of split_len
    // and each terminal "." depends on having a nonempty left or right split,
    // respectively. These "."s force separation of the damaged springs.
    let split_spring = damaged_springs[split];
    let left_empty = left_damaged_springs.is_empty();
    let right_empty = right_damaged_springs.is_empty();
    //let split_str_len = split_spring + (!left_empty as usize) + (!right_empty as usize);

    //// The pot_damaged vector is an acceleration structure
    //// that prevents us from scanning the entire row of damaged springs
    //// TODO: Counting .
    //let pot_damaged: Vec<usize> = iter::once(0)
    //    .chain(row.chars().rev().scan(0, |acc, c| {
    //        *acc += (c != '.').then_some(1).unwrap_or(0);
    //        Some(*acc)
    //    }))
    //    .collect();
    //let pot_damaged: Vec<usize> = pot_damaged.into_iter().rev().collect();

    // Scan the row for possible splits
    let mut total_arrangements: usize = 0;
    for i in 0.. {
        let i1 = i;
        let i2 = if left_empty { i1 } else { i1 + 1 };
        let i3 = i2 + split_spring;
        let i4 = if right_empty { i3 } else { i3 + 1 };

        #[allow(unused_variables)]
        let i5 = row.len();
        if i4 > i5 {
            break;
        }

        // Ensure the right split has sufficiently many potential damaged springs.
        // Since right_pot_damaged is decreasing, we can break early.
        //assert_eq!(i4, i + split_str_len);
        //let right_slice = spring_sums.slice_right(i + split_str_len);
        let right_slice = spring_sums.slice(i4, i5 + 1);
        //assert_eq!(
        //    pot_damaged[i + split_str_len],
        //    right_slice.n_not_operational()
        //);
        if right_slice.n_not_operational() < right_damaged_springs.iter().sum() {
            break;
        }

        // TODO: Create a guard on the left slice
        let left_slice = spring_sums.slice_left(i + 1);

        // Ensure the center split has suffciently many potential damaged springs
        //let center_slice = spring_sums.slice_center(i, i + split_str_len + 1);
        let center_slice = spring_sums.slice(i1, i4 + 1);
        //assert_eq!(
        //    pot_damaged[i] - pot_damaged[i + split_str_len],
        //    center_slice.n_not_operational()
        //);

        if center_slice.n_not_operational() < split_spring {
            continue;
        }

        // Ensure that the center split is exctly valid
        //let mut center_row = row.chars().skip(i);

        #[allow(unused_variables)]
        //let old_a = !left_empty && center_row.next() == Some('#');
        let new_a = !left_empty && row.get(i1..i2) == Some("#");
        //assert_eq!(old_a, new_a);
        //let old_b = center_row.by_ref().take(split_spring).any(|c| c == '.');
        // TODO: Implement new_b.. this is complicated!
        //let new_b = center_slice().n_not
        let new_b = spring_sums.slice(i2, i3 + 1).n_not_operational() != (i3 - i2);
        let new_c = !right_empty && row.get(i3..i4) == Some("#");
        //assert_eq!(old_b, new_b);
        //let new_c =
        //    !right_empty && row.get((i + split_str_len - 1)..(i + split_str_len)) == Some("#");
        //assert_eq!(old_c, new_c);
        let old_way = new_a || new_b || new_c;
        if old_way {
            continue;
        }

        // Count the number of arrangements on the left
        let left_row = &row[..i];
        //println!(
        //    "i: {} left_row: {} left_slice: ({}, {})",
        //    i,
        //    left_row.len()old_old_old_ccc,
        //    left_slice.start,
        //    left_slice.end
        //);
        let left_arrangements =
            count_arrangements(left_row, left_damaged_springs.to_vec(), &left_slice);

        // Early return if no arrangements are possible on the left
        if left_arrangements == 0 {
            continue;
        }

        // Count the number of arrangements on the right
        let right_row = &row[i4..];
        //println!(
        //    "i: {} right_row: {} right_slice: ({}, {})",
        //    i,
        //    right_row.len(),
        //    right_slice.start,
        //    right_slice.end
        //);
        let right_arrangements =
            count_arrangements(right_row, right_damaged_springs.to_vec(), &right_slice);

        // This is te key to the combinatoral speedup
        total_arrangements += left_arrangements * right_arrangements
    }
    total_arrangements
}

fn parse_input(input: &'static str) -> Vec<(Row, DamagedSprings)> {
    input
        .lines()
        .map(|line| {
            let (row, damaged_springs) = line.split_once(" ").unwrap();
            (
                row.to_string(),
                damaged_springs.split(",").flat_map(str::parse).collect(),
            )
        })
        .collect()
}

fn increase_puzzle_size(puzzle: Vec<(Row, DamagedSprings)>) -> Vec<(Row, DamagedSprings)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let row = iter::repeat(row).take(5).collect_vec().join("?");
            let damaged_springs = iter::repeat(damaged_springs).take(5).flatten().collect();
            (row, damaged_springs)
        })
        .collect()
}
