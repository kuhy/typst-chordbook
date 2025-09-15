use std::fs;
use std::fmt::Write;

#[derive(Debug)]
struct Song {
    title: String,
    author: String,
    sections: Vec<Section>,
}

#[derive(Debug)]
struct Section {
    name: String,
    lines: Vec<Line>,
}

#[derive(Debug)]
enum Line {
    ChordLyric(Vec<ChordLyric>),
    LyricOnly(String),
}

#[derive(Debug)]
struct ChordLyric {
    text: String,
    chord: String,
}

fn parse_chordbook_file(input_path: &str, output_path: &str) -> Result<(), String> {
    let input = fs::read_to_string(input_path).map_err(|e| format!("Failed to read input file: {e}"))?;
    let song = parse_chordbook(&input)?;
    let typst = song_to_typst(&song);
    fs::write(output_path, typst).map_err(|e| format!("Failed to write output file: {e}"))?;
    Ok(())
}

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
        } else {
            if let Some(section) = &mut current_section {
                if let Some(chord_line) = chord_line_buffer.take() {
                    let chord_lyrics = align_chords_with_lyrics(&chord_line, line);
                    section.lines.push(Line::ChordLyric(chord_lyrics));
                } else {
                    section.lines.push(Line::LyricOnly(line.to_string()));
                }
            }
        }
    }
    if let Some(section) = current_section.take() {
        sections.push(section);
    }

    Ok(Song { title, author, sections })
}

// Returns true if line is a chord line (all tokens look like chords)
fn is_chord_line(line: &str) -> bool {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    !tokens.is_empty() && tokens.iter().all(|t| is_chord(t))
}

// Heuristic for chord: starts with uppercase letter, may contain numbers, /, etc.
fn is_chord(s: &str) -> bool {
    let s = s.trim();
    !s.is_empty()
        && s.chars().next().unwrap().is_ascii_uppercase()
        && !s.contains(',')
        && !s.contains(' ')
}

fn align_chords_with_lyrics(chord_line: &str, lyric_line: &str) -> Vec<ChordLyric> {
    let mut result = Vec::new();
    let chord_chars: Vec<char> = chord_line.chars().collect();
    let lyric_chars: Vec<char> = lyric_line.chars().collect();

    // Find chord positions (column indices) and chord names
    let mut chord_positions = Vec::new();
    let mut idx = 0;
    while idx < chord_chars.len() {
        // Skip spaces
        while idx < chord_chars.len() && chord_chars[idx].is_whitespace() {
            idx += 1;
        }
        if idx >= chord_chars.len() {
            break;
        }
        // Start of chord
        let start = idx;
        let mut chord = String::new();
        while idx < chord_chars.len() && !chord_chars[idx].is_whitespace() {
            chord.push(chord_chars[idx]);
            idx += 1;
        }
        chord_positions.push((start, chord));
    }

    // Now, for each chord, take lyric substring from previous chord position up to this chord position
    let mut prev_pos = 0;
    for (i, &(chord_pos, ref chord)) in chord_positions.iter().enumerate() {
        let end_pos = if i + 1 < chord_positions.len() {
            chord_positions[i + 1].0
        } else {
            lyric_chars.len()
        };
        let start_pos = chord_pos;
        let end_pos = end_pos.min(lyric_chars.len());
        // Only slice if start_pos <= end_pos
        if prev_pos <= start_pos && start_pos < end_pos {
            let text = lyric_chars[start_pos..end_pos].iter().collect::<String>();
            result.push(ChordLyric {
                text,
                chord: chord.clone(),
            });
        }
        prev_pos = start_pos;
    }
    // First segment: from 0 to first chord position
    if !chord_positions.is_empty() {
        let first_chord_pos = chord_positions[0].0;
        if first_chord_pos > 0 {
            let first_text = lyric_chars[0..first_chord_pos.min(lyric_chars.len())].iter().collect::<String>();
            result.insert(0, ChordLyric {
                text: first_text,
                chord: chord_positions[0].1.clone(),
            });
        }
    } else {
        // No chords, just lyric
        result.push(ChordLyric {
            text: lyric_line.to_string(),
            chord: String::new(),
        });
    }

    // Remove empty text segments (but keep chords)
    result.into_iter().filter(|cl| !cl.text.is_empty() || !cl.chord.is_empty()).collect()
}

fn song_to_typst(song: &Song) -> String {
    let mut out = String::new();
    writeln!(&mut out, "#let song = (").unwrap();
    writeln!(&mut out, "    title: {:?},", song.title).unwrap();
    writeln!(&mut out, "    author: {:?},", song.author).unwrap();
    writeln!(&mut out, "    sections: (").unwrap();
    for section in &song.sections {
        writeln!(&mut out, "        (name: {:?}, lines: (", section.name).unwrap();
        for line in &section.lines {
            match line {
                Line::ChordLyric(parts) => {
                    write!(&mut out, "            (").unwrap();
                    for part in parts {
                        write!(
                            &mut out,
                            "(text: {:?}, chord: {:?}), ",
                            part.text, part.chord
                        )
                        .unwrap();
                    }
                    writeln!(&mut out, "),").unwrap();
                }
                Line::LyricOnly(text) => {
                    writeln!(&mut out, "            ((text: {:?})),", text).unwrap();
                }
            }
        }
        writeln!(&mut out, "        )),").unwrap();
    }
    writeln!(&mut out, "    )").unwrap();
    writeln!(&mut out, ")").unwrap();
    writeln!(&mut out, "#song").unwrap();
    out
}

fn main() {
    let input_path = "../example_input.txt";
    let output_path = "../example_output_generated.typ";
    match parse_chordbook_file(input_path, output_path) {
        Ok(_) => println!("Parsing complete! Output written to {output_path}"),
        Err(e) => eprintln!("Error: {}", e),
    }
}