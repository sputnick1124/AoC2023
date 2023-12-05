struct MapRange {
    dest_start: usize,
    src_start: usize,
    len: usize,
}

impl MapRange {
    fn from_str(input: &str) -> Self {
        let nums = input
            .split_whitespace()
            .map(|x| x.parse().expect("not a number"))
            .collect::<Vec<_>>();
        Self {
            dest_start: nums[0],
            src_start: nums[1],
            len: nums[2],
        }
    }

    fn map(&self, item: usize) -> Option<usize> {
        if item.clamp(self.src_start, self.src_start + self.len) == item {
            Some(self.dest_start + (item - self.src_start))
        } else {
            None
        }
    }
}

struct Map {
    from: String,
    to: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let (line, rest) = input.split_once("\n").unwrap();
        let (from, to) = line
            .strip_suffix(" map:")
            .and_then(|s| s.split_once("-to-"))
            .expect("not a valid map");
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            ranges: rest.lines().map(MapRange::from_str).collect(),
        }
    }

    fn map(&self, item: usize) -> usize {
        self.ranges.iter().find_map(|r| r.map(item)).unwrap_or(item)
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from_str(input: &str) -> Self {
        let mut blocks = input.split("\n\n");
        let seeds = blocks
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .and_then(|s| Some(s.split_whitespace()))
            .expect("bad seeds")
            .map(|n| n.parse().expect("bad seed"))
            .collect();
        let maps = blocks.map(Map::from_str).collect();
        Self { seeds, maps }
    }

    fn map_seed(&self, seed: usize) -> usize {
        self.maps
            .iter()
            .fold(seed, |acc, m| m.map(acc))
    }
}

fn part1(input_str: &str) -> usize {
    let almanac = Almanac::from_str(input_str);
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.map_seed(*seed))
        .min()
        .unwrap()
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
        let input_str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part1(&input_str), 35);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"";
        assert_eq!(part2(&input_str), 0);
    }
}
