# Experimental!
SplitBlocks ← (
  ⊜(□⊜□⊸≠@\n)¬⦷"\n\n".
  ⍜↙₁(⍚≡◇⊜⋕(⊸≠@|))
  ⍜↘₁(⍚≡⍚⊜(□⋕)(⊸≠@,))
)

RawInput      ← &fras "input.txt"
I             ← SplitBlocks RawInput
DeBoxSections ← ⊃(°□⊢)(°(¤□)↘1)
FixStep ← (
  ⊃(⧈∘2|⋅(¤⍉)|∘|⋅∘)
  ≡(>0/+=▽=⊃(∘|⋅⋅⊙∘|⋅∘)∩°⊟)
  ⨬(⍜⊏⇌[⟜+₁⊗0]|⊡÷2-1⊸⧻◌)⊸/↧
)
FixSection ← ⊙◌⍢FixStep(≠₁⧻)

PartI ← (
  DeBoxSections
  :¤
  /+≡(
    ⊃(⧈∘2|⋅(¤⍉)|∘)◇≡°□
    /↧≡(
      >0/+=▽=⊃(∘|⋅⋅⊙∘|⋅∘)∩°⊟
    )
    ⨬0(⊡÷2-1⊸⧻)
  )
)
PartII ← (
  DeBoxSections
  ⊃(⋅∘|¤|∘)
  ≡(⊃(⧈∘2|⋅(¤⍉)|∘)◇≡°□
    ⊸≡(>0/+=▽=⊃(∘|⋅⋅⊙∘|⋅∘)∩°⊟)
    ⊃(∘|⋅⋅∘|⍉⋅°¤)
    =1/↧
    ⨬⊸FixSection⋅0
    ⊙◌
  )
  /+⊙◌
)

PartI I
PartII I

┌─╴test
  $ 47|53
  $ 97|13
  $ 97|61
  $ 97|47
  $ 75|29
  $ 61|13
  $ 75|53
  $ 29|13
  $ 97|29
  $ 53|29
  $ 61|53
  $ 97|53
  $ 61|29
  $ 47|13
  $ 75|47
  $ 97|75
  $ 47|61
  $ 75|61
  $ 47|29
  $ 75|13
  $ 53|13
  $ 
  $ 75,47,61,53,29
  $ 97,61,53,29,13
  $ 75,29,13
  $ 75,97,47,61,53
  $ 61,13,29
  $ 97,13,75,29,47
  RawInput  ←
  TestInput ← SplitBlocks RawInput
  
  ⍤⤙≍ 143 PartI TestInput

  ⍤⤙≍ 123 PartII TestInput
└─╴
