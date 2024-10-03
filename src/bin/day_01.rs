fn main() {
    //let nothing = 123;
    //println!("Hello, my GIANT {nothing} {} world!!!", nothing + 2);

    let input = include_str!("../../puzzle_inputs/day_01.txt");
    //println!("Input:\n{}", input);
    println!("Input length: {}", input.len());
    let answer = input
        .lines()
        .map(|line| {
            //println!("line: {}", line);
            let get_first_digit = |char_iter: &mut dyn Iterator<Item = char>| {
                char_iter
                    .filter(|c| c.is_ascii_digit())
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32
            };
            let first_digit = get_first_digit(&mut line.chars());
            let last_digit = get_first_digit(&mut line.chars().rev());
            let result = first_digit * 10 + last_digit;
            //println!("first: {first_digit} last: {last_digit} result: {result}",);
            result
        })
        .sum::<i32>();
    println!("Answer: {}", answer);
}
