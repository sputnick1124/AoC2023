Input ← &fras "input.txt"

$ Time:      7  15   30
$ Distance:  9  40  200
S ←

A ← (
  ×≤@9:≥@0..
  ⍉↯⊂2÷2⧻.≡(parse ⊔)⊜□
  /×≡(⧻▽:⊙<.×⊙-.+1⇡.⊃⊢(⇌⊡1))
)

B ← (
  ⊜∘≠@\n.
  ≡(parse▽×≤@9:≥@0..)
  ⧻▽:⊙<.×⊙-.+1⇡.⊃⊢(⇌⊡1)
)

A S
$"Part 1 sample: _"
B S
$"Part 2 sample: _"

A Input
$"Part 1: _"
B Input
$"Part 2: _"
