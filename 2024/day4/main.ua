# Experimental!
SplitLines ← ⊜∘⊸≠@\n
I          ← SplitLines&fras "input.txt"

Needle ← "XMAS"
Diags  ← ⊂∩(⬚@ ⊕∘∩♭)⊙,⍜⍉(⊸⇌≡/+)⇡⊸△

NeedleII ← [
  "M M"
  " A "
  "S S"]
Needles ← [⊃(∘|⍉|⇌|⍉⇌)NeedleII]

PartI  ← ++∩₃(/+/++⊃(⌕⇌)⌕)⊃(⊙∘|⊙⍉|⊙Diags)Needle
PartII ← /+♭⧈⍜¤(/+⊞(=5/+♭=)Needles)3_3

PartI I
PartII I

┌─╴test
  TestInput ← (
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
    SplitLines
  )
  
  ⍤. =18 PartI TestInput

  ⍤. =9 PartII TestInput
└─╴
