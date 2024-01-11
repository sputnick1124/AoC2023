I ← &fras "input.txt"
$ ...#......
$ .......#..
$ #.........
$ ..........
$ ......#...
$ .#........
$ .........#
$ ..........
$ .......#..
$ #...#.....
II ←

P ← =@#⊜∘≠@\n.
Z ← =∩⧻⊃∘(▽=0.)

S ← (
  ⊃≡Z⍜⍉≡Z. # get empty rows/cols
  ⊟∩(\+)   # accumulate index offsets
  ⊙×:      # add multiplier
  ⍉⊚       # get naive indices of galaxies
  ⍉≡(+⊏,:) # add offsets per index
  ⊠(/+⌵-). # calculate manhattan distance
  ⊠>.⇡⧻.   # upper traingular matrix
  /+♭×     # mask out wanted values
  :        # swap for string interp
  &p $ "Part _: _"
)

S P II 1 1
S P I 1 1
S P II 99 2
S P I 999999 2
