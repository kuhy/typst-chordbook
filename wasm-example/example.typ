#let _p = plugin("chordpro_parser.wasm")

#let result_bytes = _p.parse(bytes("[C]Hello [G]world\nthis is [F]another [C]line"))

#let result_string = str(result_bytes)

#result_string
