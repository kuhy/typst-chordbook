#import "@preview/conchord:0.4.0": *

#let ahoj = (
    title: "fsdfdsaf",
    author: "Samson",
    sections: ((name: "Verse 1", lines: (
            ((text: "Ah", chord: "C"), ("oj Slunko", "D") ("Jak se máš", "G")),
            (("fsfs", "D")),
            ((text: "fdsafdsafdsafdsf")),
            ((chord: "C D E"))
    ), (name: "Verse 2", lines: (
        (("Jak pak fs", "D"), "aff", "A"))
    )))
)

#box(smart-chord("Am"))
// at what fret to play the chord
#box(smart-chord("Am", at: 5)) // chord at fifth fret
// what variant number to select
#box(smart-chord("Am", n: 4)) // forth "best" chord
// what tuning to use; note first, then octave (1-9)
#box(smart-chord("C", tuning: "G1 C1 E1 A1")) // ukulele

#[
  // to make tonality change automatically
  #show: chordify

  #overchord[A\#9] Lyrics #overchord(styling: emph)[A\#9] Lyrics
  #overchord(styling: fancy-styling-autotonality)[A\#9] Lyrics

  `| tonality change |`

  #change-tonality(1)

  #overchord[A\#9] Lyrics #overchord(styling: emph)[A\#9] Lyrics
  // of course, if you want to use certain a lot, use an alias: 
  #let fo = overchord.with(styling: fancy-styling-autotonality)
  #fo[A\#9] Lyrics

  #sized-chordlib(
    smart-chord: smart-chord.with(styling: it => strong(fancy-styling-plain(it)))
  )
]

= Another Brick in the wall, Pink Floyd

#[
  #show: chordify.with(heading-reset-tonality: 2)

  // in fact, heading-reset just adds change-tonality(0) there
  == Default `overchord`
  [Dm] We dоn't need nо еduсаtiоn, \
  [Dm] We dоn't need nо thоught соntrоl, \
  #change-tonality(1)
  [Dm] Nо dark sarcasm in the сlаssrооm.   \
  [Dm] Teacher leave them kids [G] аlоnе.   \
  [G] Hey, Teacher!  Leave them kids аlоnе.
]

#[
  #show: chordify.with(line-chord: inlinechord, heading-reset-tonality: 2)
  == `inlinechord`
  [Dm] We dоn't need nо еduсаtiоn, \
  [Dm] We dоn't need nо thоught соntrоl, \
  #change-tonality(1)
  [Dm] Nо dark sarcasm in the сlаssrооm. \
  [Dm] Teacher leave them kids [G] аlоnе. \
  [G] Hey, Teacher!  Leave them kids аlоnе. \
]


#[
  #show: chordify.with(line-chord: fulloverchord, heading-reset-tonality: 2)
  == `fulloverchord`
  // chordlib still works!
  #sized-chordlib(width: 150pt, N: 3, prefix: [_Chord library_ #linebreak()])

  [Dm] We dоn't need nо еduсаtiоn, \
  [Dm] We dоn't need nо thоught соntrоl, \
  #change-tonality(1)
  [Dm] Nо dark sarcasm in the сlаssrооm. \
  // every function can be also used directly
  #fulloverchord("Dm", n: 1) Teacher leave them kids #inlinechord[Dm] аlоnе. \
  [G] Hey, Teacher!  Leave them kids аlоnе.
]
