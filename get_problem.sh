#!/bin/bash
SOURCE_DIR=$(dirname "${BASH_SOURCE[0]}")
source ${SOURCE_DIR}/cookie.sh

DAY=$1
YEAR=${2:-2023}
curl -q --ssl-no-revoke --cookie "session=$COOKIE" https://adventofcode.com/$YEAR/day/$DAY | \
    pandoc --from=html --to=markdown - > problem.md
