Input ← &fras "input.txt"

$ Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
$ Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
$ Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
$ Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
$ Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
$ Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
Samp ←

Parse ← (
  ⊜∘≠@\n.       # splitlines
  ≡↘+1⊗@:.      # remove prefix up to ':'
  ≡⊃(↘+1)↙⊗@|.  # split into 2 arrays at '|'
  ∩(⊐∵⋕≡⊜□≠@ .) # split whitespace and parse each array
)

Matches ← ≡(/+∊) # size of intersection for each row

PartOne ← ⁿ:2-1▽≠,0 # calculate score

PartTwo ← (
  # takes match count as unary arg on stack
  ⊃∘(↯:1⧻) # init card count array with 1's
  ⧻.       # length for repetition
  ⍥(
    ∩⊃(↻1)⊢ # rotate and get first element of both arrays
    ⊙⍜↙+    # add first card count to first match count card counts
  )         # repeat
  ;         # kill match count array and return
)

Solve ← ∩/+⊃PartOne PartTwo Matches Parse

Solve Samp

$"Part 1 Sample: _"
:
$"Part 2 Sample: _"

Solve Input

$"Part 1 Input: _"
:
$"Part 2 Input: _"
