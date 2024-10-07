/// A `Map` is a set of `(dest_start, src_start, range_len)` tuples
type Map = Vec<(usize, usize, usize)>;

/// A `SeedRange` is a `(start, len)` tuples
type SeedRange = (usize, usize);

fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    let (seeds, maps): (Vec<usize>, Vec<Map>) = parse_input(input.lines());

    // Solve 5a
    let sol_5a: usize = solve_5a(&seeds, &maps);
    let correct_sol_5a: usize = 535088217;
    println!("* 5a *");
    println!("My solution: {sol_5a}");
    println!("Correct solution: {correct_sol_5a}");
    println!("Equal: {}\n", sol_5a == correct_sol_5a);

    // Solve 5b
    let sol_5b: usize = solve_5b(&seeds, &maps);
    let correct_sol_5b: usize = 51399228;
    println!("* 5b *");
    println!("My solution: {sol_5b}");
    println!("Correct solution: {correct_sol_5b}");
    println!("Equal: {}\n", sol_5b == correct_sol_5b);
}

fn parse_input(mut lines: impl Iterator<Item = &'static str>) -> (Vec<usize>, Vec<Map>) {
    // Parse the first line which describes the seeds
    let toks = lines.next().unwrap().split_whitespace();
    let seeds: Vec<usize> = toks.skip(1).map(|s| s.parse().unwrap()).collect();

    // Parse the rest of the lines
    let mut maps: Vec<Map> = Vec::new();
    for line in lines {
        let toks: Vec<&str> = line.split_whitespace().collect();
        match toks.len() {
            0 => continue,
            2 => maps.push(Vec::new()),
            3 => {
                let dest_start: usize = toks[0].parse().unwrap();
                let src_start: usize = toks[1].parse().unwrap();
                let range_len: usize = toks[2].parse().unwrap();
                maps.last_mut()
                    .unwrap()
                    .push((dest_start, src_start, range_len));
            }
            x => panic!("Unexpected number of tokens: {}", x),
        }
    }

    (seeds, maps)
}

/// Solve 5a by interpreting each seed as a length-1 SeedRange
fn solve_5a(seeds: &[usize], maps: &[Map]) -> usize {
    let seed_ranges: Vec<SeedRange> = seeds.iter().map(|&seed| (seed, 1)).collect();
    solve(&seed_ranges, maps)
}

/// Solve 5a by interpreting each pair of seed as a SeedRange
fn solve_5b(seeds: &[usize], maps: &[Map]) -> usize {
    let seed_ranges: Vec<SeedRange> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    solve(&seed_ranges, maps)
}

/// Puzzles 5a and 5b are the same, except for the interpretation of the seeds
fn solve(seed_ranges: &[SeedRange], maps: &[Map]) -> usize {
    // These are the ranges that we're mapping across
    let mut ranges = seed_ranges.to_vec();

    for map in maps.iter() {
        // These are the ranges that we're mappying to
        let mut dest_ranges: Vec<SeedRange> = Vec::new();

        for &(dest_start, src_start, map_len) in map.iter() {
            let src_end = src_start + map_len;

            // "Consume" the ranges into an iter, run then through the map filling (ranges, dest_ranges)
            let ranges_iter = ranges.into_iter();
            ranges = Vec::new();

            for (range_start, range_len) in ranges_iter {
                let range_end = range_start + range_len;

                // Split `(range_start, range_end) union (src_start, src_end)` into 3 segments
                let mut idx = [range_start, range_end, src_start, src_end];
                idx.sort();
                let segments = idx.iter().zip(idx.iter().skip(1));

                // Iterate over the segments and add them to the ranges or dest_ranges
                for (&segment_start, &segment_end) in segments {
                    // Skip empty segments
                    let segment_len = segment_end - segment_start;
                    if segment_len == 0 {
                        continue;
                    }

                    if segment_start >= range_start && segment_end <= range_end {
                        // This segment is in the range (start, end), so keep it
                        if segment_start >= src_start && segment_end <= src_end {
                            // This segment is contained in the map range
                            let dest_segment_start = dest_start + (segment_start - src_start);
                            dest_ranges.push((dest_segment_start, segment_len));
                        } else {
                            // This segment is not contained in the map range
                            ranges.push((segment_start, segment_len));
                        }
                    }
                }
            }
        }
        // Any final ranges left over use the identity map
        dest_ranges.extend(ranges.iter().cloned());

        // Swap the ranges and dest_ranges
        ranges = dest_ranges;
    }

    // Find the minimum index in the ranges
    *ranges.iter().map(|(start, _)| start).min().unwrap()
}
