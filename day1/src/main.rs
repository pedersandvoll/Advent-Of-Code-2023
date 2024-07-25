use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() {
    let file = BufReader::new(File::open("content/input.txt").expect("Cannot open input.txt"));

    let all_values = input_vec(file);
    let combined_values = combine_adjacent(all_values);

    let sum: u32 = combined_values.iter().sum();
    eprintln!("Sum: {:?}", sum);
}

fn input_vec(file: BufReader<File>) -> Vec<u32> {
    let mut all_values = Vec::new();

    for line in file.lines() {
        if let Ok(line) = line {
            let mut number_vec: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            match number_vec.len() {
                len if len >= 2 => {
                    let number_vec = (number_vec.first().unwrap(), number_vec.last().unwrap());
                    all_values.push(*number_vec.0);
                    all_values.push(*number_vec.1);
                }
                _ => {
                    if let Some(&value) = number_vec.first() {
                        number_vec.push(value);
                    }
                    all_values.extend(number_vec);
                }
            }
        }
    }

    all_values
}

fn combine_adjacent(values: Vec<u32>) -> Vec<u32> {
    let mut combined_values = Vec::new();
    let len = values.len();

    let mut i = 0;
    while i < len {
        if i + 1 < len {
            let combined = values[i] * 10 + values[i + 1];
            combined_values.push(combined);
        } else {
            combined_values.push(values[i]);
        }
        i += 2;
    }

    combined_values
}
