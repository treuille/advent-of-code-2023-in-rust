use std::collections::HashMap;

//use std::fmt::write;

fn main() {
    // This is the input string
    let input = include_str!("../../puzzle_inputs/day_02.txt");

    println!("input length: {}", input.len());
    //println!("input:\n{}", input);

    let max_cubes: HashMap<&str, usize> = [("red", 12), ("green", 13), ("blue", 14)]
        .into_iter()
        .collect();

    #[allow(clippy::unnecessary_filter_map)]
    let answer = input
        .lines()
        .zip(1..)
        .filter_map(|(line, game_num)| {
            //println!("line: {line}");
            let (_game_str, subsets_str) = line.split_once(": ").unwrap();
            //println!("game: \"{game_str}\"");
            //println!("game_num: \"{game_num}\"");
            //
            //println!("subsets: \"{subsets_str}\"");
            for subset in subsets_str.split(", ") {
                //println!("subset: {subset}");
                for reveal in subset.split("; ") {
                    //println!("reveal: \"{reveal}\"");
                    let (num_cubes, cube_color) = reveal.split_once(" ").unwrap();
                    //println!("num_cubes: \"{num_cubes}\"");
                    let num_cubes = num_cubes.parse::<usize>().unwrap();
                    //println!("num_cubes: {num_cubes}");
                    //println!("cube_color: \"{cube_color}\"");
                    if num_cubes > max_cubes[cube_color] {
                        return None;
                    }
                }
            }
            Some(game_num)
        })
        .sum::<usize>();

    println!("answer: {}", answer);
    //// Dummy solution to 2a
    //let sol_2a = 123;
    let _correct_sol_2a: usize = 2268;
    //println!("* 2A *");
    //println!("My solution: {sol_2a}");
    //println!("Correct solution: {correct_sol_2a}");
    //println!("Equal: {}\n", sol_2a == correct_sol_2a);
    //
    //// Dummy solution to 2a
    //let sol_2b = 78;
    //let correct_sol_2b: usize = 92;
    //println!("* 2B *");
    //println!("My solution: {sol_2b}");
    //println!("Correct solution: {correct_sol_2b}");
    //println!("Equal: {}\n", sol_2b == correct_sol_2b);
}
