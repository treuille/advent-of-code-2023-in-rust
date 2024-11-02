use advent_of_code_2023_in_rust::parse_regex;
use itertools::Itertools;
use regex::Regex;
use std::iter;

fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_18_test.txt");
    let input = include_str!("../../puzzle_inputs/day_18.txt");
    let (instructions_a, instructions_b) = parse_input(input);

    // Solve 18a
    let sol_18a: i64 = solve(&instructions_a);
    let correct_sol_18a: i64 = 26857;
    println!("* 18a *");
    println!("My solution: {sol_18a}");
    println!("Correct solution: {correct_sol_18a}");
    println!("Equal: {}\n", sol_18a == correct_sol_18a);

    // Solve 18b
    let sol_18b: i64 = solve(&instructions_b);
    let correct_sol_18b: i64 = 129373230496292;
    println!("* 18b *");
    println!("My solution: {sol_18b}");
    println!("Correct solution: {correct_sol_18b}");
    println!("Equal: {}\n", sol_18b == correct_sol_18b);
}

/// A sequence of directions (R, L, D, U) and steps to mvove in that direction
type Instructions = Vec<(char, i64)>;

/// Produces both the instructions for parts A and B at once
fn parse_input(input: &str) -> (Instructions, Instructions) {
    let pattern = r"(R|L|D|U) (\d+) \(#([0-9,a-f]{5})([0-3])\)";
    let re = Regex::new(pattern).unwrap();
    let line_iter = parse_regex::parse_lines(re, input);

    let mut instructions_a: Instructions = Vec::new();
    let mut instructions_b: Instructions = Vec::new();

    // Compute the vertices, centered in each grid cell
    for line in line_iter {
        let (dir_a, steps_a, steps_b, dir_b): (char, i64, &str, u8) = line;
        assert!(steps_a > 0, "Steps must be positive: {}", steps_a);
        instructions_a.push((dir_a, steps_a));

        let steps_b = i64::from_str_radix(steps_b, 16).unwrap();
        let dir_b = match dir_b {
            0 => 'R',
            1 => 'D',
            2 => 'L',
            3 => 'U',
            _ => panic!("Invalid direction: {dir_b}"),
        };
        instructions_b.push((dir_b, steps_b));
    }
    (instructions_a, instructions_b)
}

fn solve(instructions: &Instructions) -> i64 {
    // Calculate the unit vector from `x1` to `x2`, or zero if they coincide.
    let safe_norm = |x1: i64, x2: i64| match x1.cmp(&x2) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    };

    // Negate a vector
    let neg = |v: (i64, i64)| (-v.0, -v.1);

    let n_vertices = instructions.len();
    let intial_vertex = (0, 0);
    iter::once(intial_vertex)
        .chain(
            // Calculate a succession of cell centers by moving the initial vertex
            // around, logo-style, according to the instructions.
            instructions
                .iter()
                .scan(intial_vertex, |pos, &(dir, steps)| {
                    match dir {
                        'R' => pos.0 += steps,
                        'L' => pos.0 -= steps,
                        'D' => pos.1 += steps,
                        'U' => pos.1 -= steps,
                        _ => panic!("Invalid direction: {dir}"),
                    };
                    Some(*pos)
                }),
        )
        .skip(1)
        .cycle()
        .tuple_windows()
        .map(|(c1, c2, c3)| {
            // Now move each vertex `c2` (a cell center) to
            // a cell corner. We do this by looking at the preceeding
            // and following vertices, `c1` and `c3`, respectively, and
            // creating a coordinate system `dx` and `dy`. The handedness
            // of the coordinate system determines the corner.
            let dx = (safe_norm(c2.0, c3.0), safe_norm(c2.1, c3.1));
            let dy = (safe_norm(c2.0, c1.0), safe_norm(c2.1, c1.1));
            let rot_dx = (-dx.1, dx.0);
            let d_corner = {
                if rot_dx == dy {
                    (-dx.0 - dy.0, -dx.1 - dy.1)
                } else if rot_dx == neg(dy) {
                    (dx.0 + dy.0, dx.1 + dy.1)
                } else {
                    panic!("Unexpected corner: dx:{:?} dy:{:?}", dx, dy);
                }
            };
            #[allow(clippy::let_and_return)]
            let corner_vertex = (
                (2 * c2.0 + 1 + d_corner.0) / 2,
                (2 * c2.1 + 1 + d_corner.1) / 2,
            );
            corner_vertex
        })
        .tuple_windows()
        .take(n_vertices)
        .map(|((x1, y1), (x2, y2))| {
            // Now caluclate the area using Gauss' formula
            x1 * y2 - x2 * y1
        })
        .sum::<i64>()
        .abs()
        / 2
}
