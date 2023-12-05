fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)))
        .map(|mut c| {
            let a = c.nth(0).unwrap();
            if let Some(b) = c.last() {
                [a, b]
            } else {
                [a, a]
            }
        })
        .map(|c| {
            c.iter().fold(String::new(), |mut acc, d| {
                acc.push(*d);
                acc
            })
        })
        .map(|d| d.parse::<u32>().unwrap())
        .sum()
}

fn extract_num(word: &str) -> Vec<u32> {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    println!("{}", word);
    let mut word_nums: Vec<(usize, usize)> = nums
        .iter()
        .enumerate()
        .flat_map(|(i, w)| word.match_indices(*w).map(move |(b, _)| (b, i + 1)))
        .collect();
    word_nums.sort_by_key(|p| p.0);
    word_nums
        .iter()
        .map(|(_, i)| *i as u32)
        .chain(
            word.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap()),
        )
        .collect()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .inspect(|w| println!("{:?}", w))
        .map(|l| {
            l.split_inclusive(char::is_numeric)
                .flat_map(|w| extract_num(w))
        })
        .inspect(|x| println!("{:?}", x))
        .map(|mut nums| {
            let a = nums.by_ref().take(1).next().unwrap();
            if let Some(b) = nums.last() {
                a * 10 + b
            } else {
                a * 10 + a
            }
        })
        .sum()
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
    fn test_part1() {
        let input_str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(part1(input_str), 142);
    }

    #[test]
    fn test_part2() {
        let input_str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        assert_eq!(part2(input_str), 281);
    }

    #[test]
    fn test_number_extraction() {
        assert_eq!(extract_num("rvrr1"), vec![1]);
        assert_eq!(extract_num("54568"), vec![5, 4, 5, 6, 8]);
        assert_eq!(
            extract_num("sixeightthreesixfivevf3qfpvzvnt"),
            vec![6, 8, 3, 6, 5, 3]
        );
    }
}
