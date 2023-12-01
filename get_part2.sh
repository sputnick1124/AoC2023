#!/bin/sh
SOURCE_DIR=$(dirname "${BASH_SOURCE[0]}")
source ${SOURCE_DIR}/cookie.sh

DAY=$1
YEAR=${2:-2023}
curl -q --cookie "session=$COOKIE" https://adventofcode.com/$YEAR/day/$DAY > problem.md
