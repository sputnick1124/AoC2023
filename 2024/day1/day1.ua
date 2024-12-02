I ← &fras "input.txt"

Parse  ← ⊜(⊜⋕≠@ .)⊸≠@\n
PartI  ← /+≡(⌵/-)⍜⍉≡⍆Parse
PartII ← /+×⊙/+⍜⍉(⟜⊞⌕°⊟)Parse

PartI I
PartII I

┌─╴test
  $ 3   4
  $ 4   3
  $ 2   5
  $ 1   3
  $ 3   9
  $ 3   3
  TestInput ←
  
  ⍤. =11 PartI TestInput

  ⍤. =31 PartII TestInput
└─╴
