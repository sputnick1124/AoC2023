#!/bin/bash
if [[ "$(whoami)" =~ "M" ]]; then
    PASSDIR="personal/"
fi
COOKIE="$(pass ${PASSDIR}nick/aoc/cookie)"

DAY=$1
YEAR=${2:-2023}
curl -q --ssl-no-revoke --cookie "session=$COOKIE" https://adventofcode.com/$YEAR/day/$DAY | \
    pandoc --from=html --to=markdown - > problem.md
