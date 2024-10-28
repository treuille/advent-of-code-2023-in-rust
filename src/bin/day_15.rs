use std::array;

fn main() {
    //// Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_15.txt").trim();

    //let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    // Part A
    let answer: u64 = input.split(",").map(hash).sum();
    println!("answer: {}", answer);

    // Part B
    let boxes: [Vec<(&str, u64)>; 256] = array::from_fn(|_| Vec::new());
    dbg!(boxes);

    //// Get the ascii code for c
    //let c = 'H';
    //println!("c: {}", c);
    //println!("ascii(c): {}", c as u64);
    //println!("hash: {}", hash("HASH"));

    //// Solve 15a
    //let sol_15a: usize = 12;
    //let correct_sol_15a: usize = 32;
    //println!("* 15a *");
    //println!("My solution: {sol_15a}");
    //println!("Correct solution: {correct_sol_15a}");
    //println!("Equal: {}\n", sol_15a == correct_sol_15a);
    //
    //// Solve 15b
    //let sol_15b: usize = 56;
    //let correct_sol_15b: usize = 79;
    //println!("* 15b *");
    //println!("My solution: {sol_15b}");
    //println!("Correct solution: {correct_sol_15b}");
    //println!("Equal: {}\n", sol_15b == correct_sol_15b);
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0u64, |current_value, c| {
        (current_value + (c as u64)) * 17 % 256
    })
}
