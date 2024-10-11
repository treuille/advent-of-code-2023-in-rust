fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_09.txt");

    // Solve both puzzles at once
    let (sol_9b, sol_9a) = input
        .lines()
        .map(|line| {
            let line: Vec<i32> = line
                .split_whitespace()
                .map(|line| line.parse().unwrap())
                .collect();
            solve(&line)
        })
        .fold((0, 0), |(l_sum, r_sum), (l, r)| (l_sum + l, r_sum + r));

    // Solve 9a
    let correct_sol_9a = 1708206096;
    println!("* 9a *");
    println!("My solution: {sol_9a}");
    println!("Correct solution: {correct_sol_9a}");
    println!("Equal: {}\n", sol_9a == correct_sol_9a);

    // Solve 9b
    let correct_sol_9b = 1050;
    println!("* 9b *");
    println!("My solution: {sol_9b}");
    println!("Correct solution: {correct_sol_9b}");
    println!("Equal: {}\n", sol_9b == correct_sol_9b);
}

fn solve(line: &[i32]) -> (i32, i32) {
    match line.iter().all(|&x| x == 0) {
        true => (0, 0),
        false => {
            let next_line: Vec<i32> = line.windows(2).map(|x| x[1] - x[0]).collect();
            let (first, last) = solve(&next_line);
            (line.first().unwrap() - first, line.last().unwrap() + last)
        }
    }
}
