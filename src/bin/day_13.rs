use advent_of_code_2023_in_rust::grid::parse_char_grid;
use ndarray::{Array2, Axis};

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_13.txt");
    let input: Vec<Array2<bool>> = input
        .split("\n\n")
        .map(|s| parse_char_grid(s, |c| c == '#'))
        .collect();

    // Solve 13a
    let sol_13a = solve(&input, ());
    let correct_sol_13a: usize = 27300;
    println!("* 13a *");
    println!("My solution: {sol_13a}");
    println!("Correct solution: {correct_sol_13a}");
    println!("Equal: {}\n", sol_13a == correct_sol_13a);

    //// Solve 13b
    //let sol_13b: usize = solve_13b(&input);
    //let correct_sol_13b: usize = 29276;
    //println!("* 13b *");
    //println!("My solution: {sol_13b}");
    //println!("Correct solution: {correct_sol_13b}");
    //println!("Equal: {}\n", sol_13b == correct_sol_13b);
}

#[allow(unused_variables)]
fn solve(
    input: &[Array2<bool>],
    valid_reflection: impl FnMut(impl Iterator<Item = usize>) -> bool,
) -> usize {
    input
        .iter()
        .map(|grid| {
            let shape = grid.shape();
            let axis_coefs: [usize; 2] = [100, 1];
            for axis in 0..2 {
                for idx in 0..(shape[axis] - 1) {
                    // Box the iterator so it can meet the Sized requirement for `.all()`
                    let mut disagreements = (0..)
                        .take_while(|&offset| idx >= offset)
                        .take_while(|&offset| idx + offset + 1 < shape[axis])
                        .map(move |offset| (idx - offset, idx + offset + 1))
                        .take_while(move |&(idx_a, idx_b)| idx_a > 0 || (idx_b + 1) < shape[axis])
                        .map(move |(idx_a, idx_b)| {
                            let vec_a = grid.index_axis(Axis(axis), idx_a);
                            let vec_b = grid.index_axis(Axis(axis), idx_b);
                            vec_a.iter().zip(vec_b).filter(|(a, b)| a != b).count()
                        });

                    // Pass `&mut *disagreements` to valid_reflection to match the expected type
                    if valid_reflection(&mut disagreements) {
                        return (idx + 1) * axis_coefs[axis];
                    }
                }
            }
            panic!("No reflections found");
        })
        .sum()
}

fn valid_reflection_13a(mut disagreements: impl Iterator<Item = usize>) -> bool {
    disagreements.all(|disagreement| disagreement == 0)
}

// To move forward, we want to create a function that that takes
// an iterator producing disgreeements (usizes) and decides whether
// this is a valid reflection
#[allow(dead_code, unused_variables)]
fn valid_reflection_13b(disagreements: impl Iterator<Item = usize>) -> bool {
    unimplemented!()
    //let mut found_smudge: bool = false;
    //for disagreement in disagreements {
    //    match (disagreement, found_smudge) {
    //        (0, _) => (),
    //        (1, false) => found_smudge = true,
    //        (1, true) => return false, // We might be able to skip this
    //        (_, _) => return false,
    //    }
    //}
    //assert!(found_smudge);
    //true
}

//fn solve_13b<UsizeIter>(input: &[Array2<bool>], valid_reflection: fn(UsizeIter) -> bool) -> usize
//where
//    UsizeIter: Iterator<Item = usize>,
//{
//    input
//        .iter()
//        .map(|grid| {
//            let shape = grid.shape();
//            let axis_coefs: [usize; 2] = [100, 1];
//            let mut found_smudge: bool = false;
//            for axis in 0..2 {
//                for idx in 0..(shape[axis] - 1) {
//                    for offset in 0.. {
//                        let idx_a = if idx < offset {
//                            break;
//                        } else {
//                            idx - offset
//                        };
//                        let vec_a = grid.index_axis(Axis(axis), idx_a);
//
//                        let idx_b = if idx + offset + 1 >= shape[axis] {
//                            break;
//                        } else {
//                            idx + offset + 1
//                        };
//                        let vec_b = grid.index_axis(Axis(axis), idx_b);
//
//                        let disagreement = vec_a.iter().zip(vec_b).filter(|(a, b)| a != b).count();
//                        let found_edge = idx_a == 0 || (idx_b + 1) == shape[axis];
//                        match (disagreement, found_smudge, found_edge) {
//                            (0, true, true) | (1, _, true) => {
//                                return (idx + 1) * axis_coefs[axis];
//                            }
//                            (1, false, _) => {
//                                found_smudge = true;
//                            }
//                            (0, _, _) => (),
//                            (_, _, _) => {
//                                found_smudge = false;
//                                break;
//                            }
//                        }
//                    }
//                }
//            }
//            panic!("No reflections found");
//        })
//        .sum()
//    //let i5 = row.len();
//}
