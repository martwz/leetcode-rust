#![allow(unused_must_use)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod binarysearch;
mod leetcode;

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::{env, error::Error, fs};
use walkdir::WalkDir;

fn main() {
    println!("Hello Leetcode!");
    gen_table();
}

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn gen_table() -> Result<()> {
    let re_title = Regex::new(r"// (\d+). (.+), (.+)$").unwrap();
    let re_url = Regex::new(r"// (https?:.*)").unwrap();

    let mut entries: Vec<(i32, String, String, String, String)> = Vec::new();
    for entry in WalkDir::new("./src").follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let f_path = &entry.path().to_string_lossy().to_string();

        if f_path.ends_with(".rs") {
            let input = File::open(f_path)?;
            let buffered = BufReader::new(input);
            let mut entry = (0, "".to_string(), "".to_string(), "".to_string(), "".to_string());
            for line in buffered.lines() {
                let str = line.as_ref().unwrap();
                if re_title.is_match(str) {
                    let m = re_title.captures(str).unwrap();
                    entry.0 = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    entry.1 = m.get(2).unwrap().as_str().to_string();
                    entry.2 = m.get(3).unwrap().as_str().to_string();
                }
                if re_url.is_match(str) {
                    let m = re_url.captures(str).unwrap();
                    entry.3 = m.get(1).unwrap().as_str().to_string();
                    entry.4 = f_path.clone();
                }

                if entry.0 != 0 && !entry.3.is_empty() {
                    entries.push(entry.clone());
                    entry = (0, "".to_string(), "".to_string(), "".to_string(), "".to_string());
                }
            }
        }
    }

    entries.sort();
    println!("## Solutions ({}) ", entries.len());
    println!(
        r#"| No. | Title | Solution | Problem | Difficulty |
    |:---:|:------|:--------:|:-------:|:----------:|"#
    );
    for entry in entries {
        println!(
            "| {} | {} | [Rust](https://github.com/martinxxd/leetcode-rust/tree/master/{}) | [Leetcode]({}) | {} |",
            entry.0, entry.1, entry.4, entry.3, entry.2
        );
    }

    Ok(())
}
