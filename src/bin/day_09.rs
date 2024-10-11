use num::CheckedSub;

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_09.txt");
    //let input = include_str!("../../puzzle_inputs/day_09_test.txt");
    println!("input len: {}", input.len());
    println!("input:\n{}", input);

    let mut answer = 0;
    for line in input.lines() {
        let mut line: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        println!("line: {:?}", line);
        let mut stack = Vec::new();
        while !line.iter().all(|&x| x == 0) {
            let next_line = line.windows(2).map(|x| x[1] - x[0]).collect();
            println!("next_line: {:?}", next_line);
            stack.push(line);
            line = next_line;
        }
        println!("last_line: {:?}\n", stack.last().unwrap());
        answer += stack
            .iter()
            .rev()
            .fold(0, |delta, line| line.last().unwrap() + delta);
    }
    println!("answer: {}", answer);

    //// Solve 9a
    //let sol_9a: usize = 12;
    //let correct_sol_9a: usize = 32;
    //println!("* 9a *");
    //println!("My solution: {sol_9a}");
    //println!("Correct solution: {correct_sol_9a}");
    //println!("Equal: {}\n", sol_9a == correct_sol_9a);
    //
    //// Solve 9b
    //let sol_9b: usize = 56;
    //let correct_sol_9b: usize = 79;
    //println!("* 9b *");
    //println!("My solution: {sol_9b}");
    //println!("Correct solution: {correct_sol_9b}");
    //println!("Equal: {}\n", sol_9b == correct_sol_9b);
}
