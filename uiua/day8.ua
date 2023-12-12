Input ← &fras "input.txt"

$ RL
$
$ AAA = (BBB, CCC)
$ BBB = (DDD, EEE)
$ CCC = (ZZZ, GGG)
$ DDD = (DDD, DDD)
$ EEE = (EEE, EEE)
$ GGG = (GGG, GGG)
$ ZZZ = (ZZZ, ZZZ)
SampA ←

$ LLR
$
$ AAA = (BBB, BBB)
$ BBB = (AAA, ZZZ)
$ ZZZ = (ZZZ, ZZZ)
SampB ←

$ LR
$
$ 11A = (11B, XXX)
$ 11B = (XXX, 11Z)
$ 11Z = (11B, XXX)
$ 22A = (22B, XXX)
$ 22B = (22C, 22C)
$ 22C = (22Z, 22Z)
$ 22Z = (22B, 22B)
$ XXX = (XXX, XXX)
SampC ←

Parse ← (
  ⊃(↘+2)↙⊗@\n.
  ⊙(=@R)
  ⊜∘≠@\n.
  ⊏⍏.
  ≡⊃(↙-)(↘++1)1⊗@=.
)

Prep ← (
  ⊚⊠⌕
  ↯⊟:2÷2⧻.⍜⍉(↘2⍜⍉(⊏⍏.)↻1)
)

Step ← (
  ⊃(↻1)⊢ # get and cycle next column index
  ⊙(
    # dip into stack to work on indices
    ≡⊟:¤ # couple row and column index
    ⊙(
      ⊙(+1) # double-dip down to increment step count
      .     # copy node tree for next iteration
    )
    ⊡ # extract next node for next row index
  )
)

DoneOne ← (
  # stack is:
  # Step count
  # Node tree array
  # Current row index
  # RL steps array
  ⊙(
    # dip past RL array
    ≠⧻:+1°¤ # check if we are at the last (ZZZ) row
  )
  ;:; # throw away all but boolean check result
)

PartOne ← (
  Parse
  Prep
  :0:⊙⊙0 # setup
  ⍢(Step)(DoneOne)
  ;;;
)

PrepTwo ← (
  ∩(♭⍜⍉(↘2)⌕)@A,@Z
  ∩⊚
)

DoneTwo ← (
  ⊙(;;;)≠1/×∊⊙:
)

PartTwo ← (
  Parse
  ⊙Prep.
  PrepTwo
  ⊙(:⊙(:⊙(:0))):
  ⍢(⊙Step)(DoneTwo)
  ;;;;
)

PartOne SampA
$"Part 1 sample a: _"
PartOne SampB
$"Part 1 sample b: _"

PartTwo SampC
$"Part 2 sample c: _"

PartOne Input
$"Part 1: _"

PartTwo Input
$"Part 1: _"
