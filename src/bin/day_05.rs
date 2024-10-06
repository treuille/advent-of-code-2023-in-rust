fn main() {
    // Parse the input, counting the number of matches per card
    //let input = include_str!("../../puzzle_inputs/day_05.txt");
    let input = include_str!("../../puzzle_inputs/day_05_test.txt");

    println!("input len: {}", input.len());
    //println!("input:\n{}", input);
    println!("First line: {}", input.lines().next().unwrap());

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
                let source_start: usize = toks[1].parse().unwrap();
                let range_len: usize = toks[2].parse().unwrap();
                maps.last_mut()
                    .unwrap()
                    .push((dest_start, source_start, range_len));
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

    // Solve 5a
    let sol_5a: usize = solve_5a(&seeds, &maps);
    let correct_sol_5a: usize = 535088217;
    println!("* 5a *");
    println!("My solution: {sol_5a}");
    println!("Correct solution: {correct_sol_5a}");
    println!("Equal: {}\n", sol_5a == correct_sol_5a);

    // Solve 5b
    let sol_5b: usize = solve_5b(&seed_ranges, &maps);
    let correct_sol_5b: usize = 79;
    println!("* 5b *");
    println!("My solution: {sol_5b}");
    println!("Correct solution: {correct_sol_5b}");
    println!("Equal: {}\n", sol_5b == correct_sol_5b);
}

fn solve_5a(seeds: &[usize], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    // Find the locations of the seeds in part a
    seeds
        .iter()
        .map(|seed| {
            let mut index = *seed;
            for map in maps.iter() {
                for &(dest_start, source_start, range_len) in map {
                    if index >= source_start && index < source_start + range_len {
                        index = dest_start + (index - source_start);
                        break;
                    }
                }
            }
            index
        })
        .min()
        .unwrap()
}

fn solve_5b(seed_ranges: &[(usize, usize)], maps: &[Vec<(usize, usize, usize)>]) -> usize {
    let seed_ranges = maps.iter().fold(seed_ranges.to_vec(), |seed_ranges, map| {
        let mut dest_ranges: Vec<(usize, usize)> = Vec::new();
        let mut src_ranges = seed_ranges.clone();
        for &(dest_start, map_start, range_len) in map.iter() {
            let map_end = map_start + range_len;
            let mut src_ranges_iter = src_ranges.into_iter();
            src_ranges = Vec::new();
            for (src_start, src_len) in src_ranges_iter {
                let src_end = src_start + src_len;
                // Let's think about the possibilites here
                if src_start < map_start {
                    // 1. src_start < src_end <= map_start < map_end
                    if src_end <= map_start {
                        assert!(src_start < src_end && src_end <= map_start && map_start < map_end);

                        // The soure ranges is untouched
                        src_ranges.push((src_start, src_len));
                    }
                    // 2. src_start < map_start < src_end <= map_end
                    else if src_end <= map_end {
                        assert!(src_start < map_start && map_start < src_end && src_end <= map_end);
                        src_ranges.push((src_start, map_start - src_start));
                        dest_ranges.push((dest_start, src_end - map_start));
                    }
                    // 3. src_start < map_start < map_end < src_end
                    else if src_end > map_end {
                        assert!(src_start < map_start && map_start < map_end && map_end < src_end);
                        src_ranges.push((src_start, map_start - src_start));
                        dest_ranges.push((dest_start, range_len));
                        src_ranges.push((map_end, src_end - map_end));
                    }

                    else {
                        unreachable!("Impossible ordering #1 -- src_start: {src_start}, src_end: {src_end}, map_start: {map_start}, map_end: {map_end}");
                    }
                }
                else if src_start >= map_start {
                    // 4. map_start <= src_start < src_end <= map_end
                    if src_end <= map_end {
                        assert!(map_start <= src_start && src_start < src_end && src_end <= map_end);
                        dest_ranges.push((dest_start + (src_start - map_start), range_len));
                    }

                    else if src_end > map_end {
                        // 5. map_start <= src_start < map_end < src_end
                        if src_start < map_end {
                            assert!(map_start <= src_start && src_start < map_end && map_end < src_end);
                            dest_ranges.push((dest_start + (src_start - map_start), map_end - src_start));
                            src_ranges.push((map_end, src_end - map_end));
                        }

                        // 6. map_start < map_end <= src_start < src_end
                        else if map_end <= src_start {
                            assert!(map_start < map_end && map_end <= src_start && src_start < src_end);
                            src_ranges.push((src_start, src_len));
                        }

                        // This should be impossible
                        else {
                            unreachable!("Impossible ordering #4 -- src_start: {src_start}, src_end: {src_end}, map_start: {map_start}, map_end: {map_end}");
                        }
                    }

                    // This should be impossible
                    else {
                        unreachable!("Impossible ordering #3 -- src_start: {src_start}, src_end: {src_end}, map_start: {map_start}, map_end: {map_end}");
                    }
                }

                // This should be impossible
                else {
                    unreachable!("Impossible ordering #2 -- src_start: {src_start}, src_end: {src_end}, map_start: {map_start}, map_end: {map_end}");
                }

            }
        }
        println!("seed_ranges: {:?}", seed_ranges);
        println!("map: {:?}", map);
        seed_ranges
    });
    println!("final seed_ranges: {:?}", seed_ranges);
    unimplemented!("Working on solve_5b");

    //// Find the locations of the seeds in part a
    //seeds
    //    .iter()
    //    .map(|seed| {
    //        let mut index = *seed;
    //        for map in maps.iter() {
    //            for &(dest_start, source_start, range_len) in map {
    //                if index >= source_start && index < source_start + range_len {
    //                    index = dest_start + (index - source_start);
    //                    break;
    //                }
    //            }
    //        }
    //        index
    //    })
    //    .min()
    //    .unwrap()
}
