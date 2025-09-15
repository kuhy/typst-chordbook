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
    ChordOnly(Vec<String>),
    LyricOnly(String),
}

fn parse_chordbook_file(input_path: &str, output_path: &str) -> Result<(), String> {
    let input = fs::read_to_string(input_path).map_err(|e| format!("Failed to read input file: {e}"))?;
    let song = parse_chordbook(&input)?;
    let typst = song_to_typst(&song);
    fs::write(output_path, typst).map_err(|e| format!("Failed to write output file: {e}"))?;
    Ok(())
}

fn parse_chordbook(input: &str) -> Result<Song, String> {
    let mut lines = input.lines();
    let title = lines.next().ok_or("Missing title")?.trim().to_string();
    let author = lines.next().ok_or("Missing author")?.trim().to_string();

    let mut sections = Vec::new();
    let mut current_section: Option<Section> = None;

    for line in lines {
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
        } else {
            if let Some(section) = &mut current_section {
                if line.split_whitespace().all(|w| is_chord(w)) {
                    let chords = line.split_whitespace().map(|s| s.to_string()).collect();
                    section.lines.push(Line::ChordOnly(chords));
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

fn is_chord(s: &str) -> bool {
    let s = s.trim();
    !s.is_empty() && s.chars().next().unwrap().is_ascii_uppercase()
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
                Line::ChordOnly(chords) => {
                    write!(&mut out, "            (").unwrap();
                    for chord in chords {
                        write!(&mut out, "(chord: {:?}), ", chord).unwrap();
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