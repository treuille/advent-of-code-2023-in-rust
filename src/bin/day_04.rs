fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_04.txt");
    println!("Input length: {}\n", input.len());
    let cards = parse_input(input);
    println!("cards: {:?}", cards[0]);

    //let (parts, symbols) = parse_input(input);
    //
    //// Solve 3a
    //let sol_3a: usize = solve_3a(&parts, &symbols);
    //let correct_sol_3a: usize = 546312;
    //println!("* 3a *");
    //println!("My solution: {sol_3a}");
    //println!("Correct solution: {correct_sol_3a}");
    //println!("Equal: {}\n", sol_3a == correct_sol_3a);
    //
    //// Solve 3b
    //let sol_3b: usize = solve_3b(&parts, &symbols);
    //let correct_sol_3b: usize = 87449461;
    //println!("* 3b *");
    //println!("My solution: {sol_3b}");
    //println!("Correct solution: {correct_sol_3b}");
    //println!("Equal: {}\n", sol_3b == correct_sol_3b);
}

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    winning_numbers: Vec<usize>,
    your_numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let card = line.split_once(": ").unwrap().1;
            let (winning_numbers, your_numbers) = card.split_once(" | ").unwrap();
            println!("card: \"{card}\"");
            //println!("winning_numbers: \"{winning_numbers}\"");
            //println!("your_numbers: \"{your_numbers}\"");
            Card {
                winning_numbers: winning_numbers
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
                your_numbers: your_numbers
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}
