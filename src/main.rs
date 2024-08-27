use std::{fs, path::PathBuf};

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_hint = ValueHint::FilePath, required = true)]
    path: PathBuf,
}

fn split(line: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let splitters = vec!['！', '。', '？'];
    let ignore = vec!['「', '」', '◇'];
    let mut sentence = String::new();
    for (_, s) in line.char_indices() {
        if ignore.contains(&s) {
            continue;
        }
        sentence.push(s);
        if splitters.contains(&s) {
            result.push(sentence.trim().to_string().clone());
            sentence.clear();
        }
    }
    if !sentence.is_empty() {
        result.push(sentence);
    }
    result
}

fn main() {
    let args = Args::parse();
    let book_path = args.path;
    if !book_path.is_file() {
        println!("This path isn't a file!");
        return;
    }
    let text = fs::read_to_string(book_path).unwrap_or_else(|_| {
        println!("Can't read the file!");
        std::process::exit(1);
    });
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        // println!("Line {}, {}", line, line.len());
        let sentences = split(line.trim());
        for sentence in sentences {
            // println!("Sentence {}, {}", sentence, sentence.len());
            println!("{}", sentence);
        }
    }
}
