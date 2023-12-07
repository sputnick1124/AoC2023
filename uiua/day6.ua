Input ← &fras "input.txt"

$ Time:      7  15   30
$ Distance:  9  40  200
Samp ←

Race ← (
  ⊃⊢(⇌⊡1)  # decouple element pair
  ×⊙-.+1⇡. # get distances
  ⧻▽:⊙<.   # count winning distances
)

PartOne ← (
  ×≤@9:≥@0..           # get indices of digits
  ⍉↯⊂2÷2⧻.≡(parse ⊔)⊜□ # extract numbers and shape as table
  ≡Race                # for each row
  /×                   # multiply reduce
)

PartTwo ← (
  ⊜∘≠@\n.             # split on newline
  ≡(parse▽×≤@9:≥@0..) # mash digits together and parse
  Race
)

PartOne Samp
$"Part 1 sample: _"
PartTwo Samp
$"Part 2 sample: _"

PartOne Input
$"Part 1: _"
PartTwo Input
$"Part 2: _"
