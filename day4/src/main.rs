use std::collections::HashSet;

struct Card {
    id: usize,
    winning: HashSet<u32>,
    revealed: HashSet<u32>,
}

impl Card {
    fn from_str(input: &str) -> Self {
        let s = input.strip_prefix("Card ").map(|x| x.trim()).expect("no 'Card' label");
        let (id_str, num_str) = s.split_once(": ").unwrap();
        let id = id_str.parse().expect("id is not a number");
        let (winning_str, revealed_str) = num_str.split_once(" | ").unwrap();
        let winning = winning_str.split_whitespace().map(|n| n.parse().expect("winning num not a num")).collect();
        let revealed = revealed_str.split_whitespace().map(|n| n.parse().expect("revealed num not a num")).collect();
        Self{id, winning,revealed}
    }

    fn score(&self) -> u32 {
        self.winning.intersection(&self.revealed).count().checked_sub(1).map(|p| 2u32.pow(p as u32)).unwrap_or(0) as u32
    }
}

fn part1(input_str: &str) -> u32 {
    input_str.lines().into_iter().map(Card::from_str).map(|c| c.score()).sum()
}

fn part2(input_str: &str) -> u32 {
    unimplemented!()
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
        let input_str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(part1(&input_str), 13);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"";
        assert_eq!(part2(&input_str), 0);
    }
}
