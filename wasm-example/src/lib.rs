#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

use pest::Parser;
use pest_derive::Parser;
use std::fmt::Write;

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
