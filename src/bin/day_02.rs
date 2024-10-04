use std::{collections::HashMap, fmt::write};

/// A `Reveal` is a sequence of (num_cubes, cube_color) tuples
type Reveal = Vec<(usize, &'static str)>;

/// A `Game` is a sequence of `Reveal`s
type Game = Vec<Reveal>;

fn main() {
    // This is the input string
    let input = include_str!("../../puzzle_inputs/day_02.txt");
    let input_games: Vec<Game> = parse_input(input);

    // Solve 2a
    let sol_2a = solve_2a(&input_games);
    let correct_sol_2a: usize = 2268;
    println!("* 2A *");
    println!("My solution: {sol_2a}");
    println!("Correct solution: {correct_sol_2a}");
    println!("Equal: {}\n", sol_2a == correct_sol_2a);

    // Solve 2b
    let sol_2b = solve_2b(&input_games);
    let correct_sol_2b: usize = 63542;
    println!("* 2B *");
    println!("My solution: {sol_2b}");
    println!("Correct solution: {correct_sol_2b}");
    println!("Equal: {}\n", sol_2b == correct_sol_2b);
}

fn solve_2a(games: &[Game]) -> usize {
    let max_cubes: HashMap<&str, usize> = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    games
        .iter()
        .zip(1..)
        .filter_map(|(game, game_num)| {
            for reveal in game {
                for (num_cubes, cube_color) in reveal {
                    if *num_cubes > max_cubes[cube_color] {
                        return None;
                    }
                }
            }
            Some(game_num)
        })
        .sum::<usize>()
}

fn solve_2b(games: &[Game]) -> usize {
    games
        .iter()
        .map(|game| {
            let mut minima: HashMap<&str, usize> = ["red", "green", "blue"]
                .into_iter()
                .map(|cube_color| (cube_color, 0))
                .collect();
            for reveal in game {
                for (num_cubes, cube_color) in reveal {
                    let min_num_cubes = minima.get_mut(cube_color).unwrap();
                    *min_num_cubes = (*min_num_cubes).max(*num_cubes);
                }
            }
            minima["red"] * minima["green"] * minima["blue"]
        })
        .sum::<usize>()
}

/// Parse the inut string into a sequence of `Game`s
fn parse_input(input: &'static str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (_game_str, subsets_str) = line.split_once(": ").unwrap();
            subsets_str
                .split("; ")
                .map(|subset| {
                    subset
                        .split(", ")
                        .map(|reveal| {
                            let (num_cubes, cube_color) = reveal.split_once(" ").unwrap();
                            (num_cubes.parse::<usize>().unwrap(), cube_color)
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}
