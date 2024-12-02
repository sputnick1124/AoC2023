I ← &fras "input.txt"

Parse      ← ⊜(□⊜⋕≠@ .)⊸≠@\n
IsIncOrDec ← ⨬(≍⇌)1 ◡≍⊃⍏(⇡⧻)
IsInBounds ← =₁/×=⊃≥₁≤₃⌵⍜⍉/-◫₂
IsSafe     ← ⨬0 IsInBounds⊸IsIncOrDec

Dampener      ← ≡▽⊃(⊞≠.⇡)↯⊸⧻
CheckDampened ← /↥≡IsSafe Dampener
# We could just use CheckDampened as the only check, but it
# ends up being faster to do the simple IsSafe check first
# and only fall back to the dampened check if necessary
IsSafeDampened ← ⨬CheckDampened1 ⊸IsSafe

PartI  ← /+≡◇IsSafe Parse
PartII ← /+≡◇IsSafeDampened Parse

PartI I
PartII I

┌─╴test
  $  7 6 4 2 1
  $  1 2 7 8 9
  $  9 7 6 2 1
  $  1 3 2 4 5
  $  8 6 4 4 1
  $  1 3 6 7 9
  TestInput ←
  
  ⍤. =2 PartI TestInput

  ⍤. =4 PartII TestInput
└─╴
