#!/bin/bash
if [[ "$(whoami)" =~ "M" ]]; then
    PASSDIR="personal/"
fi
COOKIE="$(pass ${PASSDIR}nick/aoc/cookie)"
DAY=$1
if [ -z $1 ]; then
    echo need day
    exit 1
fi
YEAR=${2:-2024}
OUTPUT_FILE=${3:-input}

THISDIR=$(dirname $0)
curl -k --ssl-no-revoke --cookie "session=$COOKIE" https://adventofcode.com/$YEAR/day/$DAY/input > $OUTPUT_FILE.txt
$THISDIR/get_problem.sh $DAY $YEAR

if test -f src/main.rs; then
cat > src/main.rs << EOF
fn part1(input_str: &str) -> u32 {
    unimplemented!()
}

fn part2(input_str: &str) -> u32 {
    unimplemented!()
}

fn main() {
    let input_str = std::fs::read_to_string("$OUTPUT_FILE.txt").expect("Unable to read file");
    println!("Part 1: {}", part1(&input_str));
    println!("Part 2: {}", part2(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input_str = r"";
        assert_eq!(part1(&input_str), 0);
    }

    #[test]
    fn test_part2_sample() {
        let input_str = r"";
        assert_eq!(part2(&input_str), 0);
    }
}
EOF
fi
