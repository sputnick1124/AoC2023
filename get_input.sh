#!/bin/bash
SOURCE_DIR=$(dirname "${BASH_SOURCE[0]}")
source ${SOURCE_DIR}/cookie.sh
DAY=$1
if [ -z $1 ]; then
    echo need day
    exit 1
fi
YEAR=${2:-2023}
OUTPUT_FILE=${3:-input}
echo "$COOKIE"

THISDIR=$(dirname $0)
curl --cookie "session=$COOKIE" https://adventofcode.com/$YEAR/day/$DAY/input > $OUTPUT_FILE.txt
$THISDIR/get_problem.sh $DAY $YEAR

if ! test -f src/main.rs; then
cat > src/main.rs << EOF
fn day1(input_str: &str) -> u32 {
    unimplemented!()
}

fn day2(input_str: &str) -> u32 {
    unimplemented!()
}

fn main() {
    let input_str = std::fs::read_to_string("$OUTPUT_FILE.txt").expect("Unable to read file");
    println!("Day 1: {}", day1(&input_str));
    println!("Day 2: {}", day2(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_sample() {
        let input_str = r"";
        assert_eq!(day1(&input_str), 0);
    }

    #[test]
    fn test_day2_sample() {
        let input_str = r"";
        assert_eq!(day2(&input_str), 0);
    }
}
EOF
fi
