use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::{Array2, Axis};

/// An interator that produces a sequence of "disagreements,"
/// that is, the number of elements that differ over a sequence
/// of pairs of vectors.
type DisgreementIter<'a> = &'a mut dyn Iterator<Item = usize>;

/// The basic idea is to iterate over all possibele horizontal and vertical
/// refelctions, for each one creating a `DisgreementIter` that produces the
/// sequence of disagreements along that reflection. The `valid_reflection`
/// methods determine whether the sequence of disagreements represnet a valid reflection
/// specific to the `13a` and `13b` puzzle logic, respectively.
fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    let input: Vec<Array2<bool>> = input
        .split("\n\n")
        .map(|s| parse_char_grid(s, |c| c == '#'))
        .collect();

    // Solve 13a
    let sol_13a = solve(&input, valid_reflection_13a);
    let correct_sol_13a: usize = 27300;
    println!("* 13a *");
    println!("My solution: {sol_13a}");
    println!("Correct solution: {correct_sol_13a}");
    println!("Equal: {}\n", sol_13a == correct_sol_13a);

    // Solve 13b
    let sol_13b = solve(&input, valid_reflection_13b);
    let correct_sol_13b: usize = 29276;
    println!("* 13b *");
    println!("My solution: {sol_13b}");
    println!("Correct solution: {correct_sol_13b}");
    println!("Equal: {}\n", sol_13b == correct_sol_13b);
}

#[allow(unused_variables)]
fn solve(input: &[Array2<bool>], valid_reflection: fn(DisgreementIter) -> bool) -> usize {
    input
        .iter()
        .map(|grid| {
            let shape = grid.shape();
            let axis_coefs: [usize; 2] = [100, 1];
            for axis in 0..2 {
                for idx in 0..(shape[axis] - 1) {
                    // Box the iterator so it can meet the Sized requirement for `.all()`
                    let mut disagreements = (0..)
                        .take_while(|&offset| idx >= offset && idx + offset + 1 < shape[axis])
                        .map(|offset| (idx - offset, idx + offset + 1))
                        .map(|(idx_a, idx_b)| {
                            let vec_a = grid.index_axis(Axis(axis), idx_a);
                            let vec_b = grid.index_axis(Axis(axis), idx_b);
                            vec_a.iter().zip(vec_b).filter(|(a, b)| a != b).count()
                        });

                    if valid_reflection(&mut disagreements) {
                        return (idx + 1) * axis_coefs[axis];
                    }
                }
            }
            panic!("No reflections found");
        })
        .sum()
}

fn valid_reflection_13a(disagreements: DisgreementIter) -> bool {
    for disagreement in disagreements {
        if disagreement != 0 {
            return false;
        }
    }
    true
}

fn valid_reflection_13b(disagreements: &mut dyn Iterator<Item = usize>) -> bool {
    let mut found_smudge: bool = false;
    for disagreement in disagreements {
        match (disagreement, found_smudge) {
            (0, _) => (),
            (1, false) => found_smudge = true,
            (_, _) => return false,
        }
    }
    found_smudge
}
