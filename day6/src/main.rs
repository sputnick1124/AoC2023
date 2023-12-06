fn parse(input: &str) -> Vec<(u32, u32)> {
    let (time, dist) = input.split_once('\n').unwrap();
    parse_vec(time).zip(parse_vec(dist)).collect()
}

fn parse_vec<'s>(line: &'s str) -> impl Iterator<Item=u32> + 's
{
    line.split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
}

fn wins((time, dist): (u32, u32)) -> u32 {
    (1..time).map(|t| t * (time - t)).filter(|&d| d > dist).count() as u32
}

fn part1(input_str: &str) -> u32 {
    parse(input_str).into_iter().map(wins).fold(1, |acc, x| acc * x)
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
        let input_str = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part1(&input_str), 288);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"";
        assert_eq!(part2(&input_str), 0);
    }
}
