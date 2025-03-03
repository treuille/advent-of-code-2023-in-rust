use itertools::Itertools;
use std::iter;

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
/// The key properties of this solution are:
///
/// 1. Reflects the combinatorial structure of the problem through a
///    divide-and-conquer approach.
/// 2. Uses a runing sum acceleration data structure to quickly cut off
///    some branches of the search if the split are impossible.
/// 3. No allocations during the search.
/// 4. Solves the problem in <1.5s (without memization) due to (1)-(3).
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

fn solve(puzzle: &[(String, Vec<usize>)]) -> usize {
    puzzle
        .iter()
        .map(|(row, damaged_springs)| {
            let spring_sums = SpringSums::new(row);
            count_arrangements(row, damaged_springs, &SpringSumsSlice::new(&spring_sums))
        })
        .sum()
}

/// Acceleration data structure
struct SpringSums {
    not_operational: Vec<usize>, // Running sum of != '.' (from the right)
    not_damaged: Vec<usize>,     // Running sum of != '#' (from the right)
}

/// This is a slice into a `SpringSums`
struct SpringSumsSlice<'a> {
    sums: &'a SpringSums,
    start: usize,
    end: usize,
}

impl SpringSums {
    fn new(row: &str) -> Self {
        let mut sums = SpringSums {
            not_operational: vec![0; row.len() + 1],
            not_damaged: vec![0; row.len() + 1],
        };

        for (i, c) in row.chars().rev().enumerate() {
            let i = row.len() - i;
            sums.not_operational[i - 1] = sums.not_operational[i] + (c != '.') as usize;
            sums.not_damaged[i - 1] = sums.not_damaged[i] + (c != '#') as usize;
        }

        sums
    }
}

impl<'a> SpringSumsSlice<'a> {
    fn new(owned_vecs: &'a SpringSums) -> Self {
        let len = owned_vecs.not_operational.len();
        assert_eq!(owned_vecs.not_damaged.len(), len);

        SpringSumsSlice {
            sums: owned_vecs,
            start: 0,
            end: len,
        }
    }

    fn slice(&self, start: usize, end: usize) -> Self {
        assert!(start <= end);
        assert!(end <= self.sums.not_operational.len());

        SpringSumsSlice {
            sums: self.sums,
            start: self.start + start,
            end: self.start + end,
        }
    }

    fn n_not_damaged(&self) -> usize {
        self.sums.not_damaged[self.start] - self.sums.not_damaged[self.end - 1]
    }

    fn n_not_operational(&self) -> usize {
        self.sums.not_operational[self.start] - self.sums.not_operational[self.end - 1]
    }
}

fn count_arrangements(
    row: &str,
    damaged_springs: &[usize],
    spring_sums: &SpringSumsSlice,
) -> usize {
    // The SprintSumsSlice should have length 1+ the row length
    assert_eq!(row.len() + 1, spring_sums.end - spring_sums.start);

    if damaged_springs.is_empty() {
        return (spring_sums.n_not_damaged() == row.len()) as usize;
    } else if spring_sums.n_not_operational() < damaged_springs.iter().sum() {
        return 0;
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

    // Scan the row for possible splits
    // [..., '.', '#', '#', '#', '#', '#', '.', ...]
    //        i1   i2                       i3  i4
    let mut total_arrangements: usize = 0;
    for i1 in 0.. {
        let i2 = if left_empty { i1 } else { i1 + 1 };
        let i3 = i2 + split_spring;
        let i4 = if right_empty { i3 } else { i3 + 1 };

        // There are no more possible splits
        if i4 > row.len() {
            break;
        }

        // Ensure the right split has sufficiently many potential damaged springs.
        // Since right_pot_damaged is decreasing, we can break early.
        let right_slice = spring_sums.slice(i4, row.len() + 1);
        if right_slice.n_not_operational() < right_damaged_springs.iter().sum() {
            break;
        }

        // Ensure the center split has suffciently many potential damaged springs
        if spring_sums.slice(i1, i4 + 1).n_not_operational() < split_spring {
            continue;
        }

        // Ensure the left split has sufficiently many potential damaged springs.
        let left_slice = spring_sums.slice(0, i1 + 1);
        if left_slice.n_not_operational() < left_damaged_springs.iter().sum() {
            continue;
        }

        // Make sure that the center split is excatly valid
        if !left_empty && row.get(i1..i2) == Some("#")
            || spring_sums.slice(i2, i3 + 1).n_not_operational() != (i3 - i2)
            || !right_empty && row.get(i3..i4) == Some("#")
        {
            continue;
        }

        // Count the number of arrangements on the left
        let left_row = &row[..i1];
        let left_arrangements = count_arrangements(left_row, left_damaged_springs, &left_slice);

        // Early return if no arrangements are possible on the left
        if left_arrangements == 0 {
            continue;
        }

        // Count the number of arrangements on the right
        let right_row = &row[i4..];
        let right_arrangements = count_arrangements(right_row, right_damaged_springs, &right_slice);

        // This is te key to the combinatoral speedup
        total_arrangements += left_arrangements * right_arrangements
    }
    total_arrangements
}

fn parse_input(input: &'static str) -> Vec<(String, Vec<usize>)> {
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

fn increase_puzzle_size(puzzle: Vec<(String, Vec<usize>)>) -> Vec<(String, Vec<usize>)> {
    puzzle
        .into_iter()
        .map(|(row, damaged_springs)| {
            let row = iter::repeat(row).take(5).collect_vec().join("?");
            let damaged_springs = iter::repeat(damaged_springs).take(5).flatten().collect();
            (row, damaged_springs)
        })
        .collect()
}
