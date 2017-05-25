extern crate syntect;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightFile;

use std::io::BufRead;

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please pass in a file to highlight");
        return;
    }

    let theme = &ts.themes["base16-eighties.dark"];
    let highlighter = HighlightFile::new(&args[1], &ss, theme);

    match highlighter {
        Ok(mut highlighter) => {
            let mut line = String::new();
            while highlighter.reader.read_line(&mut line).unwrap() > 0 {
                {
                    let regions: Vec<(Style, &str)> = highlighter.highlight_lines.highlight(&line);
                    print!("{}", as_24_bit_terminal_escaped(&regions[..], false));
                }
                line.clear();
            }
        },
        Err(_) => println!("File not found!")
    }
}
