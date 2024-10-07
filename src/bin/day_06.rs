fn main() {
    // Solve 6a
    let sol_6a: usize = solve(&[40, 82, 84, 92], &[233, 1011, 1110, 1487]);
    let correct_sol_6a: usize = 3316275;
    println!("* 6a *");
    println!("My solution: {sol_6a}");
    println!("Correct solution: {correct_sol_6a}");
    println!("Equal: {}\n", sol_6a == correct_sol_6a);

    // Solve 6b
    let sol_6b: usize = solve(&[40828492], &[233101111101487]);
    let correct_sol_6b: usize = 27102791;
    println!("* 6b *");
    println!("My solution: {sol_6b}");
    println!("Correct solution: {correct_sol_6b}");
    println!("Equal: {}\n", sol_6b == correct_sol_6b);
}

fn solve(times: &[usize], dists: &[usize]) -> usize {
    let epsilon: f64 = 1e-10;
    times
        .iter()
        .zip(dists.iter())
        .map(|(&time, &dist)| {
            let t_f64: f64 = time as f64;
            let delta_f64: f64 = ((time * time - 4 * dist) as f64).sqrt();
            let min_f64 = ((t_f64 - delta_f64) / 2.0 + epsilon).ceil().max(0.0);
            let max_f64 = ((t_f64 + delta_f64) / 2.0 - epsilon).floor().min(t_f64);
            let min_usize: usize = min_f64 as usize;
            let max_usize: usize = max_f64 as usize;
            max_usize - min_usize + 1 // the number of solutions
        })
        .product()
}
