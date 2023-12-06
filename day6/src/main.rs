fn parse_part1(input: &str) -> Vec<(u128, u128)> {
    let (time, dist) = input.split_once('\n').unwrap();
    parse_vec(time).zip(parse_vec(dist)).collect()
}

fn parse_vec<'s>(line: &'s str) -> impl Iterator<Item=u128> + 's
{
    line.split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
}

fn parse_part2(input: &str) -> (u128, u128) {
    let (time, dist) = input.split_once('\n').unwrap();
    (parse(time), parse(dist))
}

fn parse(line: & str) -> u128
{
    line.split_whitespace()
        .skip(1)
        .fold(String::new(), |mut acc, s| { acc.push_str(s); acc })
        .parse()
        .expect("bad number")
}

fn wins((time, dist): (u128, u128)) -> u128 {
    (1..time).map(|t| t * (time - t)).filter(|&d| d > dist).count() as u128
}

fn part1(input_str: &str) -> u128 {
    parse_part1(input_str).into_iter().map(wins).fold(1, |acc, x| acc * x)
}

fn part2(input_str: &str) -> u128 {
    wins(parse_part2(input_str))
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
        let input_str = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(&input_str), 288);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"Time:      71530
    Distance:  940200";
        assert_eq!(part2(&input_str), 71503);
    }
}
