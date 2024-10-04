//use std::collections::HashMap;

const _INCORRECT: usize = 549927;

fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_03.txt");
    let (parts, symbols) = parse_input(input);

    //for (i, part) in parts.iter().enumerate() {
    //    if i < 10 || i > parts.len() - 10 {
    //        println!("part {}: {:?}", i, part);
    //    }
    //}
    //for (i, symbol) in symbols.iter().enumerate() {
    //    if i < 10 || i > symbols.len() - 10 {
    //        println!("symbol {}: {:?}", i, symbol);
    //    }
    //}

    let mut part_sum = 0;
    for part in parts.iter() {
        for symbol in symbols.iter() {
            if part.adjacent_to(symbol) {
                part_sum += part.part_num;
                //println!("{:?} adjacent to {:?}", part, symbol);
                //println!("part_sum: {}", part_sum);
                break;
            }
        }
    }
    println!("part_sum: {}", part_sum);

    let mut gear_ratio_sum = 0;
    for symbol in symbols.iter() {
        let adjacent_parts: Vec<&Part> = parts
            .iter()
            .filter(|part| part.adjacent_to(symbol))
            .collect();
        if adjacent_parts.len() == 2 {
            gear_ratio_sum += adjacent_parts[0].part_num * adjacent_parts[1].part_num;
        }
    }
    println!("gear_ratio_sum: {}", gear_ratio_sum);

    // Now we have the parts and symbols, we can start solving the puzzle

    //// Solve 3a
    //let sol_3a: usize = 12;
    //let correct_sol_3a: usize = 34;
    //println!("* 3a *");
    //println!("My solution: {sol_3a}");
    //println!("Correct solution: {correct_sol_3a}");
    //println!("Equal: {}\n", sol_3a == correct_sol_3a);
    //
    //// Solve 3b
    //let sol_3b: usize = 56;
    //let correct_sol_3b: usize = 78;
    //println!("* 3b *");
    //println!("My solution: {sol_3b}");
    //println!("Correct solution: {correct_sol_3b}");
    //println!("Equal: {}\n", sol_3b == correct_sol_3b);
}

/// A part somehwere on the grid
#[derive(Debug, Clone)]
struct Part {
    part_num: usize,
    x: usize,
    y: usize,
}

/// A symbol on the grid
#[allow(dead_code)]
#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

impl Part {
    /// Calculate the manhattan distance between this part and another part
    fn adjacent_to(&self, symbol: &Symbol) -> bool {
        // Parse part_num into a string
        let len = self.part_num.to_string().len();
        let min_x = self.x.saturating_sub(1);
        let max_x = self.x + len;
        let min_y = self.y.saturating_sub(1);
        let max_y = self.y + 1;

        // Check if the symbol is within the bounds of the pa
        (min_x..=max_x).contains(&symbol.x) && (min_y..=max_y).contains(&symbol.y)
    }
}

/// This is the state of the parser as we go through finding parts and symbols
fn parse_input(input: &str) -> (Vec<Part>, Vec<Symbol>) {
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        // As we go throu the input we need to keep track of the
        // current part we are parsing.
        let mut parsing_part: Option<Part> = None;

        for (x, c) in line.chars().enumerate() {
            match (&mut parsing_part, c) {
                (None, '.') => {
                    // Just finding a . is a no-op
                    continue;
                }
                (None, '0') => {
                    unimplemented!("0 is not a valid part number prefix");
                }
                (None, '0'..='9') => {
                    if c == '0' {
                        unimplemented!("0 is not a valid part number prefix");
                    }
                    // We are at the start of a new part
                    parsing_part = Some(Part {
                        part_num: c.to_digit(10).unwrap() as usize,
                        x,
                        y,
                    });
                }
                (None, _) => {
                    // We found a new symbol
                    symbols.push(Symbol { x, y });
                }
                (Some(part), '.') => {
                    // We've reached the end of a part
                    parts.push(part.clone());
                    parsing_part = None;
                }
                (Some(part), '0'..='9') => {
                    // We found another digit for this part
                    parsing_part = Some(Part {
                        part_num: part.part_num * 10 + c.to_digit(10).unwrap() as usize,
                        x: part.x,
                        y: part.y,
                    });
                }
                (Some(part), _) => {
                    // We've reached the end of a part and found a symbol
                    parts.push(part.clone());
                    parsing_part = None;
                    symbols.push(Symbol { x, y });
                } //_ => unreachable!("Impossible input sequence"),
            }
        }
        // Finally, if we are still parsing a part, add it to the list
        if let Some(part) = parsing_part {
            parts.push(part);
        }
    }
    (parts, symbols)
}
