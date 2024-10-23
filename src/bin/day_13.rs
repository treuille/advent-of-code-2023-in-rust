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
    let sol_13a = solve_13a(&input);
    let correct_sol_13a: usize = 27300;
    println!("* 13a *");
    println!("My solution: {sol_13a}");
    println!("Correct solution: {correct_sol_13a}");
    println!("Equal: {}\n", sol_13a == correct_sol_13a);

    // Solve 13b
    let sol_13b: usize = solve_13b(&input);
    let correct_sol_13b: usize = 29276;
    println!("* 13b *");
    println!("My solution: {sol_13b}");
    println!("Correct solution: {correct_sol_13b}");
    println!("Equal: {}\n", sol_13b == correct_sol_13b);
}

fn solve_13a(input: &[Array2<bool>]) -> usize {
    input
        .iter()
        .map(|grid| {
            let shape = grid.shape();
            let axis_coefs: [usize; 2] = [100, 1];
            for axis in 0..2 {
                for idx in 0..(shape[axis] - 1) {
                    for offset in 0.. {
                        let idx_a = if idx < offset {
                            break;
                        } else {
                            idx - offset
                        };
                        let vec_a = grid.index_axis(Axis(axis), idx_a);

                        let idx_b = if idx + offset + 1 >= shape[axis] {
                            break;
                        } else {
                            idx + offset + 1
                        };
                        let vec_b = grid.index_axis(Axis(axis), idx_b);

                        if vec_a != vec_b {
                            break;
                        } else if idx_a == 0 || (idx_b + 1) == shape[axis] {
                            return (idx + 1) * axis_coefs[axis];
                        }
                    }
                }
            }
            panic!("No reflections found");
        })
        .sum()
}

fn solve_13b(input: &[Array2<bool>]) -> usize {
    input
        .iter()
        .map(|grid| {
            let shape = grid.shape();
            let axis_coefs: [usize; 2] = [100, 1];
            //let mut axis_counts: [usize; 2] = [0, 0];
            let mut found_smudge: bool = false;
            for axis in 0..2 {
                for idx in 0..(shape[axis] - 1) {
                    for offset in 0.. {
                        let idx_a = if idx < offset {
                            break;
                        } else {
                            idx - offset
                        };
                        let vec_a = grid.index_axis(Axis(axis), idx_a);

                        let idx_b = if idx + offset + 1 >= shape[axis] {
                            break;
                        } else {
                            idx + offset + 1
                        };
                        let vec_b = grid.index_axis(Axis(axis), idx_b);

                        let disagreement = vec_a.iter().zip(vec_b).filter(|(a, b)| a != b).count();
                        let found_edge = idx_a == 0 || (idx_b + 1) == shape[axis];
                        match (disagreement, found_smudge, found_edge) {
                            (0, true, true) | (1, _, true) => {
                                return (idx + 1) * axis_coefs[axis];
                            }
                            (1, false, _) => {
                                found_smudge = true;
                            }
                            (0, _, _) => (),
                            (_, _, _) => {
                                found_smudge = false;
                                break;
                            }
                        }
                    }
                }
            }
            panic!("No reflections found");
            //axis_counts[0] * 100 + axis_counts[1]
        })
        .sum()
}
