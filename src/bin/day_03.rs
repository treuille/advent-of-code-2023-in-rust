fn main() {
    // Parse the input
    let input = include_str!("../../puzzle_inputs/day_03.txt");
    let (parts, symbols) = parse_input(input);

    // Solve 3a
    let sol_3a: usize = solve_3a(&parts, &symbols);
    let correct_sol_3a: usize = 546312;
    println!("* 3a *");
    println!("My solution: {sol_3a}");
    println!("Correct solution: {correct_sol_3a}");
    println!("Equal: {}\n", sol_3a == correct_sol_3a);

    // Solve 3b
    let sol_3b: usize = solve_3b(&parts, &symbols);
    let correct_sol_3b: usize = 87449461;
    println!("* 3b *");
    println!("My solution: {sol_3b}");
    println!("Correct solution: {correct_sol_3b}");
    println!("Equal: {}\n", sol_3b == correct_sol_3b);
}

#[derive(Clone)]
struct Part {
    part_num: usize,
    x: usize,
    y: usize,
}

struct Symbol {
    x: usize,
    y: usize,
}

impl Part {
    fn adjacent_to(&self, symbol: &Symbol) -> bool {
        let part_len = self.part_num.to_string().len();
        let min_x = self.x.saturating_sub(1);
        let max_x = self.x + part_len;
        let min_y = self.y.saturating_sub(1);
        let max_y = self.y + 1;

        // Adjacent symbols lie within this bounding box
        (min_x..=max_x).contains(&symbol.x) && (min_y..=max_y).contains(&symbol.y)
    }
}

fn solve_3a(parts: &[Part], symbols: &[Symbol]) -> usize {
    parts
        .iter()
        .filter_map(|part| {
            symbols
                .iter()
                .find(|symbol| part.adjacent_to(symbol))
                .map(|_| part.part_num)
        })
        .sum()
}

fn solve_3b(parts: &[Part], symbols: &[Symbol]) -> usize {
    symbols
        .iter()
        .filter_map(|symbol| {
            let adjacent_parts: Vec<&Part> = parts
                .iter()
                .filter(|part| part.adjacent_to(symbol))
                .collect();
            (adjacent_parts.len() == 2)
                .then(|| adjacent_parts[0].part_num * adjacent_parts[1].part_num)
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<Part>, Vec<Symbol>) {
    let mut parts: Vec<Part> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        // As we go through the input, keep track of the part we're parsing.
        let mut parsing_part: Option<Part> = None;

        for (x, c) in line.chars().enumerate() {
            match (&mut parsing_part, c) {
                // Just finding a . is a no-op
                (None, '.') => {
                    continue;
                }

                // We are at the start of a new part
                (None, '0'..='9') => {
                    parsing_part = Some(Part {
                        part_num: c.to_digit(10).unwrap() as usize,
                        x,
                        y,
                    });
                }

                // We found a new symbol
                (None, _) => {
                    symbols.push(Symbol { x, y });
                }

                // We've reached the end of a part
                (Some(part), '.') => {
                    parts.push(part.clone());
                    parsing_part = None;
                }

                // We found another digit for this part
                (Some(part), '0'..='9') => {
                    parsing_part = Some(Part {
                        part_num: part.part_num * 10 + c.to_digit(10).unwrap() as usize,
                        x: part.x,
                        y: part.y,
                    });
                }

                // We've reached the end of a part and found a symbol
                (Some(part), _) => {
                    parts.push(part.clone());
                    parsing_part = None;
                    symbols.push(Symbol { x, y });
                }
            }
        }

        // Finally, if we are still parsing a part, add it to the list
        if let Some(part) = parsing_part {
            parts.push(part);
        }
    }
    (parts, symbols)
}
