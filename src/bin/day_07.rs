use std::cmp::Ordering;

/// A hande is a length-5 array of cards
type Hand = [u8; 5];

/// A Puzzle is a list of hands and their bids
type Puzzle = Vec<(Hand, usize)>;

fn main() {
    //// Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_07.txt");
    //let input = include_str!("../../puzzle_inputs/day_07_test.txt");
    let mut input: Puzzle = parse_input(input.lines());

    //println!("* Input *");
    //for (hand, bid) in input.iter() {
    //    println!(
    //        "Hand: {}, Type: {:?}, Bid: {}",
    //        to_string(hand),
    //        HandType::from_hand(hand),
    //        bid
    //    );
    //}

    // Solve 7a
    let sol_7a: usize = solve(&mut input);
    let correct_sol_7a: usize = 256448566;
    println!("* 7a *");
    println!("My solution: {sol_7a}");
    println!("Correct solution: {correct_sol_7a}");
    println!("Equal: {}\n", sol_7a == correct_sol_7a);

    //println!("* Sorted Input *");
    //for (hand, bid) in input.iter() {
    //    println!(
    //        "Hand: {}, Type: {:?}, Bid: {}",
    //        to_string(hand),
    //        HandType::from_hand(hand),
    //        bid
    //    );
    //}

    //// Solve 7b
    //let sol_7b: usize = 56;
    //let correct_sol_7b: usize = 79;
    //println!("* 7b *");
    //println!("My solution: {sol_7b}");
    //println!("Correct solution: {correct_sol_7b}");
    //println!("Equal: {}\n", sol_7b == correct_sol_7b);
}

fn solve(input: &mut Puzzle) -> usize {
    input.sort_unstable_by(cmp_hand_and_bid);

    input
        .iter()
        .zip(1..)
        .map(|((_hand, bid), order)| bid * order)
        .sum()
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

fn to_string(hand: &Hand) -> String {
    hand.iter()
        .map(|c| {
            match c {
                0 => '*', // <- 0 means Joker
                10 => 'T',
                11 => 'J', // <- 11 means Jack
                12 => 'Q',
                13 => 'K',
                14 => 'A',
                c => c.to_string().chars().next().unwrap(),
            }
        })
        .collect()
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum HandType {
    // all cards' labels are distinct: 23456
    HighCard,

    // two cards share one label, and the other three cards have a different label from the pair
    // and each other: A23A4
    OnePair,

    // two cards share one label, two other cards share a second label, and the remaining card has
    // a third label: 23432
    TwoPair,

    // three cards have the same label, and the remaining two cards are each different from any
    // other card in the hand: TTT98
    ThreeOfAKind,

    // three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    // Four of a kind, where four cards have the same label and one card has a different label:
    // AA8AA
    FourOfAKind,

    // all five cards have the same label: AAAAA
    FiveOfAKind,
}

impl HandType {
    fn from_hand(hand: &Hand) -> Self {
        let mut counts: [usize; 15] = [0; 15];
        for card in hand.iter() {
            counts[*card as usize] += 1;
        }

        let mut pairs = 0;
        let mut triples = 0;
        let mut quads = 0;
        let mut quints = 0;
        for count in counts.iter() {
            match count {
                0 | 1 => continue,
                2 => pairs += 1,
                3 => triples += 1,
                4 => quads += 1,
                5 => quints += 1,
                _ => {
                    println!("Invalid hand: {}", to_string(hand));
                    println!("Counts: {:?}", counts);
                    println!("Pairs: {}", pairs);
                    println!("Triples: {}", triples);
                    println!("Quads: {}", quads);
                    println!("Quints: {}", quints);
                    unreachable!("Cannot have more than 5 cards of the same label")
                }
            }
        }

        match (pairs, triples, quads, quints) {
            (0, 0, 0, 0) => HandType::HighCard,
            (1, 0, 0, 0) => HandType::OnePair,
            (2, 0, 0, 0) => HandType::TwoPair,
            (0, 1, 0, 0) => HandType::ThreeOfAKind,
            (0, 0, 1, 0) => HandType::FourOfAKind,
            (0, 0, 0, 1) => HandType::FiveOfAKind,
            (1, 1, 0, 0) => HandType::FullHouse,
            _ => panic!("Invalid hand: {}", to_string(hand)),
        }
    }
}

fn cmp_hand_and_bid((h1, b1): &(Hand, usize), (h2, b2): &(Hand, usize)) -> Ordering {
    let ord = HandType::from_hand(h1).cmp(&HandType::from_hand(h2));
    if ord != Ordering::Equal {
        return ord;
    }

    let ord = h1.cmp(h2);
    if ord != Ordering::Equal {
        return ord;
    }

    b1.cmp(b2)
    //
    //match
    //    Ordering::Equal => {
    //        for (card, other_card) in self.0.iter().zip(other.0.iter()) {
    //            match card.cmp(other_card) {
    //                Ordering::Equal => continue,
    //                x => return x,
    //            }
    //        }
    //        Ordering::Equal
    //    }
    //    x => x,
    //}
}

fn parse_input(lines: impl Iterator<Item = &'static str>) -> Puzzle {
    lines
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (to_hand(hand), bid.parse().unwrap())
        })
        .collect()
}
