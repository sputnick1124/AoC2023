WordNums ← {"one" "two" "three" "four" "five" "six" "seven" "eight" "nine"}

Input ← &fras "input.txt"

$ two1nine
$ eightwothree
$ abcone2threexyz
$ xtwone3four
$ 4nineeightseven2
$ zoneight234
$ 7pqrstsixteen
SampTwo ←

$ 1abc2
$ pqr3stu8vwx
$ a1b2c3d4e5f
$ treb7uchet
SampOne ←

PartOne ← (
  ⬚@ ⊜∘≠@\n.
  +1⇡9
  ⊙⊠(⌕+@0).
  /+×:
  ≡(+⊃(×10⊢)(⊢⇌)▽≠0.)
  /+
)

PartTwo ← (
  ⬚@ ⊜∘≠@\n.
  ⊃(⊠(⊐⌕)WordNums)(⊠⌕+@0+1⇡9)
  +∩(/+×+1⇡9)
  ≡(+×10⊃⊢(⊢⇌)▽≠0.)
  /+
)

PartOne SampOne
$"Part 1 sample: _"
PartTwo SampTwo
$"Part 2 sample: _"

PartOne Input
$"Part 1: _"
PartTwo Input
$"Part 2: _"
