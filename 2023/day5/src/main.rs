use std::ops::Range;

struct MapRange {
    dest: Range<usize>,
    src: Range<usize>,
}

impl MapRange {
    fn from_str(input: &str) -> Self {
        let nums = input
            .split_whitespace()
            .map(|x| x.parse().expect("not a number"))
            .collect::<Vec<_>>();
        Self {
            dest: nums[0]..nums[0] + nums[2],
            src: nums[1]..nums[1] + nums[2],
        }
    }

    fn map(&self, item: usize) -> Option<usize> {
        if self.src.contains(&item) {
            Some(self.dest.start() + (item - self.src.start()))
        } else {
            None
        }
    }

    fn map_range(&self, range: &SeedRange) -> (Option<SeedRange>, Option<SeedRange>) {
        let (start, stop) = (range.0.start(), range.0.stop());
        let mapped_start = self.map(start);
        let mapped_end = self.map(stop);
        let one = Some(start).and_then(|a| mapped_start.
        let mapped1 = mapped_start.map_or(self.dest.start(), |a| mapped_end.map_or(self.dest.stop(), |b| a..b));
        let mapped2 = mapped_start.and_then(|a| mapped_end.map_or(self.dest.stop(), |b| a..b));
        let unmapped = mapped.map_or(self.|r| r.stop()+1)
        match (self.src.contains(start), self.src.contains(stop)) {
            (true, true) => (Some(SeedRange(self.map(start)..self.map(stop))), None),
            (true, false) => (Some(SeedRange(self.map(start)..self.src.stop())), Some(SeedRange(self.src.stop()..stop))),
            (false, true) => (Some(SeedRange(self.dest.start()..self.map(stop))), Some(SeedRange(start..self.dest.start()))),
            (false, false) => (None, Some(SeedRange(start..stop))),
        }
    }
}

struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let (_, rest) = input.split_once("\n").unwrap();
        Self {
            ranges: rest.lines().map(MapRange::from_str).collect(),
        }
    }

    fn map(&self, item: usize) -> usize {
        self.ranges.iter().find_map(|r| r.map(item)).unwrap_or(item)
    }

    fn map_range(&self, range: &SeedRange) -> Vec<SeedRange> {
        let 
    }
}

struct SeedRange(Range<usize>)

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
        self.maps.iter().fold(seed, |acc, m| m.map(acc))
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

fn part2(input_str: &str) -> usize {
    let almanac = Almanac::from_str(input_str);
    almanac
        .seeds
        .chunks(2)
        .flat_map(|sl| (sl[0]..sl[0] + sl[1]).map(|seed| almanac.map_seed(seed)))
        .min()
        .unwrap()
}

fn main() {
    let input_str = std::fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&input_str));
    println!("Part 2: {}", part2(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT_STR: &str = r"seeds: 79 14 55 13

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

    #[test]
    fn test_part1_sample() {
        assert_eq!(part1(&INPUT_STR), 35);
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(part2(&INPUT_STR), 46);
    }
}
