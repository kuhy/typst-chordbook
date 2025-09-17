#import "@preview/conchord:0.4.0": overchord, aligned-chords
// #import "example_output.typ": song
#import "@preview/chordx:0.6.0": single-chord
#import "@preview/itemize:0.1.2"

#set page("a5")

#let _p = plugin("chordpro_parser.wasm")

#let result_bytes = _p.parse(read("example_input.txt", encoding: none))

#let song = cbor(result_bytes)

#song

#show ref: itemize.ref-enum
#show: itemize.default-enum-list.with(body-indent: 0.4cm)

#let oc(
  /// chord name to attach. Should be plain string for tagging to work -> str
  text,
  /// styling function that is applied to the string -> (text <chord>) => content
  styling: strong,
  /// alignment of the word above the point -> alignment
  align: start,
  /// height of the chords -> length
  height: 1em,
  /// width of space in current font,
  /// may be set to zero if you don't put
  /// any spaces between chords and words -> length
  width: 0em) = {
    box(place(align, box(styling([#text <chord>]), width: float("inf")*1pt)), height: 1em + height, width: width)
    sym.zwj
  }

//a #oc("C") a


#let chorded(ch, txt) = block(
  align(center)[
    #strong(ch)
    #txt
  ]
)


//#(box(place(start, box([kj <fsa>], width: float("inf")*1pt)), height: 1em + 1em, width: -0.25em))

//fdsa #chorded("C", "A")fas

#let render-line(line) = [#(
  for seg in line {
    if "ChordLyric" in seg {
        for chordlyr in seg.at(1) {
        overchord(chordlyr.chord, width: 0em, height: 0.6em)

        [#chordlyr.text]
        //aligned-chords[#seg.chord][#seg.text]
        }
    } else if "chord" in seg {
        //overchord(seg.chord)
        //aligned-chords[#seg.chord][~~~~~~]
    } else if "LyricOnly" in seg {
        seg.at(1)
    }
  }
)]

#let render-section(section) = [
    #box(
        inset: (left: 0.7cm),
       box(
           inset: (left: -0.7cm),
            stroke: (left: 3pt),
           {
               set list(marker: box(width: 0.5cm, [#h(1fr) R1:])) if section.name != "Verse 1"
               set list(marker: box(width: 0.5cm, [#h(1fr) 1.])) if section.name == "Verse 1"
               list.item(for line in section.lines {
                   render-line(line)
                   linebreak()
               })
           }
        )
    )
]

    #grid(columns: 3, gutter: 5pt, align: center + horizon, [#block(
  fill: luma(230),
  inset: 8pt,
  radius: 4pt,
  [*1*],
    )],
        [#heading(song.title)], [#h(1fr)#(text(12pt)[_#(song.author)_])]
    )
#h(0pt)
    #place(dy: -13pt, line(length: 100%))

#for section in song.sections {
    render-section(section)
}

