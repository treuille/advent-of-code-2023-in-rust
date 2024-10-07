use std::cmp::Ordering;

/// A `Hand` is a length-5 array of cards
type Hand = [u8; 5];

/// A `HandType` is a rank-ordered enum of poker hands
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// A `Puzzle` is a list of hands and their bids
type Puzzle = Vec<(Hand, usize)>;

fn main() {
    // Solve 7a
    let mut input: Puzzle = parse_input(include_str!("../../puzzle_inputs/day_07.txt"));
    let sol_7a: usize = solve(&mut input);
    let correct_sol_7a: usize = 256448566;
    println!("* 7a *");
    println!("My solution: {sol_7a}");
    println!("Correct solution: {correct_sol_7a}");
    println!("Equal: {}\n", sol_7a == correct_sol_7a);

    // Solve 7b
    convert_jacks_to_jokers(&mut input);
    let sol_7b: usize = solve(&mut input);
    let correct_sol_7b: usize = 254412181;
    println!("* 7b *");
    println!("My solution: {sol_7b}");
    println!("Correct solution: {correct_sol_7b}");
    println!("Equal: {}\n", sol_7b == correct_sol_7b);
}

fn solve(input: &mut Puzzle) -> usize {
    input.sort_unstable_by(cmp_hand_and_bid);
    input.iter().zip(1..).map(|((_, bid), i)| bid * i).sum()
}

fn to_hand(s: &str) -> Hand {
    let hand: Vec<u8> = s
        .chars()
        .map(|c| match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            c => c.to_digit(10).unwrap() as u8,
        })
        .collect();
    [hand[0], hand[1], hand[2], hand[3], hand[4]]
}

impl HandType {
    fn from_hand(hand: &Hand) -> Self {
        let mut counts: [usize; 15] = [0; 15];
        for card in hand.iter() {
            counts[*card as usize] += 1;
        }

        let jokers = counts[0];
        let mut pairs: usize = 0;
        let mut triples: usize = 0;
        let mut quads: usize = 0;
        let mut quints: usize = 0;
        for count in &counts[2..] {
            match count {
                0 | 1 => continue,
                2 => pairs += 1,
                3 => triples += 1,
                4 => quads += 1,
                5 => quints += 1,
                _ => unreachable!("Invalid hand: {:?}", hand),
            }
        }

        match (pairs, triples, quads, quints, jokers) {
            (0, 0, 0, 0, 0) => HandType::HighCard,
            (0, 0, 0, 0, 1) => HandType::OnePair,
            (0, 0, 0, 0, 2) => HandType::ThreeOfAKind,
            (0, 0, 0, 0, 3) => HandType::FourOfAKind,
            (0, 0, 0, 0, 4) => HandType::FiveOfAKind,
            (0, 0, 0, 0, 5) => HandType::FiveOfAKind,
            (1, 0, 0, 0, 0) => HandType::OnePair,
            (1, 0, 0, 0, 1) => HandType::ThreeOfAKind,
            (1, 0, 0, 0, 2) => HandType::FourOfAKind,
            (1, 0, 0, 0, 3) => HandType::FiveOfAKind,
            (2, 0, 0, 0, 0) => HandType::TwoPair,
            (2, 0, 0, 0, 1) => HandType::FullHouse,
            (0, 1, 0, 0, 0) => HandType::ThreeOfAKind,
            (0, 1, 0, 0, 1) => HandType::FourOfAKind,
            (0, 1, 0, 0, 2) => HandType::FiveOfAKind,
            (0, 0, 1, 0, 0) => HandType::FourOfAKind,
            (0, 0, 1, 0, 1) => HandType::FiveOfAKind,
            (0, 0, 0, 1, 0) => HandType::FiveOfAKind,
            (1, 1, 0, 0, 0) => HandType::FullHouse,
            _ => panic!("Invalid hand: {:?}", hand),
        }
    }
}

fn cmp_hand_and_bid((h1, _): &(Hand, usize), (h2, _): &(Hand, usize)) -> Ordering {
    match HandType::from_hand(h1).cmp(&HandType::from_hand(h2)) {
        Ordering::Equal => h1.cmp(h2),
        ord => ord,
    }
}

fn parse_input(input: &str) -> Puzzle {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (to_hand(hand), bid.parse().unwrap())
        })
        .collect()
}

fn convert_jacks_to_jokers(input: &mut Puzzle) {
    for (hand, _) in input.iter_mut() {
        for card in hand.iter_mut() {
            if *card == 11 {
                *card = 0;
            }
        }
    }
}
