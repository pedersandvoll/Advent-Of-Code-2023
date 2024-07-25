use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() {
    let file =
        BufReader::new(File::open("content/exampleInput.txt").expect("Cannot open input.txt"));

    input_vec(file);
}

fn input_vec(file: BufReader<File>) {
    for line in file.lines() {
        if let Ok(line) = line {
            let number_vec: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            eprintln!("{:?}", number_vec);
        }
    }
}
