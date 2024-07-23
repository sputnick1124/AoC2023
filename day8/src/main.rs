fn part1(input_str: &str) -> u32 {
    unimplemented!()
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
    fn test_part1_sample_a() {
        let input_str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&input_str), 0);
    }

    #[test]
    fn test_part1_sample_b() {
            let input_str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&input_str), 0);
}

    #[test]
    fn test_part2_sample() {
        let input_str = r"";
        assert_eq!(part2(&input_str), 0);
    }
}
