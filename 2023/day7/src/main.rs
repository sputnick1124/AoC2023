#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("unexpected card: {}", c),
        }
    }

    fn from_char_with_joker(c: char) -> Self {
        match Self::from_char(c) {
            Self::Jack => Self::Joker,
            card => card,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    High(Hand),
    Pair(Hand),
    TwoPair(Hand),
    Three(Hand),
    FullHouse(Hand),
    Four(Hand),
    Five(Hand),
}

impl HandType {
    fn from_str(input: &str) -> Self {
        Hand::from_str(input).hand_type()
    }

    fn from_str_with_joker(input: &str) -> Self {
        Hand::from_str_with_jokers(input).hand_type().promote()
    }

    fn promote(self) -> Self {
        let hand_copy = match &self {
            Self::High(hand)
                | Self::Pair(hand)
                | Self::TwoPair(hand)
                | Self::Three(hand)
                | Self::FullHouse(hand)
                | Self::Four(hand)
                | Self::Five(hand) => hand.clone()
        };
        match &hand_copy.jokers {
            0 => self,
            1 => match &self {
                Self::High(_) => Self::Pair(hand_copy),
                Self::Pair(_) => Self::Three(hand_copy),
                Self::TwoPair(_) => Self::FullHouse(hand_copy),
                Self::Three(_) => Self::Four(hand_copy),
                _ => Self::Five(hand_copy),
            }
            2 => match &self {
                Self::Pair(_) => Self::Three(hand_copy),
                Self::TwoPair(_) => Self::Four(hand_copy),
                _ => Self::Five(hand_copy),
            }
            3 => match &self {
                Self::Three(_) => Self::Four(hand_copy),
                _ => Self::Five(hand_copy),
            }
            _ => Self::Five(hand_copy),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Hand {
    cards: (Card, Card, Card, Card, Card),
    jokers: usize,
}

impl Hand {
    fn from_str(input: &str) -> Self {
        let mut v: Vec<Card> = input.chars().rev().map(Card::from_char).collect();
        Self {
            cards: (
                v.pop().unwrap(),
                v.pop().unwrap(),
                v.pop().unwrap(),
                v.pop().unwrap(),
                v.pop().unwrap(),
            ), jokers: 0
        }
    }

    fn from_str_with_jokers(input: &str) -> Self {
        let mut v: Vec<Card> = input.chars().rev().map(Card::from_char_with_joker).collect();
        let jokers = v.iter().filter(|&c| *c == Card::Joker).count();
        Self {
            cards: (
                v.pop().unwrap(),
                v.pop().unwrap(),
                v.pop().unwrap(),
                v.pop().unwrap(),
                v.pop().unwrap(),
            ), jokers
        }
    }

    fn hand_type(&self) -> HandType {
        let mut cards = [
            self.cards.0,
            self.cards.1,
            self.cards.2,
            self.cards.3,
            self.cards.4,
        ];
        cards.sort();
        let [a, b, c, d, e] = cards;
        let (w, x, y, z) = (a == b, b == c, c == d, d == e);
        match (w, x, y, z) {
            (true, true, true, true) => HandType::Five(self.clone()),
            (w, true, true, z) if w ^ z => HandType::Four(self.clone()),
            (true, x, y, true) if x ^ y => HandType::FullHouse(self.clone()),
            (true, true, false, false)
            | (false, true, true, false)
            | (false, false, true, true) => HandType::Three(self.clone()),
            (w, x, y, z) if ((w ^ x) && (y ^ z)) && (!(x && y)) => HandType::TwoPair(self.clone()),
            (w, x, y, z) if w ^ x ^ y ^ z => HandType::Pair(self.clone()),
            _ => HandType::High(self.clone()),
        }
    }
}

fn parse_part1(line: &str) -> (HandType, u32) {
    let (left, right) = line.split_once(" ").expect("bad line");
    let hand = HandType::from_str(left);
    let bid = right.parse().expect("not a number");
    (hand, bid)
}

fn parse_part2(line: &str) -> (HandType, u32) {
    let (left, right) = line.split_once(" ").expect("bad line");
    let hand = HandType::from_str_with_joker(left);
    let bid = right.parse().expect("not a number");
    (hand, bid)
}

fn part1(input_str: &str) -> u32 {
    let mut hands = input_str.lines().map(parse_part1).collect::<Vec<_>>();
    hands.sort();
    hands.iter().map(|(_, b)| b).enumerate().fold(0, |acc, (i, b)| acc +(i as u32+1)*b)
}


fn part2(input_str: &str) -> u32 {
    let mut hands = input_str.lines().map(parse_part2).collect::<Vec<_>>();
    hands.sort();
    hands.iter().map(|(_, b)| b).enumerate().fold(0, |acc, (i, b)| acc +(i as u32+1)*b)
}

fn main() {
    let input_str = std::fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&input_str));
    println!("Part 2: {}", part2(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input_str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part1(&input_str), 6440);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part2(&input_str), 5905);
    }
}
