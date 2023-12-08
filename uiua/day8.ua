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

Parse ← (
  ⊃(↘+2)↙⊗@\n.
  ⊙(=@R)
  ⊜∘≠@\n.
  ⊏⍏.
  ≡⊃(↙-)(↘++1)1⊗@=.
  ⊚⊠⌕
  ↯⊟:2÷2⧻.⍜⍉(↘2⍜⍉(⊏⍏.)↻1)
)

PartOne ← (
  Parse
  :0:⊙⊙0 # setup
  ⍢(⊙(⊡⊙(.⊙(+1))⊟:)⊃(↻1)⊢)(;:;⊙(≠⧻:+1))
  ;;;
)

PartOne SampA
$"Part 1 Sample A: _"
PartOne SampB
$"Part 1 Sample B: _"

PartOne Input
$"Part 1: _"
