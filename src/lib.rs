pub mod grid {
    use ndarray::Array2;

    /// Parses a sring representing a grid of characters into a 2D array.
    pub fn parse_char_grid<T, F>(input: &str, parse_char: F) -> Array2<T>
    where
        F: Fn(char) -> T,
        T: Clone,
    {
        let lines: Vec<&str> = input.trim().lines().collect();
        let rows = lines.len();
        let input: Vec<T> = lines
            .iter()
            .flat_map(|line| line.trim().chars())
            .map(parse_char)
            .collect();
        let cols = input.len() / rows;
        Array2::from_shape_vec((rows, cols), input).unwrap()
    }

    /// Returns the (up to four) valid compass neighbors of a position in a grid.
    pub fn neighbors(
        (x, y): (usize, usize),
        (w, h): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        assert!(x < w);
        assert!(y < h);
        [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            (x + 1 < w).then_some((x + 1, y)),
            (y + 1 < h).then_some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
    }
}

pub mod graph;
pub mod parse_regex;

#[cfg(test)]
mod tests {
    use super::grid::{neighbors, parse_char_grid};
    use ndarray::arr2;
    use std::collections::HashSet;

    //#[test]
    //fn pair_parse_regex() {
    //    let re = Regex::new(r"(\d+) -> ([a-z]+)").unwrap();
    //    let (a, b): (usize, &str) = parse_line(&re, "1 -> abc");
    //    assert_eq!(a, 1);
    //    assert_eq!(b, "abc");
    //}
    //
    //#[test]
    //fn triple_parse_regex() {
    //    let re = Regex::new(r"(\d+) \+\+ ([a-z]+) \+\+ ([a-z])").unwrap();
    //    let (a, b, c): (usize, &str, char) = parse_line(&re, "99 ++ xyz ++ q");
    //    assert_eq!(a, 99);
    //    assert_eq!(b, "xyz");
    //    assert_eq!(c, 'q');
    //}
    //
    //#[test]
    //fn quadruple_parse_regex() {
    //    let re = Regex::new(r"(\-?\d+),(\d+),(\d+),(\d+)").unwrap();
    //    let (a, b, c, d): (isize, usize, usize, usize) = parse_line(&re, "-1,0,1,2");
    //    assert_eq!(a, -1);
    //    assert_eq!(b, 0);
    //    assert_eq!(c, 1);
    //    assert_eq!(d, 2);
    //}
    //
    //#[test]
    //fn lines_parse_regexs() {
    //    let input = "
    //    a => x
    //    b => y
    //    c => z
    //    ";
    //
    //    let re = Regex::new(r"([a-z]) => ([a-z])").unwrap();
    //    let mut iter = parse_lines(re, input);
    //
    //    assert_eq!(Some(('a', 'x')), iter.next());
    //    assert_eq!(Some(('b', 'y')), iter.next());
    //    assert_eq!(Some(('c', 'z')), iter.next());
    //    assert_eq!(None, iter.next());
    //}
    //
    #[test]
    fn test_grid_parsing() {
        let to_u32 = |c: char| c.to_digit(10).unwrap();
        let input = "\n123\n 456\n";
        let array = parse_char_grid(input, to_u32);
        let expected = arr2(&[[1, 2, 3], [4, 5, 6]]);
        assert_eq!(array.dim(), (2, 3));
        assert_eq!(array, expected);
    }

    #[test]
    fn test_neighbors() {
        fn assert_set_eq<S1, S2>(s1: S1, s2: S2)
        where
            S1: IntoIterator<Item = (usize, usize)>,
            S2: IntoIterator<Item = (usize, usize)>,
        {
            let s1: HashSet<_> = s1.into_iter().collect();
            let s2: HashSet<_> = s2.into_iter().collect();
            assert_eq!(s1, s2);
        }

        let dim = (3, 4);
        assert_set_eq(neighbors((0, 0), dim), [(0, 1), (1, 0)]);
        assert_set_eq(neighbors((1, 0), dim), [(0, 0), (1, 1), (2, 0)]);
        assert_set_eq(neighbors((0, 1), dim), [(0, 0), (1, 1), (0, 2)]);
        assert_set_eq(neighbors((1, 1), dim), [(0, 1), (2, 1), (1, 0), (1, 2)]);
        assert_set_eq(neighbors((2, 3), dim), [(1, 3), (2, 2)]);
    }
}
