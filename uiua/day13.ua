I ← &fras "input.txt"

$ #.##..##.
$ ..#.##.#.
$ ##......#
$ ##......#
$ ..#.##.#.
$ ..##..##.
$ #.#.##.#.
$
$ #...##..#
$ #....#..#
$ ..##..###
$ #####.##.
$ #####.##.
$ ..##..###
$ #....#..#
II ←

PP ← ⊜□¬↥⊃∘(↻¯1)⌕"\n\n". # separate blocks
P ← =@#⊜∘≠@\n.           # parse block into bitmask matrix

# S! takes the whole input and a reducing scoring function
S! ← (
  PP # get blocks
  ≡(
    # for every block
    ⊃∘⍉⊐P # get bitmask and its transpose
    ∩(
      # for bitmask and transpose
      ⊠(/+=0=).  # cross with itself to determine num of per-row differences
      ⍜⍉≡/+⇡△.   # get indices of elements in anti-diagonals
      ⊕^1∩♭      # extract anti-diagonals with reducing transformation
      °¤↘1⍉↯¯1_2 # throw away odd-numbered diagonals (since reflection has to be in-between rows)
      ×⊃∊(+1⊗)1  # get index of matching row pair
    )
    +×100 # calc final score
  )
  /+ # sum across blocks
)

S!(/×=0) I # reflection with no differences
&p $"Part 1: _"
S!(=2/+) I # reflection with single difference (2 because symmetrical matrix)
&p $"Part 2: _"
