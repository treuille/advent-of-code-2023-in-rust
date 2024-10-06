fn main() {
    // Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_05.txt");
    //let input = include_str!("../../puzzle_inputs/day_05_test.txt");

    // Parse the first line (seeds)
    let mut lines = input.lines();
    let seeds = lines.next().unwrap().split_whitespace();
    let seeds: Vec<usize> = seeds.skip(1).map(|s| s.parse().unwrap()).collect();
    println!("Seeds: {:?}", seeds);

    // Parse the rest of the lines
    let mut maps: Vec<Vec<(usize, usize, usize)>> = Vec::new();
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

    // Convert to seed ranges
    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
    let mut seed_iter = seeds.iter();
    while let Some(&range_start) = seed_iter.next() {
        let &range_len = seed_iter.next().unwrap();
        seed_ranges.push((range_start, range_len));
    }
    println!("Seed ranges: {:?}", seed_ranges);

    //// Solve 5a
    //let sol_5a: usize = solve_5a(&seeds, &maps);
    //let correct_sol_5a: usize = 535088217;
    //println!("* 5a *");
    //println!("My solution: {sol_5a}");
    //println!("Correct solution: {correct_sol_5a}");
    //println!("Equal: {}\n", sol_5a == correct_sol_5a);

    // Solve 5b
    let sol_5b: usize = solve_5b(&seed_ranges, &maps);
    let correct_sol_5b: usize = 79;
    println!("* 5b *");
    println!("My solution: {sol_5b}");
    println!("Correct solution: {correct_sol_5b}");
    println!("Equal: {}\n", sol_5b == correct_sol_5b);
}

#[allow(dead_code)]
fn solve_5a(seeds: &[usize], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    // Find the locations of the seeds in part a
    seeds
        .iter()
        .map(|seed| {
            let mut index = *seed;
            for map in maps.iter() {
                for &(dest_start, src_start, range_len) in map {
                    if index >= src_start && index < src_start + range_len {
                        index = dest_start + (index - src_start);
                        break;
                    }
                }
            }
            index
        })
        .min()
        .unwrap()
}

#[allow(dead_code, unused_variables, unused_mut)]
fn solve_5b(seed_ranges: &[(usize, usize)], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    fn are_disjoint(ranges: &[(usize, usize)]) -> bool {
        ranges
            .iter()
            .zip(ranges.iter().skip(1))
            .all(|(&(start1, len1), &(start2, _))| start1 + len1 <= start2)
    }

    fn count_seeds(ranges: &[(usize, usize)]) -> usize {
        ranges.iter().map(|(_, len)| len).sum()
    }

    fn print_seed_counts(
        src_ranges: &[(usize, usize)],
        dest_ranges: &[(usize, usize)],
        num_seeds: usize,
    ) {
        // Now print the seed counts
        let num_src_seeds: usize = count_seeds(src_ranges);
        let num_dest_seeds: usize = count_seeds(dest_ranges);
        let total_seeds: usize = num_src_seeds + num_dest_seeds;
        println!(
            "Source seeds: {num_src_seeds} Dest seeds: {num_dest_seeds} Total seeds: {total_seeds}",
        );

        // Print the actual ranges
        println!("( Source Ranges: {:?} )", src_ranges);
        println!("( Dest Ranges: {:?} )", dest_ranges);

        // Make sure the total number of seeds is the same
        assert_eq!(total_seeds, num_seeds);

        // Make sure the ranges are disjoint
        assert!(are_disjoint(src_ranges));
        assert!(are_disjoint(dest_ranges));
    }

    // Count the seeds in the initial ranges
    let num_seeds: usize = count_seeds(seed_ranges);
    assert!(are_disjoint(seed_ranges));

    // Count the seeds in the initial ranges
    let num_seeds: usize = count_seeds(seed_ranges);

    // These are the ranges that we're mapping across
    let mut ranges = seed_ranges.to_vec();

    for (iter, map) in maps.iter().enumerate() {
        // These are the ranges that we're mappying to
        let mut dest_ranges: Vec<(usize, usize)> = Vec::new();

        // debug - begin
        println!("\n*** Before iteration {iter} ***");
        print_seed_counts(&ranges, &dest_ranges, num_seeds);
        // debug - end

        for &(dest_start, src_start, range_len) in map.iter() {
            //assert!(dest_start > 0);
            let src_end = src_start + range_len;

            // "Consume" the ranges into an iter, run then through the map filling (ranges, dest_ranges)
            let ranges_iter = ranges.into_iter();
            ranges = Vec::new();

            for (range_start, range_len) in ranges_iter {
                assert!(range_len > 0);
                let range_end = range_start + range_len;

                // Divide the range tino three segments from
                // min(range_start, src_start) to max(range_end, src_end).
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
                        if segment_start >= src_start && range_end <= src_end {
                            // This segment is contained in the map range
                            let dest_segment_start = dest_start + (segment_start - src_start);
                            //assert!(dest_segment_start >= dest_start);
                            //assert!(segment_len <= range_len);
                            //assert!(dest_segment_start > 0);
                            dest_ranges.push((dest_segment_start, segment_len));
                        } else {
                            // This segment is not contained in the map range
                            ranges.push((segment_start, segment_len));
                        }
                    }
                }
            }

            // debug - begin
            println!("*** After map ({dest_start}, {src_start}, {range_len}) ***");
            print_seed_counts(&ranges, &dest_ranges, num_seeds);
            // debug - end
        }
        // debug - begin
        println!("\n*** After iteration {iter} ***");
        print_seed_counts(&ranges, &dest_ranges, num_seeds);
        // debug - end

        // Any final ranges left over use the identity map
        dest_ranges.extend(ranges.iter().cloned());

        // Swap the ranges and dest_ranges
        ranges = dest_ranges;
    }

    println!("\n*** Final ***");
    print_seed_counts(&ranges, &[], num_seeds);

    // Find the minimum index in the ranges
    let min_index = ranges.iter().map(|(start, _)| start).min().unwrap();
    println!("Min index: {}", min_index);

    unimplemented!("solve_5b not implemented");
}
