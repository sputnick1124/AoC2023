use std::collections::HashMap;

struct Game {
    id: u32,
    samples: Vec<Set>,
}

impl Game {
    fn is_feasible(&self, rgb: (u32, u32, u32)) -> bool {
        self.max(Color::Red).unwrap_or(0) <= rgb.0
            && self.max(Color::Green).unwrap_or(0) <= rgb.1
            && self.max(Color::Blue).unwrap_or(0) <= rgb.2
    }

    fn max(&self, color: Color) -> Option<u32> {
        self.samples
            .iter()
            .map(|s| s.get_amount(color))
            .max()
    }

    fn min_power(&self) -> u32 {
        let mut colors = HashMap::new();
        colors.insert(Color::Red, self.max(Color::Red).unwrap_or(1));
        colors.insert(Color::Green, self.max(Color::Green).unwrap_or(1));
        colors.insert(Color::Blue, self.max(Color::Blue).unwrap_or(1));
        Set { colors }.power()
    }

    fn from_str(input: &str) -> Self {
        let (game_str, samples_str) = input.split_once(": ").unwrap();
        let (_, game_num_str) = game_str.split_once(" ").unwrap();
        let id = game_num_str.parse().unwrap();

        let samples = samples_str
            .split("; ")
            .map(|s| {
                s.split(", ").fold(HashMap::new(), |mut acc, c| {
                    let (amount, color) = c.split_once(" ").unwrap();
                    acc.insert(Color::from_str(color), amount.parse().unwrap());
                    acc
                })
            })
            .map(|s| Set { colors: s })
            .collect::<Vec<Set>>();
        Self { id, samples }
    }
}

struct Set {
    colors: HashMap<Color, u32>,
}

impl Set {
    fn get_amount(&self, color: Color) -> u32 {
        *self.colors.get(&color).unwrap_or(&0)
    }

    fn power(&self) -> u32 {
        self.colors.get(&Color::Red).unwrap_or(&1) * self.colors.get(&Color::Green).unwrap_or(&1) * self.colors.get(&Color::Blue).unwrap_or(&1)
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn from_str(input: &str) -> Self {
        match input {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("Not a color: {}", input),
        }
    }
}

fn part1(input_str: &str) -> u32 {
    input_str.lines().map(|l| Game::from_str(l)).filter(|g| g.is_feasible((12, 13, 14))).map(|g| g.id).sum()
}

fn part2(input_str: &str) -> u32 {
    input_str.lines().map(|l| Game::from_str(l)).map(|g| g.min_power()).sum()
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
        let input_str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(&input_str), 8);
        assert_eq!(part2(&input_str), 2286);
    }
}
