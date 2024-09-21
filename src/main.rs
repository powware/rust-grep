use loop_unwrap::{unwrap_continue, ToOption};
use regex::Regex;
use std::collections::LinkedList;
use std::env;
use std::error::Error;
use std::io::{stdin, BufRead, BufReader, IsTerminal};
use std::result::Result;

fn line_matches(line: &str, regex: &Regex) -> bool {
    let matches: Vec<_> = regex.find_iter(&line).collect();

    !matches.is_empty()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: LinkedList<String> = env::args().collect();
    if args.len() < 2 {
        println!("pattern required");
        return Ok(());
    }

    args.pop_front();
    let pattern = args.pop_front().expect("args.len() < 2 prevents failure");
    let regex = Regex::new(&pattern).expect("malformed pattern");

    if args.len() > 0 {
        while args.len() > 0 {
            let path = args
                .pop_front()
                .expect("while args.len() > 0 prevents failure");
            println!("searching in {path:?}");

            let file = unwrap_continue!(std::fs::File::open(&path));

            let mut line_number = 1;
            for line in BufReader::new(&file).lines() {
                let line = unwrap_continue!(line);
                if line_matches(&line, &regex) {
                    println!("{line_number:?} {line:?}");
                }
                line_number += 1;
            }

            println!("finished searching in {path:?}");
        }
    } else {
        let stdin = stdin();
        if stdin.is_terminal() {
            let mut line = String::new();
            stdin.read_line(&mut line)?;

            if line_matches(&line, &regex) {
                println!("{line:?}");
            }
        } else {
            let mut line_number = 1;
            loop {
                let mut line = String::new();
                if stdin.read_line(&mut line)? == 0 {
                    break;
                }

                if line.ends_with('\n') {
                    line.pop();
                }
                if line_matches(&line, &regex) {
                    println!("{line_number:?} {line:?}");
                }
                line_number += 1;
            }
        }
    }

    Ok(())
}
