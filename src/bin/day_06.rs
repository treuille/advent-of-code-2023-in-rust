fn main() {
    let test_input_times: [usize; 3] = [7, 15, 30];
    let test_input_dists: [usize; 3] = [9, 40, 200];

    let input_times: &[usize] = &test_input_times;
    let input_dists: &[usize] = &test_input_dists;

    println!("input_times: {:?}", input_times);
    println!("input_dists: {:?}", input_dists);

    // This is the math
    // s : seconds waited
    // t - s : time to race
    // s * (t - s) = s * t - s ^ 2 : distance raced
    // t - 2 * s = 0 : distance maximum
    // s = t / 2 : distance maximum
    // m : distance to beat
    // s * t - s ^ 2 - m = 0 : solutions to beat minimum distance
    //   s = (t - sqrt(t ^ 2 - 4 * m)) / 2
    //   A = -1
    //   B = t
    //   C = -m
    //   s = (-B - sqrt(B ^ 2 - 4 * A * C)) / 2 * A
    //   s = (-t - sqrt(t ^ 2 - 4 * m)) / -2
    //   s = (t +- sqrt(t ^ 2 - 4 * m)) / 2

    // Solve 1
    for (&t, &m) in input_times.iter().zip(input_dists.iter()) {
        let t_f64: f64 = t as f64;
        let delta_squared_f64: f64 = (t * t - 4 * m) as f64;
        assert!(delta_squared_f64 >= 0.0);
        let delta_f64: f64 = delta_squared_f64.sqrt();
        let min_f64 = ((t_f64 - delta_f64) / 2.0).ceil().max(0.0);
        let max_f64 = ((t_f64 + delta_f64) / 2.0).floor().min(t_f64);
        let min: usize = min_f64 as usize;
        let max: usize = max_f64 as usize;
        let solns = max - min + 1;
        println!(
            "t: {}, m: {}, min: {}, max: {}, solns: {}",
            t, m, min, max, solns,
        );
    }

    //// Solve 5a
    //let sol_5a: usize = 12;
    //let correct_sol_5a: usize = 32;
    //println!("* 5a *");
    //println!("My solution: {sol_5a}");
    //println!("Correct solution: {correct_sol_5a}");
    //println!("Equal: {}\n", sol_5a == correct_sol_5a);
    //
    //// Solve 5b
    //let sol_5b: usize = 56;
    //let correct_sol_5b: usize = 79;
    //println!("* 5b *");
    //println!("My solution: {sol_5b}");
    //println!("Correct solution: {correct_sol_5b}");
    //println!("Equal: {}\n", sol_5b == correct_sol_5b);
}
