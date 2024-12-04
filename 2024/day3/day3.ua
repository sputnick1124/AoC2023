JoinLines ← /◇⊂⊜□⊸≠@\n
I         ← JoinLines &fras "input.txt"

Parse          ← ⋕↘₁⍉regex"mul\\((\\d+),(\\d+)\\)"
RemoveDisabled ← ▽=0/↥≡◇⦷⊸⊙¤≡⊢⊸regex"don't\\(\\).+?(do\\(\\)|$)"

PartI  ← /+/×Parse
PartII ← PartI RemoveDisabled

PartI I
PartII I

┌─╴test
  $ xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

  ⍤. =161 PartI

  $ xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
  ⍤. =48 PartII
└─╴
