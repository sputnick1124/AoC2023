I ← &fras "input.txt"

PreParse ← ♭↙₁⍉regex"don't\\(\\).+?(do\\(\\)|$)"
Parse    ← ⋕↘₁⍉regex"mul\\((\\d+),(\\d+)\\)"

PartI  ← /+/×Parse
PartII ← PartI▽=0⊙°¤/↥⊸≡(⦷°□)⊙¤PreParse.

PartI I
PartII I

┌─╴test
  $ xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

  ⍤. =161 PartI

  $ xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
  ⍤. =48 PartII
└─╴
