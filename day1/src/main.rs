use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() {
    let file = BufReader::new(File::open("content/input.txt").expect("Cannot open input.txt"));

    let all_values = input_vec(file);

    let sum: u32 = all_values.iter().sum();
    eprintln!("Sum: {:?}", sum);
}

fn input_vec(file: BufReader<File>) -> Vec<u32> {
    let mut all_values = Vec::new();

    for line in file.lines() {
        if let Ok(line) = line {
            let number_vec: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            match number_vec.len() {
                len if len >= 2 => {
                    let number_vec = (number_vec.first().unwrap(), number_vec.last().unwrap());
                    let format_vec = format!("{:?}{:?}", number_vec.0, number_vec.1);
                    let number_vec = format_vec.parse::<u32>().unwrap();
                    println!("format_vec: {:?}", number_vec);
                    all_values.push(number_vec);
                }
                _ => {
                    if let Some(&value) = number_vec.first() {
                        let format_vec = format!("{:?}{:?}", value, number_vec.first().unwrap());
                        let number_vec = format_vec.parse::<u32>().unwrap();
                        println!("format_vec: {:?}", number_vec);
                        all_values.push(number_vec);
                    }
                }
            }
        }
    }

    all_values
}
