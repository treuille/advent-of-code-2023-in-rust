use advent_of_code_2023_in_rust::parse_regex;
use regex::Regex;
use std::collections::HashSet;

fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_18_test.txt");
    let input = include_str!("../../puzzle_inputs/day_18.txt");

    //println!("input len: {}", input.len());
    //println!("input:\n{}", input);

    let pattern = r"(R|L|D|U) (\d+) \(#([0-9,a-f]{6})\)";
    let re = Regex::new(pattern).unwrap();

    let mut dug = HashSet::new();
    let mut pos = (0, 0);
    dug.insert(pos);
    for line in parse_regex::parse_lines(re, input) {
        let (dir, steps, _color): (char, i32, &str) = line;
        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => panic!("Invalid direction: {}", dir),
        };
        //println!("dir: {}, steps: {}, color: {}", dir, steps, color);
        for _ in 0..steps {
            pos = (pos.0 + dx, pos.1 + dy);
            dug.insert(pos);
        }
    }

    // Find the bounds
    let min_x = dug.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = dug.iter().map(|&(_, y)| y).min().unwrap();
    let max_x = dug.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = dug.iter().map(|&(_, y)| y).max().unwrap();

    // Print in out
    println!("Before raseterization");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if dug.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    println!();

    //// Raseterize the interior
    //for y in min_y..=max_y {
    //    let mut interior = false;
    //    let mut last_trench = false;
    //
    //    for x in min_x..=max_x {
    //        if dug.contains(&(x, y)) {
    //            if !last_trench {
    //                interior = !interior;
    //                last_trench = true;
    //            }
    //        } else {
    //            if interior {
    //                dug.insert((x, y));
    //            }
    //            last_trench = false;
    //        }
    //    }
    //}

    // Raseterize the interior
    let mut interior = HashSet::new();
    for y in min_y..=max_y {
        let mut in_interior = false;

        for x in min_x..=max_x {
            if dug.contains(&(x - 1, y)) && !dug.contains(&(x, y)) {
                in_interior = !in_interior;
            }
            if in_interior && !dug.contains(&(x, y)) {
                interior.insert((x, y));
            }
        }
    }
    dug.extend(interior);

    // Print in out
    println!("After raseterization");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = if dug.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    println!();
    println!("dug.len(): {}", dug.len());

    //// Solve 18a
    //let sol_18a: usize = 12;
    //let correct_sol_18a: usize = 32;
    //println!("* 18a *");
    //println!("My solution: {sol_18a}");
    //println!("Correct solution: {correct_sol_18a}");
    //println!("Equal: {}\n", sol_18a == correct_sol_18a);
    //
    //// Solve 18b
    //let sol_18b: usize = 56;
    //let correct_sol_18b: usize = 79;
    //println!("* 18b *");
    //println!("My solution: {sol_18b}");
    //println!("Correct solution: {correct_sol_18b}");
    //println!("Equal: {}\n", sol_18b == correct_sol_18b);
}
