use regex::Regex;
use std::iter;
use std::collections::HashMap;

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

    fn is_part_number(&self, num: Number) -> bool {
        [num.row.checked_sub(1), Some(num.row + 1)]
            .iter()
            .filter_map(|&x| x)
            .flat_map(|i| (num.start.saturating_sub(1)..=num.end + 1).map(move |j| (i, j)))
            .chain(
                iter::once(num.start.checked_sub(1).map(|s| (num.row, s)))
                .chain(iter::once(Some((num.row, num.end+1))))
                .filter_map(|x| x)
            )
            .any(|ij| self.symbols.contains_key(&ij))
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
        let input_str = r"";
        assert_eq!(part2(&input_str), 0);
    }
}
