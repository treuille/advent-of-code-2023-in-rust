use std::cmp::Ordering;
use std::fmt;

fn main() {
    //// Parse the input, counting the number of matches per card
    let input = include_str!("../../puzzle_inputs/day_07.txt");
    //let input = include_str!("../../puzzle_inputs/day_07_test.txt");
    let mut input: Vec<(Hand, usize)> = parse_input(input.lines());

    //println!("* Input *");
    //for (hand, bid) in input.iter() {
    //    println!(
    //        "Hand: {}, Type: {:?}, Bid: {}",
    //        hand,
    //        HandType::from_hand(hand),
    //        bid
    //    );
    //}

    input.sort();

    //println!("* Sorted Input *");
    //for (hand, bid) in input.iter() {
    //    println!(
    //        "Hand: {}, Type: {:?}, Bid: {}",
    //        hand,
    //        HandType::from_hand(hand),
    //        bid
    //    );
    //}

    // Solve 7a
    let answer: usize = input
        .iter()
        .zip(1..)
        .map(|((_hand, bid), order)| bid * order)
        .sum();
    println!("Answer: {}", answer);

    //// Solve 7a
    //let sol_7a: usize = 12;
    //let correct_sol_7a: usize = 32;
    //println!("* 7a *");
    //println!("My solution: {sol_7a}");
    //println!("Correct solution: {correct_sol_7a}");
    //println!("Equal: {}\n", sol_7a == correct_sol_7a);
    //
    //// Solve 7b
    //let sol_7b: usize = 56;
    //let correct_sol_7b: usize = 79;
    //println!("* 7b *");
    //println!("My solution: {sol_7b}");
    //println!("Correct solution: {correct_sol_7b}");
    //println!("Equal: {}\n", sol_7b == correct_sol_7b);
}

#[derive(Eq, Clone, Copy)]
struct Card(char);

impl Card {
    fn val(&self) -> usize {
        match self.0 {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            c => c.to_digit(10).unwrap() as usize,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val().cmp(&other.val())
    }
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
        for card in hand.0.iter() {
            counts[card.val()] += 1;
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
                    println!("Invalid hand: {}", hand);
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
            _ => panic!("Invalid hand: {}", hand),
        }
    }
}

#[derive(Eq)]
struct Hand([Card; 5]);

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match HandType::from_hand(self).cmp(&HandType::from_hand(other)) {
            Ordering::Equal => {
                for (card, other_card) in self.0.iter().zip(other.0.iter()) {
                    match card.cmp(other_card) {
                        Ordering::Equal => continue,
                        x => return x,
                    }
                }
                Ordering::Equal
            }
            x => x,
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.0[0].0, self.0[1].0, self.0[2].0, self.0[3].0, self.0[4].0
        )
    }
}

fn parse_input(lines: impl Iterator<Item = &'static str>) -> Vec<(Hand, usize)> {
    lines
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand: Vec<Card> = hand.chars().map(Card).collect();
            let hand = Hand([hand[0], hand[1], hand[2], hand[3], hand[4]]);
            let bid = bid.parse().unwrap();
            (hand, bid)
        })
        .collect()
}
