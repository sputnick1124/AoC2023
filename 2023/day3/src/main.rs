use regex::Regex;
use std::collections::HashMap;
use std::iter;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Clone, Copy, Debug)]
struct Number {
    val: u32,
    row: usize,
    start: usize,
    end: usize,
}

struct Schematic {
    numbers: Vec<Number>,
    symbols: HashMap<(usize, usize), char>,
}

impl Schematic {
    fn part_numbers(&self) -> Vec<Number> {
        self.numbers
            .iter()
            .filter(|&n| self.is_part_number(*n))
            .map(|&n| n.clone())
            // .inspect(|x| println!("{:?}", x))
            .collect()
    }

    fn get_neighbors(&self, num: Number) -> impl Iterator<Item = (usize, usize)> {
        iter::once(num.row.checked_sub(1))
            .chain(iter::once(Some(num.row + 1)))
            .filter_map(|x| x)
            .flat_map(move |i| (num.start.saturating_sub(1)..=num.end + 1).map(move |j| (i, j)))
            .chain(
                iter::once(num.start.checked_sub(1).map(|s| (num.row, s)))
                    .chain(iter::once(Some((num.row, num.end + 1))))
                    .filter_map(|x| x),
            )
    }

    fn is_part_number(&self, num: Number) -> bool {
        self.get_neighbors(num)
            .any(|ij| self.symbols.contains_key(&ij))
    }

    fn gear_ratios(&self) -> Vec<u32> {
        self
            .part_numbers()
            .into_iter()
            .flat_map(|n| {
                self.get_neighbors(n).filter_map(move |i| {
                    self.symbols
                        .get(&i)
                        .and_then(|&c| if c == '*' { Some((i, n.val)) } else { None })
                })
            })
            .fold(HashMap::new(), |mut acc, ((i, j), n)| {
                acc.entry((i, j)).or_insert(vec![]).push(n);
                acc
            })
            .into_values()
            .filter(|v| v.len() == 2)
            .map(|v| v.into_iter().fold(1, |acc, x| acc * x))
            .collect()
    }

    fn from_str(input: &str) -> Self {
        // println!("{}", input);
        let numbers = input
            .lines()
            .enumerate()
            .flat_map(|(row, s)| {
                RE.find_iter(s).map(move |m| Number {
                    val: m.as_str().parse().unwrap(),
                    row,
                    start: m.start(),
                    end: m.end() - 1,
                })
            })
            // .inspect(|x| println!("{:?}", x))
            .collect();
        let symbols = input
            .lines()
            .enumerate()
            .flat_map(|(row, s)| {
                s.chars().enumerate().filter_map(move |(col, c)| {
                    if c != '.' {
                        Some(((row, col), c))
                    } else {
                        None
                    }
                })
            })
            .collect::<HashMap<_, _>>();
        // println!("{:?}", symbols);
        Self { numbers, symbols }
    }
}

fn part1(input_str: &str) -> u32 {
    let schematic = Schematic::from_str(input_str);
    schematic.part_numbers().iter().map(|n| n.val).sum()
}

fn part2(input_str: &str) -> u32 {
    let schematic = Schematic::from_str(input_str);
    schematic.gear_ratios().iter().sum()
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
        let input_str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(&input_str), 4361);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(&input_str), 467835);
    }
}
