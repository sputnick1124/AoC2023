SplitLines ← ⊜∘⊸≠@\n
I          ← SplitLines&fras "input.txt"

Needle ← "XMAS"
Diags  ← ⊂∩(⬚@ ⊕∘∩♭)⊙,⍜⍉(⊸⇌≡/+)⇡⊸△

PartI  ← ++∩₃(/+/++⊃(⌕⇌)⌕)⊃(⊙∘|⊙⍉|⊙Diags)Needle
PartII ← ∘

PartI I
# PartII I

┌─╴test
  $ MMMSXXMASM
  $ MSAMXMSMSA
  $ AMXSXMAAMM
  $ MSAMASMSMX
  $ XMASAMXAMM
  $ XXAMMXXAMA
  $ SMSMSASXSS
  $ SAXAMASAAA
  $ MAMMMXMMMM
  $ MXMXAXMASX
  TestInput ← SplitLines
  
  ⍤. =18 PartI TestInput

  # ⍤. = PartII
└─╴
