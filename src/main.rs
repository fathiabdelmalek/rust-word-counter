use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<()> {
    println!("Welcome to Words Counter!");
    println!("Enter the path to the text file:");
    let binding = get_input();
    let file_path = binding.trim();
    let count = count_words_in_file(file_path)
        .with_context(|| format!("Failed to read file {}", file_path))?;
    println!("Total number of words: {}", count);
    Ok(())
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn count_words_in_file<P: AsRef<Path>>(file_path: P) -> Result<usize> {
    let file = File::open(file_path).with_context(|| "Unable to open file")?;
    let reader = io::BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line.with_context(|| "Unable to read line")?;
        count += line.split_whitespace().count();
    }
    Ok(count)
}
