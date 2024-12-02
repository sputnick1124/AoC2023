InputFile ← ⬚"input.txt"⊡1&args
Input ← &fras InputFile

Race ← (
  ⊃⊢(⇌⊡1)  # decouple element pair
  ×⊙-.+1⇡. # get distances
  ⧻▽:⊙<.   # count winning distances
)

# Solves part one of the challenge
PartOne ← (
  ×≤@9:≥@0..      # get indices of digits
  ⍉↯⊂2÷2⧻.≡(⊔⋕)⊜□ # extract numbers and shape as table
  ≡Race           # for each row
  /×              # multiply reduce
)

PartTwo ← (
  ⊜∘≠@\n.         # split on newline
  ≡(⋕▽×≤@9:≥@0..) # mash digits together and parse
  Race
)

PartOne Input
$"Part 1: _"
PartTwo Input
$"Part 2: _"

---
$ Time:      7  15   30
$ Distance:  9  40  200
Samp ←

⍤⊃⋅∘≍ 288 PartOne Samp
⍤⊃⋅∘≍ 71503 PartTwo Samp
---
