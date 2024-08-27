use epub::doc::EpubDoc;
use scraper::{Html, Selector};
use std::path::PathBuf;

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
    if !sentence.trim().is_empty() {
        result.push(sentence.trim().to_string().clone());
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
    let mut book = EpubDoc::new(book_path).unwrap_or_else(|_| {
        println!("Can't read the file!");
        std::process::exit(1);
    });
    book.go_next();
    let mut keys: Vec<String> = Vec::new();
    for key in book.resources.keys() {
        if key.starts_with("p-0") {
            keys.push(key.clone());
        }
    }
    keys.sort();
    for key in keys {
        let text = book
            .get_resource_str(&key)
            .unwrap_or_else(|| {
                println!("Can't parce the text!");
                std::process::exit(1);
            })
            .0;
        let fragment = Html::parse_fragment(&text);
        let selector = Selector::parse("p.calibre3").unwrap();
        for element in fragment.select(&selector) {
            let mut line = element.inner_html();
            let text = element.text().collect::<Vec<_>>();
            if text.is_empty() {
                continue;
            }
            for child in element.child_elements() {
                let in_ruby = child.html();
                let out_ruby = format!(
                    " {}",
                    in_ruby
                        .replace("<ruby>", "")
                        .replace("</ruby>", "")
                        .replace("<rt>", "[")
                        .replace("</rt>", "] ")
                        .trim()
                );
                line = line.replace(&in_ruby, &out_ruby).replace(" ", " ");
            }
            for sentence in split(&line) {
                println!("{}", sentence);
            }
        }
    }
}
