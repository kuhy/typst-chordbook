use std::fs;
use std::fmt::Write;

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

use pest::Parser;
use pest_derive::Parser;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[derive(Parser)]
#[grammar = "chordpro.pest"]
struct ChordProParser;

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn parse(expr: &[u8]) -> Result<Vec<u8>, String> {
    let input = std::str::from_utf8(expr).map_err(|e| e.to_string())?;
    let pairs = ChordProParser::parse(Rule::file, input).map_err(|e| e.to_string())?;

    let mut output = String::new();

    for line in pairs {
        writeln!(&mut output, "{:#?}", line).unwrap();
    }

    Ok(output.into_bytes())
}

fn main() {
    let input = fs::read("../example_input.txt").expect("Failed to read input file");
    match parse(&input) {
        Ok(output) => {
            fs::write("../example_output_generated.typ", output).expect("Failed to write output file");
            println!("Parsing complete! Output written to example_output.typ");
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

