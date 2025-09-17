/// Public WASM entry point: parse chordbook bytes → Typst bytes
use std::str;
use serde::{Serialize, Deserialize};
use ciborium::into_writer;

#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::wasm_func;

#[cfg(target_arch = "wasm32")]
wasm_minimal_protocol::initiate_protocol!();

#[derive(Debug, Serialize, Deserialize)]
struct Song {
    title: String,
    author: String,
    sections: Vec<Section>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Section {
    name: String,
    lines: Vec<Line>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Line {
    ChordLyric(Vec<ChordLyric>),
    LyricOnly(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct ChordLyric {
    text: String,
    chord: String,
}

/// Public WASM entry point: parse chordbook bytes → CBOR bytes
#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn parse(expr: &[u8]) -> Result<Vec<u8>, String> {
    let input = str::from_utf8(expr)
        .map_err(|e| format!("Input is not valid UTF-8: {e}"))?;

    let song = parse_chordbook(input)?;

    // Encode to CBOR via ciborium
    let mut buf = Vec::new();
    into_writer(&song, &mut buf)
        .map_err(|e| format!("Failed to encode CBOR: {e}"))?;

    Ok(buf)
}

// Parsing logic stays mostly the same as before

fn parse_chordbook(input: &str) -> Result<Song, String> {
    let mut lines = input.lines().peekable();
    let title = lines.next().ok_or("Missing title")?.trim().to_string();
    let author = lines.next().ok_or("Missing author")?.trim().to_string();

    let mut sections = Vec::new();
    let mut current_section: Option<Section> = None;
    let mut chord_line_buffer: Option<String> = None;

    while let Some(line) = lines.next() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            if let Some(section) = current_section.take() {
                sections.push(section);
            }
            let name = line.trim_matches(&['[', ']'][..]).to_string();
            current_section = Some(Section { name, lines: Vec::new() });
            chord_line_buffer = None;
        } else if is_chord_line(line) {
            chord_line_buffer = Some(line.to_string());
        } else if let Some(section) = &mut current_section {
            if let Some(chord_line) = chord_line_buffer.take() {
                let chord_lyrics = align_chords_with_lyrics(&chord_line, line);
                section.lines.push(Line::ChordLyric(chord_lyrics));
            } else {
                section.lines.push(Line::LyricOnly(line.to_string()));
            }
        }
    }
    if let Some(section) = current_section.take() {
        sections.push(section);
    }

    Ok(Song { title, author, sections })
}

fn is_chord_line(line: &str) -> bool {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    !tokens.is_empty() && tokens.iter().all(|t| is_chord(t))
}

fn is_chord(s: &str) -> bool {
    let s = s.trim();
    !s.is_empty()
        && s.chars().next().unwrap().is_ascii_uppercase()
        && !s.contains(',')
        && !s.contains(' ')
}

fn align_chords_with_lyrics(chord_line: &str, lyric_line: &str) -> Vec<ChordLyric> {
    let chord_chars: Vec<char> = chord_line.chars().collect();
    let lyric_chars: Vec<char> = lyric_line.chars().collect();

    // collect (position, chord_name)
    let mut chords: Vec<(usize, String)> = Vec::new();
    let mut idx = 0;
    while idx < chord_chars.len() {
        while idx < chord_chars.len() && chord_chars[idx].is_whitespace() {
            idx += 1;
        }
        if idx >= chord_chars.len() {
            break;
        }
        let start = idx;
        let mut name = String::new();
        while idx < chord_chars.len() && !chord_chars[idx].is_whitespace() {
            name.push(chord_chars[idx]);
            idx += 1;
        }
        chords.push((start, name));
    }

    // if no chords, return whole lyric as a single (empty-chord) segment
    if chords.is_empty() {
        return vec![ChordLyric {
            text: lyric_line.to_string(),
            chord: String::new(),
        }];
    }

    let mut out = Vec::new();
    let lyric_len = lyric_chars.len();

    // initial text before the first chord -> empty chord
    let first_pos = chords[0].0.min(lyric_len);
    if first_pos > 0 {
        let initial_text = lyric_chars[0..first_pos].iter().collect::<String>();
        out.push(ChordLyric {
            text: initial_text,
            chord: String::new(),
        });
    }

    // for each chord, take lyric from its pos to next chord pos (or to end)
    for i in 0..chords.len() {
        let (pos, ref chord) = &chords[i];
        let start = (*pos).min(lyric_len);
        let end = if i + 1 < chords.len() {
            chords[i + 1].0.min(lyric_len)
        } else {
            lyric_len
        };
        if start < end {
            let text = lyric_chars[start..end].iter().collect::<String>();
            out.push(ChordLyric {
                text,
                chord: chord.clone(),
            });
        } else {
            // no lyric under this chord (still keep the chord)
            out.push(ChordLyric {
                text: String::new(),
                chord: chord.clone(),
            });
        }
    }

    out.into_iter()
        .filter(|cl| !cl.text.is_empty() || !cl.chord.is_empty())
        .collect()
}
